// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_NVX_image_view_handle` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::nvx::image_view_handle::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nvx::image_view_handle::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// uint64_t vkGetDeviceCombinedImageSamplerIndexNVX(VkDevice device, uint64_t imageViewIndex, uint64_t samplerIndex)
    /// ```
    pub unsafe fn get_device_combined_image_sampler_index(
        &self,
        device: vk::Device,
        image_view_index: uint64_t,
        sampler_index: uint64_t,
    ) -> uint64_t {
        unsafe {
            let _r = self.0.GetDeviceCombinedImageSamplerIndexNVX(
                device.abi(), 
                image_view_index.abi(), 
                sampler_index.abi(), 
            );
            _r
        }
    }
    /// ```c
    /// VkResult vkGetImageViewAddressNVX(VkDevice device, VkImageView imageView, VkImageViewAddressPropertiesNVX* pProperties)
    /// ```
    pub unsafe fn get_image_view_address(
        &self,
        device: vk::Device,
        image_view: vk::ImageView,
        properties: &mut ImageViewAddressPropertiesNVX,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetImageViewAddressNVX(
                device.abi(), 
                image_view.abi(), 
                properties.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// uint64_t vkGetImageViewHandle64NVX(VkDevice device, VkImageViewHandleInfoNVX const* pInfo)
    /// ```
    pub unsafe fn get_image_view_handle64(
        &self,
        device: vk::Device,
        info: &ImageViewHandleInfoNVX,
    ) -> uint64_t {
        unsafe {
            let _r = self.0.GetImageViewHandle64NVX(
                device.abi(), 
                info.abi(), 
            );
            _r
        }
    }
    /// ```c
    /// uint32_t vkGetImageViewHandleNVX(VkDevice device, VkImageViewHandleInfoNVX const* pInfo)
    /// ```
    pub unsafe fn get_image_view_handle(
        &self,
        device: vk::Device,
        info: &ImageViewHandleInfoNVX,
    ) -> uint32_t {
        unsafe {
            let _r = self.0.GetImageViewHandleNVX(
                device.abi(), 
                info.abi(), 
            );
            _r
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::nvx::image_view_handle {
    type Commands = Device;
}

/// Device object
pub trait NvxImageViewHandleDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// uint64_t vkGetDeviceCombinedImageSamplerIndexNVX(VkDevice device, uint64_t imageViewIndex, uint64_t samplerIndex)
    /// ```
    unsafe fn get_device_combined_image_sampler_index(
        &self,
        image_view_index: uint64_t,
        sampler_index: uint64_t,
    ) -> uint64_t {
        unsafe {
            self.commands().get_device_combined_image_sampler_index(
                self.raw(),
                image_view_index,
                sampler_index,
            )
        }
    }
    /// ```c
    /// VkResult vkGetImageViewAddressNVX(VkDevice device, VkImageView imageView, VkImageViewAddressPropertiesNVX* pProperties)
    /// ```
    unsafe fn get_image_view_address(
        &self,
        image_view: vk::ImageView,
        properties: &mut ImageViewAddressPropertiesNVX,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_image_view_address(
                self.raw(),
                image_view,
                properties,
            )
        }
    }
    /// ```c
    /// uint64_t vkGetImageViewHandle64NVX(VkDevice device, VkImageViewHandleInfoNVX const* pInfo)
    /// ```
    unsafe fn get_image_view_handle64(
        &self,
        info: &ImageViewHandleInfoNVX,
    ) -> uint64_t {
        unsafe {
            self.commands().get_image_view_handle64(
                self.raw(),
                info,
            )
        }
    }
    /// ```c
    /// uint32_t vkGetImageViewHandleNVX(VkDevice device, VkImageViewHandleInfoNVX const* pInfo)
    /// ```
    unsafe fn get_image_view_handle(
        &self,
        info: &ImageViewHandleInfoNVX,
    ) -> uint32_t {
        unsafe {
            self.commands().get_image_view_handle(
                self.raw(),
                info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::nvx::image_view_handle {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::nvx::image_view_handle> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::nvx::image_view_handle {
        type Output = crate::hnd::Device<vk::extensions::nvx::image_view_handle>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::nvx::image_view_handle>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::nvx::image_view_handle> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::nvx::image_view_handle> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::nvx::image_view_handle> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::nvx::image_view_handle> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvxImageViewHandleDevice for crate::hnd::Device<vk::extensions::nvx::image_view_handle> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nvx::image_view_handle, vk::Device> for crate::hnd::Device<vk::extensions::nvx::image_view_handle> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
