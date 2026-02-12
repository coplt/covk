pub use crate::sys;
pub use crate::sys::ffi;
pub use generated::enums::*;
pub use generated::fn_ptrs::*;
pub use generated::new;
pub use generated::raw_hnds::*;
pub use generated::setter::*;
pub use generated::structs::*;

pub use extensions::*;
/// Vulkan extensions
pub mod extensions
{
    pub use crate::generated::extensions::*;
}

use crate::{Abi, CommandScope, Sys, Vk, generated};
use ::core::ffi::c_void;
use ::core::num::NonZeroU64;

/// Vulkan core
#[derive(Debug, Clone, Copy)]
pub struct core;

impl CommandScope<Instance> for core {
    type Commands = crate::commands::Instance;
}
impl CommandScope<Device> for core {
    type Commands = crate::commands::Device;
}

pub type StdVideoAV1Level = u32;
pub type StdVideoAV1Profile = u32;
pub type StdVideoH264LevelIdc = u32;
pub type StdVideoH265LevelIdc = u32;
pub type StdVideoH264ProfileIdc = u32;
pub type StdVideoH265ProfileIdc = u32;
pub type StdVideoVP9Level = u32;
pub type StdVideoVP9Profile = u32;
pub type StdVideoDecodeAV1ReferenceInfo = c_void;
pub type StdVideoAV1SequenceHeader = c_void;
pub type StdVideoDecodeAV1PictureInfo = c_void;
pub type StdVideoDecodeH264ReferenceInfo = c_void;
pub type StdVideoH264SequenceParameterSet = c_void;
pub type StdVideoH264PictureParameterSet = c_void;
pub type StdVideoDecodeH264PictureInfo = c_void;
pub type StdVideoDecodeH265ReferenceInfo = c_void;
pub type StdVideoH265VideoParameterSet = c_void;
pub type StdVideoH265SequenceParameterSet = c_void;
pub type StdVideoH265PictureParameterSet = c_void;
pub type StdVideoDecodeH265PictureInfo = c_void;
pub type StdVideoDecodeVP9PictureInfo = c_void;
pub type StdVideoEncodeAV1ReferenceInfo = c_void;
pub type StdVideoEncodeAV1PictureInfo = c_void;
pub type StdVideoEncodeAV1DecoderModelInfo = c_void;
pub type StdVideoEncodeAV1OperatingPointInfo = c_void;
pub type StdVideoEncodeH264ReferenceInfo = c_void;
pub type StdVideoEncodeH264SliceHeader = c_void;
pub type StdVideoEncodeH264PictureInfo = c_void;
pub type StdVideoEncodeH265ReferenceInfo = c_void;
pub type StdVideoEncodeH265SliceSegmentHeader = c_void;
pub type StdVideoEncodeH265PictureInfo = c_void;

/// `1.0`
pub const API_VERSION_1_0: u32 = sys::VK_API_VERSION_1_0;
/// `1.1`
pub const API_VERSION_1_1: u32 = sys::VK_API_VERSION_1_1;
/// `1.2`
pub const API_VERSION_1_2: u32 = sys::VK_API_VERSION_1_2;
/// `1.3`
pub const API_VERSION_1_3: u32 = sys::VK_API_VERSION_1_3;
/// `1.4`
pub const API_VERSION_1_4: u32 = sys::VK_API_VERSION_1_4;

/// `VkSampleMask`
pub type SampleMask = sys::VkSampleMask;

/// `VkBool32`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Bool(pub sys::VkBool32);
from_into_transparent!(Bool : sys::VkBool32);

impl Abi<sys::VkBool32> for Bool {
    fn abi(self) -> sys::VkBool32 {
        self.0
    }
}

impl<'a> Abi<*const sys::VkBool32> for &'a Bool {
    fn abi(self) -> *const sys::VkBool32 {
        &self.0
    }
}

impl<'a> Abi<*mut sys::VkBool32> for &'a mut Bool {
    fn abi(self) -> *mut sys::VkBool32 {
        &mut self.0
    }
}

impl<'a> Abi<*const sys::VkBool32> for &'a [Bool] {
    fn abi(self) -> *const sys::VkBool32 {
        unsafe { self.as_ptr().cast() }
    }
}

impl<'a> Abi<*mut sys::VkBool32> for &'a mut [Bool] {
    fn abi(self) -> *mut sys::VkBool32 {
        unsafe { self.as_mut_ptr().cast() }
    }
}

impl<'a> Vk<'a> for sys::VkBool32 {
    type Target = Bool;

    fn vk(self) -> Self::Target {
        Bool(self)
    }
}

impl<'a> Sys<'a> for Bool {
    type Target = sys::VkBool32;

    fn sys(self) -> Self::Target {
        self.0
    }
}

/// `1`
pub const TRUE: Bool = Bool::True;
/// `0`
pub const FALSE: Bool = Bool::False;

impl Bool {
    /// `1`
    pub const True: Self = Self(1);
    /// `0`
    pub const False: Self = Self(0);

    /// As `bool``
    pub const fn bool(self) -> bool {
        self.0 != 0
    }
}

impl From<Bool> for bool {
    fn from(value: Bool) -> Self {
        value.0 != 0
    }
}

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        Self(if value { 1 } else { 0 })
    }
}

impl ::core::fmt::Debug for Bool {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        if self.bool() {
            write!(f, "true")
        } else {
            write!(f, "false")
        }
    }
}

impl ::core::fmt::Display for Bool {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        if self.bool() {
            write!(f, "true")
        } else {
            write!(f, "false")
        }
    }
}

/// `VkDeviceSize`
pub type DeviceSize = sys::VkDeviceSize;

/// `VkDeviceAddress`
pub type DeviceAddress = sys::VkDeviceAddress;

/// `VkRemoteAddressNV`
pub type RemoteAddressNV = ffi::VkRemoteAddressNV;
