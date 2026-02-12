// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_AMD_buffer_marker` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::amd::buffer_marker::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::amd::buffer_marker::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdWriteBufferMarker2AMD(VkCommandBuffer commandBuffer, VkPipelineStageFlags2 stage, VkBuffer dstBuffer, VkDeviceSize dstOffset, uint32_t marker)
    /// ```
    pub unsafe fn cmd_write_buffer_marker2(
        &self,
        command_buffer: vk::CommandBuffer,
        stage: PipelineStageFlags2,
        dst_buffer: vk::Buffer,
        dst_offset: DeviceSize,
        marker: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdWriteBufferMarker2AMD(
                command_buffer.abi(), 
                stage.abi(), 
                dst_buffer.abi(), 
                dst_offset.abi(), 
                marker.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdWriteBufferMarkerAMD(VkCommandBuffer commandBuffer, VkPipelineStageFlagBits pipelineStage, VkBuffer dstBuffer, VkDeviceSize dstOffset, uint32_t marker)
    /// ```
    pub unsafe fn cmd_write_buffer_marker(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_stage: PipelineStageFlags,
        dst_buffer: vk::Buffer,
        dst_offset: DeviceSize,
        marker: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdWriteBufferMarkerAMD(
                command_buffer.abi(), 
                pipeline_stage.abi(), 
                dst_buffer.abi(), 
                dst_offset.abi(), 
                marker.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::amd::buffer_marker {
    type Commands = Device;
}

/// Device object
pub trait AmdBufferMarkerDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::amd::buffer_marker {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::amd::buffer_marker> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::amd::buffer_marker {
        type Output = crate::hnd::Device<vk::extensions::amd::buffer_marker>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::amd::buffer_marker>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::amd::buffer_marker> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::amd::buffer_marker> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::amd::buffer_marker> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::amd::buffer_marker> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::AmdBufferMarkerDevice for crate::hnd::Device<vk::extensions::amd::buffer_marker> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::amd::buffer_marker, vk::Device> for crate::hnd::Device<vk::extensions::amd::buffer_marker> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait AmdBufferMarkerCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdWriteBufferMarker2AMD(VkCommandBuffer commandBuffer, VkPipelineStageFlags2 stage, VkBuffer dstBuffer, VkDeviceSize dstOffset, uint32_t marker)
    /// ```
    unsafe fn write_buffer_marker2(
        &self,
        stage: PipelineStageFlags2,
        dst_buffer: vk::Buffer,
        dst_offset: DeviceSize,
        marker: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_write_buffer_marker2(
                self.raw(),
                stage,
                dst_buffer,
                dst_offset,
                marker,
            )
        }
    }
    /// ```c
    /// void vkCmdWriteBufferMarkerAMD(VkCommandBuffer commandBuffer, VkPipelineStageFlagBits pipelineStage, VkBuffer dstBuffer, VkDeviceSize dstOffset, uint32_t marker)
    /// ```
    unsafe fn write_buffer_marker(
        &self,
        pipeline_stage: PipelineStageFlags,
        dst_buffer: vk::Buffer,
        dst_offset: DeviceSize,
        marker: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_write_buffer_marker(
                self.raw(),
                pipeline_stage,
                dst_buffer,
                dst_offset,
                marker,
            )
        }
    }
}
