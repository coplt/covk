// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_image_compression_control` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::image_compression_control::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::image_compression_control::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkGetImageSubresourceLayout2(VkDevice device, VkImage image, VkImageSubresource2 const* pSubresource, VkSubresourceLayout2* pLayout)
    /// ```
    pub unsafe fn get_image_subresource_layout2(
        &self,
        device: vk::Device,
        image: vk::Image,
        subresource: &ImageSubresource2,
        layout: &mut SubresourceLayout2,
    ) -> () {
        unsafe {
            self.0.GetImageSubresourceLayout2EXT(
                device.abi(), 
                image.abi(), 
                subresource.abi(), 
                layout.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::image_compression_control {
    type Commands = Device;
}

/// Device object
pub trait ExtImageCompressionControlDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkGetImageSubresourceLayout2(VkDevice device, VkImage image, VkImageSubresource2 const* pSubresource, VkSubresourceLayout2* pLayout)
    /// ```
    unsafe fn get_image_subresource_layout2(
        &self,
        image: vk::Image,
        subresource: &ImageSubresource2,
        layout: &mut SubresourceLayout2,
    ) -> () {
        unsafe {
            self.commands().get_image_subresource_layout2(
                self.raw(),
                image,
                subresource,
                layout,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::ext::image_compression_control {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::image_compression_control> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::image_compression_control {
        type Output = crate::hnd::Device<vk::extensions::ext::image_compression_control>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::image_compression_control>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::image_compression_control> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::image_compression_control> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::image_compression_control> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::image_compression_control> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtImageCompressionControlDevice for crate::hnd::Device<vk::extensions::ext::image_compression_control> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::image_compression_control, vk::Device> for crate::hnd::Device<vk::extensions::ext::image_compression_control> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
