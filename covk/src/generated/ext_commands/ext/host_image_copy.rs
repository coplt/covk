// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_host_image_copy` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::host_image_copy::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::host_image_copy::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkCopyImageToImage(VkDevice device, VkCopyImageToImageInfo const* pCopyImageToImageInfo)
    /// ```
    pub unsafe fn copy_image_to_image(
        &self,
        device: vk::Device,
        copy_image_to_image_info: &CopyImageToImageInfo,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.CopyImageToImageEXT(
                device.abi(), 
                copy_image_to_image_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkCopyImageToMemory(VkDevice device, VkCopyImageToMemoryInfo const* pCopyImageToMemoryInfo)
    /// ```
    pub unsafe fn copy_image_to_memory(
        &self,
        device: vk::Device,
        copy_image_to_memory_info: &CopyImageToMemoryInfo,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.CopyImageToMemoryEXT(
                device.abi(), 
                copy_image_to_memory_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkCopyMemoryToImage(VkDevice device, VkCopyMemoryToImageInfo const* pCopyMemoryToImageInfo)
    /// ```
    pub unsafe fn copy_memory_to_image(
        &self,
        device: vk::Device,
        copy_memory_to_image_info: &CopyMemoryToImageInfo,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.CopyMemoryToImageEXT(
                device.abi(), 
                copy_memory_to_image_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
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
    /// ```c
    /// VkResult vkTransitionImageLayout(VkDevice device, uint32_t transitionCount, VkHostImageLayoutTransitionInfo const* pTransitions)
    /// ```
    pub unsafe fn transition_image_layout(
        &self,
        device: vk::Device,
        transitions: &[HostImageLayoutTransitionInfo],
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.TransitionImageLayoutEXT(
                device.abi(), 
                transitions.len() as _, 
                transitions.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::host_image_copy {
    type Commands = Device;
}

/// Device object
pub trait ExtHostImageCopyDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkCopyImageToImage(VkDevice device, VkCopyImageToImageInfo const* pCopyImageToImageInfo)
    /// ```
    unsafe fn copy_image_to_image(
        &self,
        copy_image_to_image_info: &CopyImageToImageInfo,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().copy_image_to_image(
                self.raw(),
                copy_image_to_image_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCopyImageToMemory(VkDevice device, VkCopyImageToMemoryInfo const* pCopyImageToMemoryInfo)
    /// ```
    unsafe fn copy_image_to_memory(
        &self,
        copy_image_to_memory_info: &CopyImageToMemoryInfo,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().copy_image_to_memory(
                self.raw(),
                copy_image_to_memory_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCopyMemoryToImage(VkDevice device, VkCopyMemoryToImageInfo const* pCopyMemoryToImageInfo)
    /// ```
    unsafe fn copy_memory_to_image(
        &self,
        copy_memory_to_image_info: &CopyMemoryToImageInfo,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().copy_memory_to_image(
                self.raw(),
                copy_memory_to_image_info,
            )
        }
    }
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
    /// ```c
    /// VkResult vkTransitionImageLayout(VkDevice device, uint32_t transitionCount, VkHostImageLayoutTransitionInfo const* pTransitions)
    /// ```
    unsafe fn transition_image_layout(
        &self,
        transitions: &[HostImageLayoutTransitionInfo],
    ) -> crate::Result<()> {
        unsafe {
            self.commands().transition_image_layout(
                self.raw(),
                transitions,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::ext::host_image_copy {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::host_image_copy> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::host_image_copy {
        type Output = crate::hnd::Device<vk::extensions::ext::host_image_copy>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::host_image_copy>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::host_image_copy> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::host_image_copy> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::host_image_copy> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::host_image_copy> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtHostImageCopyDevice for crate::hnd::Device<vk::extensions::ext::host_image_copy> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::host_image_copy, vk::Device> for crate::hnd::Device<vk::extensions::ext::host_image_copy> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
