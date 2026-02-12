use crate::{vtbl::GlobalCommands, *};
use alloc::sync::Arc;
use core::{
    ffi::{CStr, c_char},
    ops::Deref,
};
#[cfg(feature = "loaded")]
use libloading::{AsFilename, Library};
use thiserror::Error;

/// Utils type for proc addr
#[derive(Debug, Clone, Copy)]
pub struct ProcAddr(pub FN_vkVoidFunction);

impl ProcAddr {
    /// unsafe cast the proc addr to function pointer
    pub unsafe fn cast<T: Copy>(self) -> T {
        const { assert!(core::mem::size_of::<T>() == core::mem::size_of::<usize>()) }
        let ptr = ((&self.0) as *const FN_vkVoidFunction).cast::<T>();
        unsafe { *ptr }
    }
}

/// `GetInstanceProcAddr`
#[derive(Debug, Clone, Copy)]
pub struct GetInstanceProcAddr(pub FN_vkGetInstanceProcAddr);

impl GetInstanceProcAddr {
    /// `get`
    pub fn get(&self, instance: VkInstance, name: *const c_char) -> Option<ProcAddr> {
        unsafe { (self.0)(instance, name).map(ProcAddr) }
    }
}

/// `GetDeviceProcAddr`
#[derive(Debug, Clone, Copy)]
pub struct GetDeviceProcAddr(pub FN_vkGetDeviceProcAddr);

impl GetDeviceProcAddr {
    /// `get`
    pub fn get(&self, device: VkDevice, name: *const c_char) -> Option<ProcAddr> {
        unsafe { (self.0)(device, name).map(ProcAddr) }
    }
}

/// Errors that can occur while loading the Vulkan library or its symbols.
#[derive(Debug, Error)]
pub enum LoadingError {
    /// The library could not be loaded.
    #[error(transparent)]
    #[cfg(feature = "loaded")]
    #[cfg_attr(docsrs, doc(cfg(feature = "loaded")))]
    LoadError(libloading::Error),
    /// The library was loaded, but a symbol could not be loaded.
    #[error(transparent)]
    #[cfg(feature = "loaded")]
    #[cfg_attr(docsrs, doc(cfg(feature = "loaded")))]
    SymbolLoadError(libloading::Error),
    #[error("The command `{0}` is not available in the Vulkan library")]
    MissingGlobalCommand(&'static str),
}

/// Vulkan entry point
#[derive(Debug)]
pub struct Entry {
    pub vkGetInstanceProcAddr: FN_vkGetInstanceProcAddr,
    _lib: Option<Library>,
}

impl Entry {
    #[cfg(any(feature = "loaded", feature = "linked"))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "loaded", feature = "linked"))))]
    /// Create a new entry point.
    pub unsafe fn new() -> Result<Arc<Self>, LoadingError> {
        #[cfg(feature = "linked")]
        {
            return Ok(Self::linked());
        }
        #[cfg(feature = "loaded")]
        {
            return unsafe { Self::load() };
        }
        unreachable!()
    }

    /// Create a new entry point from a vkGetInstanceProcAddr function pointer.
    pub unsafe fn from_vkGetInstanceProcAddr(
        vkGetInstanceProcAddr: FN_vkGetInstanceProcAddr,
    ) -> Arc<Self> {
        Arc::new(Self {
            vkGetInstanceProcAddr,
            _lib: None,
        })
    }
}

impl Entry {
    /// Load entry points linked at compile time
    #[cfg(feature = "linked")]
    #[cfg_attr(docsrs, doc(cfg(feature = "linked")))]
    pub const fn linked() -> Arc<Self> {
        Self {
            vkGetInstanceProcAddr: vkGetInstanceProcAddr,
            _lib: None,
        }
    }
}

#[cfg(feature = "linked")]
extern "C" {
    fn vkGetInstanceProcAddr(instance: VkInstance, pName: *const c_char) -> PFN_vkVoidFunction;
}

#[cfg(feature = "loaded")]
#[cfg_attr(docsrs, doc(cfg(feature = "loaded")))]
impl Entry {
    #[cfg(windows)]
    /// The default vulkan libary path.
    pub const DEFAULT_LIB_PATH: &'static str = "vulkan-1.dll";
    #[cfg(all(
        unix,
        not(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "android",
            target_os = "fuchsia"
        ))
    ))]
    /// The default vulkan libary path.
    pub const DEFAULT_LIB_PATH: &'static str = "libvulkan.so.1";
    #[cfg(any(target_os = "android", target_os = "fuchsia"))]
    /// The default vulkan libary path.
    pub const DEFAULT_LIB_PATH: &'static str = "libvulkan.so";
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    /// The default vulkan libary path.
    pub const DEFAULT_LIB_PATH: &'static str = "libvulkan.dylib";

    /// Load the vulkan library from the default path.
    pub unsafe fn load() -> Result<Arc<Self>, LoadingError> {
        unsafe { Self::load_from(Self::DEFAULT_LIB_PATH) }
    }

    /// Load the vulkan library from the specified path.
    pub unsafe fn load_from(path: impl AsFilename) -> Result<Arc<Self>, LoadingError> {
        let lib = unsafe { Library::new(path) }.map_err(LoadingError::LoadError)?;
        let vkGetInstanceProcAddr = unsafe {
            lib.get(b"vkGetInstanceProcAddr\0")
                .map(|a| *a)
                .map_err(LoadingError::SymbolLoadError)?
        };
        Ok(Arc::new(Self {
            vkGetInstanceProcAddr,
            _lib: Some(lib),
        }))
    }
}

impl Entry {
    /// Return a function pointer for a command
    pub unsafe fn GetInstanceProcAddr(
        &self,
        instance: VkInstance,
        name: *const c_char,
    ) -> Option<ProcAddr> {
        unsafe { (self.vkGetInstanceProcAddr)(instance, name).map(ProcAddr) }
    }
}

/// The vulkan library
#[derive(Debug)]
pub struct Vulkan {
    pub entry: Arc<Entry>,
    pub global_commands: GlobalCommands,
}

impl Vulkan {
    /// Create a new Vulkan library from the default entry point.
    #[cfg(any(feature = "loaded", feature = "linked"))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "loaded", feature = "linked"))))]
    pub unsafe fn new() -> Result<Arc<Self>, LoadingError> {
        unsafe { Self::from_entry(Entry::new()?) }
    }

    /// Create a new Vulkan library from the given entry point.
    pub unsafe fn from_entry(entry: Arc<Entry>) -> Result<Arc<Self>, LoadingError> {
        Ok(Arc::new(Self {
            global_commands: unsafe { GlobalCommands::new(&entry)? },
            entry,
        }))
    }
}

impl Vulkan {
    /// Return a function pointer for a command
    pub unsafe fn GetInstanceProcAddr(
        &self,
        instance: VkInstance,
        name: *const c_char,
    ) -> Option<ProcAddr> {
        unsafe { self.entry.GetInstanceProcAddr(instance, name) }
    }
}

impl Deref for Vulkan {
    type Target = GlobalCommands;

    fn deref(&self) -> &Self::Target {
        &self.global_commands
    }
}
