// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_hdr_metadata` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::hdr_metadata::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::hdr_metadata::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkSetHdrMetadataEXT(VkDevice device, uint32_t swapchainCount, VkSwapchainKHR const* pSwapchains, VkHdrMetadataEXT const* pMetadata)
    /// ```
    pub unsafe fn set_hdr_metadata(
        &self,
        device: vk::Device,
        swapchains: &[vk::SwapchainKHR],
        metadata: &[HdrMetadataEXT],
    ) -> () {
        unsafe {
            self.0.SetHdrMetadataEXT(
                device.abi(), 
                swapchains.len() as _, 
                swapchains.abi(), 
                metadata.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::hdr_metadata {
    type Commands = Device;
}

/// Device object
pub trait ExtHdrMetadataDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkSetHdrMetadataEXT(VkDevice device, uint32_t swapchainCount, VkSwapchainKHR const* pSwapchains, VkHdrMetadataEXT const* pMetadata)
    /// ```
    unsafe fn set_hdr_metadata(
        &self,
        swapchains: &[vk::SwapchainKHR],
        metadata: &[HdrMetadataEXT],
    ) -> () {
        unsafe {
            self.commands().set_hdr_metadata(
                self.raw(),
                swapchains,
                metadata,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::ext::hdr_metadata {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::hdr_metadata> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::hdr_metadata {
        type Output = crate::hnd::Device<vk::extensions::ext::hdr_metadata>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::hdr_metadata>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::hdr_metadata> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::hdr_metadata> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::hdr_metadata> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::hdr_metadata> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtHdrMetadataDevice for crate::hnd::Device<vk::extensions::ext::hdr_metadata> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::hdr_metadata, vk::Device> for crate::hnd::Device<vk::extensions::ext::hdr_metadata> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
