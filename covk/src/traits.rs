use super::*;
use core::ffi::{CStr, c_char, c_void};
use core::hash::Hash;

/// Convert between sys types and vk types
pub trait Vk<'a> {
    type Target;

    /// Convert to vk types
    fn vk(self) -> Self::Target;
}

/// Convert between sys types and vk types
pub trait Sys<'a> {
    type Target;
    /// Convert to sys types
    fn sys(self) -> Self::Target;
}

/// Marker trait for struct with `sType` and `pNext` field
pub trait Chainable: Sized + StructType {
    /// The const reference of `pNext` field
    fn p_next(&self) -> &*const c_void;
    /// The mutable reference of `pNext` field
    fn p_next_mut(&mut self) -> &mut *mut c_void;

    /// Push the next struct to the chain
    unsafe fn push_next<E: Extend<Self>>(&mut self, ex: &mut E) -> &mut Self {
        core::mem::replace(ex.p_next_mut(), *self.p_next_mut());
        *self.p_next_mut() = (ex as *mut E).cast();
        self
    }

    /// Push the next struct to the chain
    unsafe fn with_next<E: Extend<Self>>(mut self, ex: &mut E) -> Self {
        core::mem::replace(ex.p_next_mut(), *self.p_next_mut());
        *self.p_next_mut() = (ex as *mut E).cast();
        self
    }

    /// Get the next struct in the chain
    unsafe fn next(&'_ self) -> Option<&'_ vk::BaseInStructure<'_>> {
        let p_next = *self.p_next();
        if p_next.is_null() {
            None
        } else {
            Some(unsafe { &*(p_next.cast::<vk::BaseInStructure>()) })
        }
    }

    /// Get the next struct in the chain
    unsafe fn next_mut(&'_ mut self) -> Option<&'_ mut vk::BaseOutStructure<'_>> {
        let p_next = *self.p_next_mut();
        if p_next.is_null() {
            None
        } else {
            Some(unsafe { &mut *(p_next.cast::<vk::BaseOutStructure>()) })
        }
    }

    /// Upcast the struct to `VkBaseInStructure`
    unsafe fn upcast(&'_ self) -> &'_ vk::BaseInStructure<'_> {
        unsafe { core::mem::transmute(self) }
    }

    /// Upcast the struct to `VkBaseOutStructure`
    unsafe fn upcast_mut(&'_ mut self) -> &'_ mut vk::BaseOutStructure<'_> {
        unsafe { core::mem::transmute(self) }
    }
}

/// Type safe extend chain
pub trait Extend<T: Chainable>: Chainable {}

impl<'a> vk::BaseInStructure<'a> {
    /// Downcast the structure to the specified type if `sType` matches, otherwise return `None`.
    pub unsafe fn downcast<T: Chainable>(&self) -> Option<&T> {
        if self.s_type == T::TYPE {
            Some(unsafe { core::mem::transmute(self) })
        } else {
            None
        }
    }
    /// Downcast the structure to the specified type if `sType` matches, otherwise return `None`.
    pub unsafe fn downcast_mut<T: Chainable>(&mut self) -> Option<&mut T> {
        if self.s_type == T::TYPE {
            Some(unsafe { core::mem::transmute(self) })
        } else {
            None
        }
    }
}

impl<'a> vk::BaseOutStructure<'a> {
    /// Downcast the structure to the specified type if `sType` matches, otherwise return `None`.
    pub unsafe fn downcast<T: Chainable>(&self) -> Option<&T> {
        if self.s_type == T::TYPE {
            Some(unsafe { core::mem::transmute(self) })
        } else {
            None
        }
    }
    /// Downcast the structure to the specified type if `sType` matches, otherwise return `None`.
    pub unsafe fn downcast_mut<T: Chainable>(&mut self) -> Option<&mut T> {
        if self.s_type == T::TYPE {
            Some(unsafe { core::mem::transmute(self) })
        } else {
            None
        }
    }
}

/// Marker trait for vulkan handles
pub trait RawHandle:
    ObjectType + Copy + Eq + Ord + Hash + core::fmt::Debug + core::fmt::Pointer
{
}

/// Marker trait for non parent handles
pub trait RawRootHandle: RawHandle {}

/// Marker trait for sub handles
pub trait RawSubHandle: RawHandle {
    /// Parent handle type
    type Parent: RawHandle;
}

/// Mark the struct type
pub trait StructType {
    // The struct type
    const TYPE: vk::StructureType;
}

/// Mark the object type
pub trait ObjectType {
    // The object type
    const TYPE: vk::ObjectType;
}

/// Marker trait for proc addr loader
pub trait ProcAddrLoader {
    /// Load the proc addr
    unsafe fn load(&mut self, name: &CStr) -> Option<ProcAddr>;
}

impl<F: FnMut(&CStr) -> Option<ProcAddr>> ProcAddrLoader for F {
    unsafe fn load(&mut self, name: &CStr) -> Option<ProcAddr> {
        self(name)
    }
}

/// Utils trait for ad hoc
pub trait Abi<T> {
    fn abi(self) -> T;
}

impl<T> Abi<T> for T {
    fn abi(self) -> T {
        self
    }
}

impl<T> Abi<*const T> for &T {
    fn abi(self) -> *const T {
        self
    }
}

impl<T> Abi<*mut T> for &mut T {
    fn abi(self) -> *mut T {
        self
    }
}

impl<T> Abi<*const T> for &[T] {
    fn abi(self) -> *const T {
        self.as_ptr()
    }
}

impl<T> Abi<*mut T> for &mut [T] {
    fn abi(self) -> *mut T {
        self.as_mut_ptr()
    }
}

impl<T> Abi<*const T> for Option<&T> {
    fn abi(self) -> *const T {
        self.map(|a| a as *const T).unwrap_or_default()
    }
}

impl<T> Abi<*mut T> for Option<&mut T> {
    fn abi(self) -> *mut T {
        self.map(|a| a as *mut T).unwrap_or_default()
    }
}

impl<T> Abi<*const T> for Option<&[T]> {
    fn abi(self) -> *const T {
        self.map(|a| a.as_ptr()).unwrap_or_default()
    }
}

impl<T> Abi<*mut T> for Option<&mut [T]> {
    fn abi(self) -> *mut T {
        self.map(|a| a.as_mut_ptr()).unwrap_or_default()
    }
}

impl Abi<vk::Bool> for bool {
    fn abi(self) -> vk::Bool {
        self.into()
    }
}

impl Abi<sys::VkBool32> for bool {
    fn abi(self) -> sys::VkBool32 {
        if self { sys::VK_TRUE } else { sys::VK_FALSE }
    }
}

impl<'a> Abi<*const c_char> for &'a CStr {
    fn abi(self) -> *const c_char {
        self.as_ptr()
    }
}

impl<'a> Abi<*const c_char> for Option<&'a CStr> {
    fn abi(self) -> *const c_char {
        self.map(|a| a.as_ptr()).unwrap_or_default()
    }
}

impl Abi<[u8; sys::VK_UUID_SIZE as usize]> for Uuid {
    fn abi(self) -> [u8; sys::VK_UUID_SIZE as usize] {
        unsafe { core::mem::transmute(self) }
    }
}
