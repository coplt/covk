// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_AMD_draw_indirect_count` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::amd::draw_indirect_count::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::amd::draw_indirect_count::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdDrawIndexedIndirectCount(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkBuffer countBuffer, VkDeviceSize countBufferOffset, uint32_t maxDrawCount, uint32_t stride)
    /// ```
    pub unsafe fn cmd_draw_indexed_indirect_count(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: uint32_t,
        stride: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdDrawIndexedIndirectCountAMD(
                command_buffer.abi(), 
                buffer.abi(), 
                offset.abi(), 
                count_buffer.abi(), 
                count_buffer_offset.abi(), 
                max_draw_count.abi(), 
                stride.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdDrawIndirectCount(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkBuffer countBuffer, VkDeviceSize countBufferOffset, uint32_t maxDrawCount, uint32_t stride)
    /// ```
    pub unsafe fn cmd_draw_indirect_count(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: uint32_t,
        stride: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdDrawIndirectCountAMD(
                command_buffer.abi(), 
                buffer.abi(), 
                offset.abi(), 
                count_buffer.abi(), 
                count_buffer_offset.abi(), 
                max_draw_count.abi(), 
                stride.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::amd::draw_indirect_count {
    type Commands = Device;
}

/// Device object
pub trait AmdDrawIndirectCountDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::amd::draw_indirect_count {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::amd::draw_indirect_count> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::amd::draw_indirect_count {
        type Output = crate::hnd::Device<vk::extensions::amd::draw_indirect_count>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::amd::draw_indirect_count>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::amd::draw_indirect_count> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::amd::draw_indirect_count> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::amd::draw_indirect_count> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::amd::draw_indirect_count> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::AmdDrawIndirectCountDevice for crate::hnd::Device<vk::extensions::amd::draw_indirect_count> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::amd::draw_indirect_count, vk::Device> for crate::hnd::Device<vk::extensions::amd::draw_indirect_count> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait AmdDrawIndirectCountCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdDrawIndexedIndirectCount(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkBuffer countBuffer, VkDeviceSize countBufferOffset, uint32_t maxDrawCount, uint32_t stride)
    /// ```
    unsafe fn draw_indexed_indirect_count(
        &self,
        buffer: vk::Buffer,
        offset: DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: uint32_t,
        stride: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_draw_indexed_indirect_count(
                self.raw(),
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            )
        }
    }
    /// ```c
    /// void vkCmdDrawIndirectCount(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkBuffer countBuffer, VkDeviceSize countBufferOffset, uint32_t maxDrawCount, uint32_t stride)
    /// ```
    unsafe fn draw_indirect_count(
        &self,
        buffer: vk::Buffer,
        offset: DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: uint32_t,
        stride: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_draw_indirect_count(
                self.raw(),
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            )
        }
    }
}
