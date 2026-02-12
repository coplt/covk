// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_NV_copy_memory_indirect` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::nv::copy_memory_indirect::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::copy_memory_indirect::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdCopyMemoryIndirectNV(VkCommandBuffer commandBuffer, VkDeviceAddress copyBufferAddress, uint32_t copyCount, uint32_t stride)
    /// ```
    pub unsafe fn cmd_copy_memory_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_buffer_address: DeviceAddress,
        copy_count: uint32_t,
        stride: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdCopyMemoryIndirectNV(
                command_buffer.abi(), 
                copy_buffer_address.abi(), 
                copy_count.abi(), 
                stride.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdCopyMemoryToImageIndirectNV(VkCommandBuffer commandBuffer, VkDeviceAddress copyBufferAddress, uint32_t copyCount, uint32_t stride, VkImage dstImage, VkImageLayout dstImageLayout, VkImageSubresourceLayers const* pImageSubresources)
    /// ```
    pub unsafe fn cmd_copy_memory_to_image_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_buffer_address: DeviceAddress,
        stride: uint32_t,
        dst_image: vk::Image,
        dst_image_layout: ImageLayout,
        image_subresources: &[ImageSubresourceLayers],
    ) -> () {
        unsafe {
            self.0.CmdCopyMemoryToImageIndirectNV(
                command_buffer.abi(), 
                copy_buffer_address.abi(), 
                image_subresources.len() as _, 
                stride.abi(), 
                dst_image.abi(), 
                dst_image_layout.abi(), 
                image_subresources.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::nv::copy_memory_indirect {
    type Commands = Device;
}

/// Device object
pub trait NvCopyMemoryIndirectDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::nv::copy_memory_indirect {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::nv::copy_memory_indirect> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::nv::copy_memory_indirect {
        type Output = crate::hnd::Device<vk::extensions::nv::copy_memory_indirect>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::nv::copy_memory_indirect>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::nv::copy_memory_indirect> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::nv::copy_memory_indirect> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::nv::copy_memory_indirect> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::nv::copy_memory_indirect> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvCopyMemoryIndirectDevice for crate::hnd::Device<vk::extensions::nv::copy_memory_indirect> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::copy_memory_indirect, vk::Device> for crate::hnd::Device<vk::extensions::nv::copy_memory_indirect> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait NvCopyMemoryIndirectCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdCopyMemoryIndirectNV(VkCommandBuffer commandBuffer, VkDeviceAddress copyBufferAddress, uint32_t copyCount, uint32_t stride)
    /// ```
    unsafe fn copy_memory_indirect(
        &self,
        copy_buffer_address: DeviceAddress,
        copy_count: uint32_t,
        stride: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_copy_memory_indirect(
                self.raw(),
                copy_buffer_address,
                copy_count,
                stride,
            )
        }
    }
    /// ```c
    /// void vkCmdCopyMemoryToImageIndirectNV(VkCommandBuffer commandBuffer, VkDeviceAddress copyBufferAddress, uint32_t copyCount, uint32_t stride, VkImage dstImage, VkImageLayout dstImageLayout, VkImageSubresourceLayers const* pImageSubresources)
    /// ```
    unsafe fn copy_memory_to_image_indirect(
        &self,
        copy_buffer_address: DeviceAddress,
        stride: uint32_t,
        dst_image: vk::Image,
        dst_image_layout: ImageLayout,
        image_subresources: &[ImageSubresourceLayers],
    ) -> () {
        unsafe {
            self.commands().cmd_copy_memory_to_image_indirect(
                self.raw(),
                copy_buffer_address,
                stride,
                dst_image,
                dst_image_layout,
                image_subresources,
            )
        }
    }
}
