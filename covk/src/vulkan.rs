use crate::*;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::ffi::CStr;
use sys::loader;

/// The vulkan library
#[derive(Debug, Clone)]
pub struct Vulkan {
    sys: Arc<loader::Vulkan>,
}

impl Vulkan {
    /// Load the vulkan library and global commands
    pub unsafe fn new() -> core::result::Result<Self, loader::LoadingError> {
        Ok(Self {
            sys: unsafe { loader::Vulkan::new()? },
        })
    }
    /// Create from [loader::Vulkan]
    pub fn from_sys(sys: Arc<loader::Vulkan>) -> core::result::Result<Self, loader::LoadingError> {
        Ok(Self { sys })
    }

    /// Create a new vulkan library from a vkGetInstanceProcAddr function pointer.
    pub unsafe fn from_vkGetInstanceProcAddr(
        vkGetInstanceProcAddr: sys::FN_vkGetInstanceProcAddr,
    ) -> core::result::Result<Self, loader::LoadingError> {
        Ok(Self {
            sys: unsafe {
                loader::Vulkan::from_entry(loader::Entry::from_vkGetInstanceProcAddr(
                    vkGetInstanceProcAddr,
                ))?
            },
        })
    }

    // Get sys vulkan
    pub fn sys(&self) -> &Arc<loader::Vulkan> {
        &self.sys
    }
}

#[cfg(feature = "linked")]
#[cfg_attr(docsrs, doc(cfg(feature = "linked")))]
impl Vulkan {
    /// Load vulkan library linked at compile time
    pub fn linked() -> core::result::Result<Self, loader::LoadingError> {
        Ok(Self {
            sys: unsafe { loader::Vulkan::from_entry(loader::Entry::linked())? },
        })
    }
}

#[cfg(feature = "loaded")]
#[cfg_attr(docsrs, doc(cfg(feature = "loaded")))]
impl Vulkan {
    /// Load the vulkan library from the default path.
    pub fn load() -> core::result::Result<Self, loader::LoadingError> {
        Ok(Self {
            sys: unsafe { loader::Vulkan::from_entry(loader::Entry::load()?)? },
        })
    }
    /// Load the vulkan library from the specified path.
    pub fn load_from(
        path: impl libloading::AsFilename,
    ) -> core::result::Result<Self, loader::LoadingError> {
        Ok(Self {
            sys: unsafe { loader::Vulkan::from_entry(loader::Entry::load_from(path)?)? },
        })
    }
}

impl Vulkan {
    /// Return a function pointer for a command
    pub unsafe fn get_instance_proc_addr<'a>(
        &self,
        inst: Option<vk::Instance>,
        name: &CStr,
    ) -> Option<ProcAddr> {
        unsafe { self.sys.GetInstanceProcAddr(inst.sys(), name.as_ptr()) }
    }
    /// Query instance-level version before instance creation
    ///
    /// `vkEnumerateInstanceVersion`
    pub unsafe fn enumerate_instance_version(&self) -> Result<u32> {
        unsafe {
            let mut version = 0;
            self.sys
                .EnumerateInstanceVersion(&mut version)
                .vk()
                .result(|| Some(version))
        }
    }
    /// Returns up to requested number of global extension properties
    ///
    /// `vkEnumerateInstanceExtensionProperties`
    pub unsafe fn enumerate_instance_extension_properties(
        &self,
        layer_name: Option<&CStr>,
        properties: Option<&mut Vec<vk::ExtensionProperties>>,
    ) -> Result<(u32, vk::Result)> {
        unsafe {
            let p_layer_name = layer_name.map(|a| a.as_ptr()).unwrap_or_default();
            let mut len = 0;
            self.sys
                .EnumerateInstanceExtensionProperties(p_layer_name, &mut len, core::ptr::null_mut())
                .vk()
                .result_multi(|| Some(()))?;
            if let Some(properties) = properties {
                properties.reserve(len as usize);
                let start = properties.as_mut_ptr().add(properties.len()).cast();
                let r = self
                    .sys
                    .EnumerateInstanceExtensionProperties(p_layer_name, &mut len, start)
                    .vk()
                    .result_multi(|| Some(len))?;
                properties.set_len(properties.len() + r.0 as usize);
                Ok(r)
            } else {
                Ok((len, vk::Result::Success))
            }
        }
    }
    /// Returns up to requested number of global layer properties
    ///
    /// `vkEnumerateInstanceLayerProperties`
    pub unsafe fn enumerate_instance_layer_properties(
        &self,
        properties: Option<&mut Vec<vk::LayerProperties>>,
    ) -> Result<(u32, vk::Result)> {
        unsafe {
            let mut len = 0;
            self.sys
                .EnumerateInstanceLayerProperties(&mut len, core::ptr::null_mut())
                .vk()
                .result_multi(|| Some(()))?;
            if let Some(properties) = properties {
                properties.reserve(len as usize);
                let start = properties.as_mut_ptr().add(properties.len()).cast();
                let r = self
                    .sys
                    .EnumerateInstanceLayerProperties(&mut len, start)
                    .vk()
                    .result_multi(|| Some(len))?;
                properties.set_len(properties.len() + r.0 as usize);
                Ok(r)
            } else {
                Ok((len, vk::Result::Success))
            }
        }
    }
    /// Create a new Vulkan instance
    ///
    /// `vkCreateInstance`
    pub unsafe fn create_instance(
        &'_ self,
        create_info: &vk::InstanceCreateInfo,
    ) -> Result<vk::Instance> {
        unsafe {
            let mut instance = sys::VkInstance::null();
            self.sys
                .CreateInstance(create_info.abi(), Default::default(), &mut instance)
                .vk()
                .result(|| instance.vk())
        }
    }
}
