#![no_std]
#![macro_use]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(unpredictable_function_pointer_comparisons)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused)]
#![allow(rustdoc::broken_intra_doc_links)]

extern crate alloc;

#[macro_export]
/// `(($variant as u32) << 29u32) | (($major as u32) << 22u32) | (($minor as u32) << 12u32) | ($patch as u32)`
macro_rules! VK_MAKE_API_VERSION {
    ( $variant:expr, $major:expr, $minor:expr, $patch:expr ) => {
        (($variant as u32) << 29u32)
            | (($major as u32) << 22u32)
            | (($minor as u32) << 12u32)
            | ($patch as u32)
    };
}

#[macro_export]
/// `($version as u32) >> 29u32`
macro_rules! VK_API_VERSION_VARIANT {
    ( $version:expr ) => {
        ($version as u32) >> 29u32
    };
}

#[macro_export]
/// `(($version as u32) >> 22u32) & 0x7Fu32`
macro_rules! VK_API_VERSION_MAJOR {
    ( $version:expr ) => {
        (($version as u32) >> 22u32) & 0x7Fu32
    };
}

#[macro_export]
/// `(($version as u32) >> 12u32) & 0x3FFu32`
macro_rules! VK_API_VERSION_MINOR {
    ( $version:expr ) => {
        (($version as u32) >> 12u32) & 0x3FFu32
    };
}

#[macro_export]
/// `($version as u32) & 0xFFFu32`
macro_rules! VK_API_VERSION_PATCH {
    ( $version:expr ) => {
        ($version as u32) & 0xFFFu32
    };
}

/// `VK_MAKE_API_VERSION!(0, 1, 0, 0)`
pub const VK_API_VERSION_1_0: u32 = VK_MAKE_API_VERSION!(0, 1, 0, 0);
/// `VK_MAKE_API_VERSION!(0, 1, 1, 0)`
pub const VK_API_VERSION_1_1: u32 = VK_MAKE_API_VERSION!(0, 1, 1, 0);
/// `VK_MAKE_API_VERSION!(0, 1, 2, 0)`
pub const VK_API_VERSION_1_2: u32 = VK_MAKE_API_VERSION!(0, 1, 2, 0);
/// `VK_MAKE_API_VERSION!(0, 1, 3, 0)`
pub const VK_API_VERSION_1_3: u32 = VK_MAKE_API_VERSION!(0, 1, 3, 0);
/// `VK_MAKE_API_VERSION!(0, 1, 4, 0)`
pub const VK_API_VERSION_1_4: u32 = VK_MAKE_API_VERSION!(0, 1, 4, 0);

/// VkSampleMask
pub type VkSampleMask = u32;
/// VkBool32
pub type VkBool32 = u32;
/// VkFlags
pub type VkFlags = u32;
/// VkFlags64
pub type VkFlags64 = u64;
/// VkDeviceSize
pub type VkDeviceSize = u64;
/// VkDeviceAddress
pub type VkDeviceAddress = u64;

impl VkBaseInStructure {
    /// Downcast the structure to the specified type if `sType` matches, otherwise return `None`.
    pub unsafe fn downcast<T: Chainable>(&self) -> Option<&T> {
        if self.sType == T::TYPE {
            Some(unsafe { core::mem::transmute(self) })
        } else {
            None
        }
    }
    /// Downcast the structure to the specified type if `sType` matches, otherwise return `None`.
    pub unsafe fn downcast_mut<T: Chainable>(&mut self) -> Option<&mut T> {
        if self.sType == T::TYPE {
            Some(unsafe { core::mem::transmute(self) })
        } else {
            None
        }
    }
}

impl VkBaseOutStructure {
    /// Downcast the structure to the specified type if `sType` matches, otherwise return `None`.
    pub unsafe fn downcast<T: Chainable>(&self) -> Option<&T> {
        if self.sType == T::TYPE {
            Some(unsafe { core::mem::transmute(self) })
        } else {
            None
        }
    }
    /// Downcast the structure to the specified type if `sType` matches, otherwise return `None`.
    pub unsafe fn downcast_mut<T: Chainable>(&mut self) -> Option<&mut T> {
        if self.sType == T::TYPE {
            Some(unsafe { core::mem::transmute(self) })
        } else {
            None
        }
    }
}

/// Marker trait for struct with `sType` and `pNext` field
pub trait Chainable: Sized {
    /// The value of `sType` field
    const TYPE: crate::VkStructureType;
    /// The const reference of `pNext` field
    fn p_next(&self) -> &*const core::ffi::c_void;
    /// The mutable reference of `pNext` field
    fn p_next_mut(&mut self) -> &mut *mut core::ffi::c_void;

    /// Push the next struct to the chain
    unsafe fn push_next<E: Extend<Self>>(&mut self, ex: &mut E) -> &mut Self {
        core::mem::replace(ex.p_next_mut(), *self.p_next_mut());
        *self.p_next_mut() = (ex as *mut E).cast();
        self
    }

    /// Get the next struct in the chain
    unsafe fn next(&self) -> Option<&VkBaseInStructure> {
        let p_next = *self.p_next();
        if p_next.is_null() {
            None
        } else {
            Some(unsafe { &*(p_next.cast::<VkBaseInStructure>()) })
        }
    }

    /// Get the next struct in the chain
    unsafe fn next_mut(&mut self) -> Option<&mut VkBaseOutStructure> {
        let p_next = *self.p_next_mut();
        if p_next.is_null() {
            None
        } else {
            Some(unsafe { &mut *(p_next.cast::<VkBaseOutStructure>()) })
        }
    }

    /// Upcast the struct to `VkBaseInStructure`
    unsafe fn upcast(&self) -> &VkBaseInStructure {
        unsafe { core::mem::transmute(self) }
    }

    /// Upcast the struct to `VkBaseOutStructure`
    unsafe fn upcast_mut(&mut self) -> &mut VkBaseOutStructure {
        unsafe { core::mem::transmute(self) }
    }
}

/// Type safe extend chain
pub trait Extend<T: Chainable>: Chainable {}

/// Marker trait for vulkan handles
pub trait Handle {
    /// Handle type
    const TYPE: crate::VkObjectType;

    fn null() -> Self;
    fn is_null(&self) -> bool;
}

/// Marker trait for non parent handles
pub trait RootHandle: Handle {}

/// Marker trait for sub handles
pub trait SubHandle: Handle {
    /// Parent handle type
    type Parent: Handle;
}

macro_rules! from_into_transparent {
    ( $t:ty : $u:ty ) => {
        impl From<$t> for $u {
            fn from(value: $t) -> Self {
                value.0 as _
            }
        }
        impl From<$u> for $t {
            fn from(value: $u) -> Self {
                Self(value as _)
            }
        }
    };
}

macro_rules! flags {
    ( $t:ty ) => {
        impl $t {
            /// `0`
            pub const fn empty() -> Self {
                Self(0)
            }
            /// return is `0`
            pub const fn is_empty(self) -> bool {
                self.0 == 0
            }
            /// return is all bits in `flags` are set
            pub const fn has_flags(self, flags: Self) -> bool {
                (self.0 & flags.0) == flags.0
            }
            /// return is any bits in `flags` are set
            pub const fn has_any_flags(self, flags: Self) -> bool {
                (self.0 & flags.0) != 0
            }
            /// return is only bits in `flags` are set
            pub const fn has_flags_only(self, flags: Self) -> bool {
                (self.0 & !flags.0) == 0
            }
        }
        impl core::ops::Not for $t {
            type Output = Self;

            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }
        impl core::ops::BitOr for $t {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }
        impl core::ops::BitAnd for $t {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }
        impl core::ops::BitXor for $t {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }
        impl core::ops::BitAndAssign for $t {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }
        impl core::ops::BitOrAssign for $t {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }
        impl core::ops::BitXorAssign for $t {
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0;
            }
        }
    };
}

macro_rules! impl_extends {
    { $this:ty : $($ex:ty),+ } => {
        $(
            impl Extend<$ex> for $this {}
        )+
    };
}

macro_rules! impl_handle {
    ( $t:ty : $objtyp:expr; $det:ty $(= $parent:ty)? ) => {
        impl $t {
            pub const fn null() -> Self {
                Self(0 as _)
            }
            pub fn is_null(&self) -> bool {
                self == &Self::null()
            }
        }
        unsafe impl Send for $t {}
        unsafe impl Sync for $t {}
        impl Handle for $t {
            const TYPE: crate::VkObjectType = $objtyp;
            fn null() -> Self {
                Self::null()
            }
            fn is_null(&self) -> bool {
                self.is_null()
            }
        }
        impl $det for $t {
            $(type Parent = $parent;)?
        }
        impl core::fmt::Pointer for $t
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{:x}", self.0 as u64)
            }
        }
    };
}

macro_rules! impl_zeroed {
    ( $t:ty ) => {
        impl $t {
            pub const ZEROED: Self = unsafe { MaybeUninit::<Self>::zeroed().assume_init() };
        }
    };
}

macro_rules! impl_default_zeroed {
    ( $t:ty ) => {
        impl_zeroed!($t);
        impl Default for $t {
            fn default() -> Self {
                Self::ZEROED
            }
        }
    };
}

macro_rules! impl_default_zeroed_with_s_type {
    ( $t:ty : $s_type:expr ) => {
        impl_zeroed!($t);
        impl Chainable for $t {
            const TYPE: crate::VkStructureType = $s_type;
            fn p_next(&self) -> &*const core::ffi::c_void {
                unsafe { core::mem::transmute(&self.pNext) }
            }
            fn p_next_mut(&mut self) -> &mut *mut core::ffi::c_void {
                unsafe { core::mem::transmute(&mut self.pNext) }
            }
        }
        impl Default for $t {
            fn default() -> Self {
                Self {
                    sType: $s_type,
                    ..Self::ZEROED
                }
            }
        }
    };
}

macro_rules! impl_debug_for_union {
    ( $t:ty ) => {
        impl core::fmt::Debug for $t {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(stringify!($t)).finish_non_exhaustive()
            }
        }
    };
}

macro_rules! impl_enum_display {
    ( $t:ty { $($item:ident),* } ) => {
        impl core::fmt::Debug for $t {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", self)
            }
        }
        impl core::fmt::Display for $t {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match *self {
                    $(
                        $item => write!(f, "{}", stringify!($item)),
                    )*
                    _ => write!(f, "{}", self.0),
                }
            }
        }
    };
}

macro_rules! impl_flags_display {
    ( $t:ident { $($item:ident),* } ) => {
        impl core::fmt::Debug for $t {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if self.is_empty() {
                    return write!(f, "{}(0)", stringify!($t));
                }
                let mut first = true;
                $(
                    if self.has_flags($item)
                    {
                        if first { first = false; } else { write!(f, " | ")?; }
                        write!(f, "{}", stringify!($item))?;
                    }
                )*
                const ALL_FLAGS: $t = const { $t ( 0 $(| $item.0)* ) };
                if ALL_FLAGS.is_empty() {
                    return write!(f, "0");
                }
                let remaining = *self & !ALL_FLAGS;
                if !remaining.is_empty() {
                    if !first {
                        write!(f, " | ")?;
                    }
                    write!(f, "{}(0x{:#x})", stringify!($t), remaining.0)?;
                }
                Ok(())
            }
        }
        impl core::fmt::Display for $t {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if self.is_empty() {
                    return write!(f, "0");
                }
                let mut first = true;
                $(
                    if self.has_flags($item)
                    {
                        if first { first = false; } else { write!(f, " | ")?; }
                        write!(f, "{}", stringify!($item))?;
                    }
                )*
                const ALL_FLAGS: $t = const { $t ( 0 $(| $item.0)* ) };
                if ALL_FLAGS.is_empty() {
                    return write!(f, "0");
                }
                let remaining = *self & !ALL_FLAGS;
                if !remaining.is_empty() {
                    if !first {
                        write!(f, " | ")?;
                    }
                    write!(f, "0x{:#x}", remaining.0)?;
                }
                Ok(())
            }
        }
    };
}

#[macro_export]
/// Create Cstr from literal UTF-8 string with a trailing null terminator.
///
/// ```
/// # use core::ffi::CStr;
/// # use covk_sys::c;
/// let s = c!("Hello, world!");
/// assert_eq!(s, CStr::from_bytes_with_nul(b"Hello, world!\0").unwrap());
/// ```
macro_rules! c {
    ( $s:literal ) => {
        unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(concat!($s, "\0").as_bytes()) }
    };
}

/// ffi types
pub mod ffi;
/// Vulkan loader
pub mod loader;

mod generated;
use core::{mem::MaybeUninit, ptr::NonNull};

pub use generated::*;

/// Vulkan command vtables
pub mod vtbl {
    use crate::{loader::*, *};
    use core::ffi::{CStr, c_char};
    pub use vtbl_gen::*;

    /// Command vtable
    pub trait CommandVTable {
        /// Command function pointers
        type Commands;
        // unsafe fn load(entry: &Entry) -> Result<Self::Commands, LoadingError>;
    }

    /// Global commands
    #[derive(Debug)]
    pub struct GlobalCommands {
        pub vkEnumerateInstanceVersion: FN_vkEnumerateInstanceVersion,
        pub vkEnumerateInstanceExtensionProperties: FN_vkEnumerateInstanceExtensionProperties,
        pub vkEnumerateInstanceLayerProperties: FN_vkEnumerateInstanceLayerProperties,
        pub vkCreateInstance: FN_vkCreateInstance,
    }

    impl GlobalCommands {
        /// Load global commands from the entry point
        pub unsafe fn new(entry: &Entry) -> Result<Self, LoadingError> {
            unsafe {
                Self::load(|name| entry.GetInstanceProcAddr(VkInstance::null(), name.as_ptr()))
            }
        }
        pub unsafe fn load(
            mut get: impl FnMut(&CStr) -> Option<ProcAddr>,
        ) -> Result<Self, LoadingError> {
            unsafe {
                Ok(Self {
                    vkEnumerateInstanceVersion: get(c!("vkEnumerateInstanceVersion"))
                        .ok_or_else(|| {
                            LoadingError::MissingGlobalCommand("vkEnumerateInstanceVersion")
                        })?
                        .cast(),
                    vkEnumerateInstanceExtensionProperties: get(c!(
                        "vkEnumerateInstanceExtensionProperties"
                    ))
                    .ok_or_else(|| {
                        LoadingError::MissingGlobalCommand("vkEnumerateInstanceExtensionProperties")
                    })?
                    .cast(),
                    vkEnumerateInstanceLayerProperties: get(c!(
                        "vkEnumerateInstanceLayerProperties"
                    ))
                    .ok_or_else(|| {
                        LoadingError::MissingGlobalCommand("vkEnumerateInstanceLayerProperties")
                    })?
                    .cast(),
                    vkCreateInstance: get(c!("vkCreateInstance"))
                        .ok_or_else(|| LoadingError::MissingGlobalCommand("vkCreateInstance"))?
                        .cast(),
                })
            }
        }
    }

    impl GlobalCommands {
        /// Query instance-level version before instance creation
        ///
        /// `vkEnumerateInstanceVersion`
        pub unsafe fn EnumerateInstanceVersion(&self, pApiVersion: *mut u32) -> VkResult {
            unsafe { (self.vkEnumerateInstanceVersion)(pApiVersion) }
        }
        /// Returns up to requested number of global extension properties
        ///
        /// `vkEnumerateInstanceExtensionProperties`
        pub unsafe fn EnumerateInstanceExtensionProperties(
            &self,
            pLayerName: *const c_char,
            pPropertyCount: *mut u32,
            pProperties: *mut VkExtensionProperties,
        ) -> VkResult {
            unsafe {
                (self.vkEnumerateInstanceExtensionProperties)(
                    pLayerName,
                    pPropertyCount,
                    pProperties,
                )
            }
        }
        /// Returns up to requested number of global layer properties
        ///
        /// `vkEnumerateInstanceLayerProperties`
        pub unsafe fn EnumerateInstanceLayerProperties(
            &self,
            pPropertyCount: *mut u32,
            pProperties: *mut VkLayerProperties,
        ) -> VkResult {
            unsafe { (self.vkEnumerateInstanceLayerProperties)(pPropertyCount, pProperties) }
        }
        /// Create a new Vulkan instance
        ///
        /// `vkCreateInstance`
        pub unsafe fn CreateInstance(
            &self,
            pCreateInfo: *const VkInstanceCreateInfo,
            pAllocator: *const VkAllocationCallbacks,
            pInstance: *mut VkInstance,
        ) -> VkResult {
            unsafe { (self.vkCreateInstance)(pCreateInfo, pAllocator, pInstance) }
        }
    }

    impl crate::vtbl::InstanceCommands_1_0 {
        /// `vkGetInstanceProcAddr`
        pub unsafe fn GetInstanceProcAddr(&self, instance: VkInstance, pName: &CStr) -> Option<ProcAddr> {
            unsafe {
                (self
                    .vkGetInstanceProcAddr
                    .expect("Unable to load GetInstanceProcAddr"))(
                    instance, pName.as_ptr()
                )
                    .map(ProcAddr)
            }
        }
        /// `vkGetDeviceProcAddr`
        pub unsafe fn GetDeviceProcAddr(&self, device: VkDevice, pName: &CStr) -> Option<ProcAddr> {
            unsafe {
                (self
                    .vkGetDeviceProcAddr
                    .expect("Unable to load GetDeviceProcAddr"))(
                    device, pName.as_ptr()
                )
                .map(ProcAddr)
            }
        }
    }
}

pub type StdVideoAV1Level = u32;
pub type StdVideoAV1Profile = u32;
pub type StdVideoH264LevelIdc = u32;
pub type StdVideoH265LevelIdc = u32;
pub type StdVideoH264ProfileIdc = u32;
pub type StdVideoH265ProfileIdc = u32;
pub type StdVideoVP9Level = u32;
pub type StdVideoVP9Profile = u32;
pub type StdVideoDecodeAV1ReferenceInfo = core::ffi::c_void;
pub type StdVideoAV1SequenceHeader = core::ffi::c_void;
pub type StdVideoDecodeAV1PictureInfo = core::ffi::c_void;
pub type StdVideoDecodeH264ReferenceInfo = core::ffi::c_void;
pub type StdVideoH264SequenceParameterSet = core::ffi::c_void;
pub type StdVideoH264PictureParameterSet = core::ffi::c_void;
pub type StdVideoDecodeH264PictureInfo = core::ffi::c_void;
pub type StdVideoDecodeH265ReferenceInfo = core::ffi::c_void;
pub type StdVideoH265VideoParameterSet = core::ffi::c_void;
pub type StdVideoH265SequenceParameterSet = core::ffi::c_void;
pub type StdVideoH265PictureParameterSet = core::ffi::c_void;
pub type StdVideoDecodeH265PictureInfo = core::ffi::c_void;
pub type StdVideoDecodeVP9PictureInfo = core::ffi::c_void;
pub type StdVideoEncodeAV1ReferenceInfo = core::ffi::c_void;
pub type StdVideoEncodeAV1PictureInfo = core::ffi::c_void;
pub type StdVideoEncodeAV1DecoderModelInfo = core::ffi::c_void;
pub type StdVideoEncodeAV1OperatingPointInfo = core::ffi::c_void;
pub type StdVideoEncodeH264ReferenceInfo = core::ffi::c_void;
pub type StdVideoEncodeH264SliceHeader = core::ffi::c_void;
pub type StdVideoEncodeH264PictureInfo = core::ffi::c_void;
pub type StdVideoEncodeH265ReferenceInfo = core::ffi::c_void;
pub type StdVideoEncodeH265SliceSegmentHeader = core::ffi::c_void;
pub type StdVideoEncodeH265PictureInfo = core::ffi::c_void;

#[cfg(test)]
mod tests {
    use core::ffi::CStr;

    #[test]
    fn test_c_macro() {
        let s = c!("Hello, world!");
        assert_eq!(s, CStr::from_bytes_with_nul(b"Hello, world!\0").unwrap());
    }
}
