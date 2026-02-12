// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_maintenance5` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::maintenance5::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::maintenance5::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdBindIndexBuffer2(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkDeviceSize size, VkIndexType indexType)
    /// ```
    pub unsafe fn cmd_bind_index_buffer2(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: Option<vk::Buffer>,
        offset: DeviceSize,
        size: DeviceSize,
        index_type: IndexType,
    ) -> () {
        unsafe {
            self.0.CmdBindIndexBuffer2KHR(
                command_buffer.abi(), 
                buffer.abi(), 
                offset.abi(), 
                size.abi(), 
                index_type.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetDeviceImageSubresourceLayout(VkDevice device, VkDeviceImageSubresourceInfo const* pInfo, VkSubresourceLayout2* pLayout)
    /// ```
    pub unsafe fn get_device_image_subresource_layout(
        &self,
        device: vk::Device,
        info: &DeviceImageSubresourceInfo,
        layout: &mut SubresourceLayout2,
    ) -> () {
        unsafe {
            self.0.GetDeviceImageSubresourceLayoutKHR(
                device.abi(), 
                info.abi(), 
                layout.abi(), 
            );
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
            self.0.GetImageSubresourceLayout2KHR(
                device.abi(), 
                image.abi(), 
                subresource.abi(), 
                layout.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetRenderingAreaGranularity(VkDevice device, VkRenderingAreaInfo const* pRenderingAreaInfo, VkExtent2D* pGranularity)
    /// ```
    pub unsafe fn get_rendering_area_granularity(
        &self,
        device: vk::Device,
        rendering_area_info: &RenderingAreaInfo,
    ) -> Extent2D {
        unsafe {
            let mut _v: Extent2D = Default::default();
            self.0.GetRenderingAreaGranularityKHR(
                device.abi(), 
                rendering_area_info.abi(), 
                (&mut _v).abi(), 
            );
            _v
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::maintenance5 {
    type Commands = Device;
}

/// Device object
pub trait KhrMaintenance5Device {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkGetDeviceImageSubresourceLayout(VkDevice device, VkDeviceImageSubresourceInfo const* pInfo, VkSubresourceLayout2* pLayout)
    /// ```
    unsafe fn get_device_image_subresource_layout(
        &self,
        info: &DeviceImageSubresourceInfo,
        layout: &mut SubresourceLayout2,
    ) -> () {
        unsafe {
            self.commands().get_device_image_subresource_layout(
                self.raw(),
                info,
                layout,
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
    /// void vkGetRenderingAreaGranularity(VkDevice device, VkRenderingAreaInfo const* pRenderingAreaInfo, VkExtent2D* pGranularity)
    /// ```
    unsafe fn get_rendering_area_granularity(
        &self,
        rendering_area_info: &RenderingAreaInfo,
    ) -> Extent2D {
        unsafe {
            self.commands().get_rendering_area_granularity(
                self.raw(),
                rendering_area_info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::maintenance5 {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::maintenance5> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::maintenance5 {
        type Output = crate::hnd::Device<vk::extensions::khr::maintenance5>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::maintenance5>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::maintenance5> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::maintenance5> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::maintenance5> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::maintenance5> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrMaintenance5Device for crate::hnd::Device<vk::extensions::khr::maintenance5> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::maintenance5, vk::Device> for crate::hnd::Device<vk::extensions::khr::maintenance5> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait KhrMaintenance5CommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdBindIndexBuffer2(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkDeviceSize size, VkIndexType indexType)
    /// ```
    unsafe fn bind_index_buffer2(
        &self,
        buffer: Option<vk::Buffer>,
        offset: DeviceSize,
        size: DeviceSize,
        index_type: IndexType,
    ) -> () {
        unsafe {
            self.commands().cmd_bind_index_buffer2(
                self.raw(),
                buffer,
                offset,
                size,
                index_type,
            )
        }
    }
}
