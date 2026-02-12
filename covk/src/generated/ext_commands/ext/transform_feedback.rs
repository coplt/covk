// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_transform_feedback` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::transform_feedback::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::transform_feedback::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdBeginQueryIndexedEXT(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t query, VkQueryControlFlags flags, uint32_t index)
    /// ```
    pub unsafe fn cmd_begin_query_indexed(
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        query: uint32_t,
        flags: QueryControlFlags,
        index: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdBeginQueryIndexedEXT(
                command_buffer.abi(), 
                query_pool.abi(), 
                query.abi(), 
                flags.abi(), 
                index.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdBeginTransformFeedbackEXT(VkCommandBuffer commandBuffer, uint32_t firstCounterBuffer, uint32_t counterBufferCount, VkBuffer const* pCounterBuffers, VkDeviceSize const* pCounterBufferOffsets)
    /// ```
    pub unsafe fn cmd_begin_transform_feedback(
        &self,
        command_buffer: vk::CommandBuffer,
        first_counter_buffer: uint32_t,
        counter_buffers: &[vk::Buffer],
        counter_buffer_offsets: Option<&[DeviceSize]>,
    ) -> () {
        unsafe {
            self.0.CmdBeginTransformFeedbackEXT(
                command_buffer.abi(), 
                first_counter_buffer.abi(), 
                counter_buffers.len() as _, 
                counter_buffers.abi(), 
                counter_buffer_offsets.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdBindTransformFeedbackBuffersEXT(VkCommandBuffer commandBuffer, uint32_t firstBinding, uint32_t bindingCount, VkBuffer const* pBuffers, VkDeviceSize const* pOffsets, VkDeviceSize const* pSizes)
    /// ```
    pub unsafe fn cmd_bind_transform_feedback_buffers(
        &self,
        command_buffer: vk::CommandBuffer,
        first_binding: uint32_t,
        buffers: &[vk::Buffer],
        offsets: &[DeviceSize],
        sizes: Option<&[DeviceSize]>,
    ) -> () {
        unsafe {
            self.0.CmdBindTransformFeedbackBuffersEXT(
                command_buffer.abi(), 
                first_binding.abi(), 
                buffers.len() as _, 
                buffers.abi(), 
                offsets.abi(), 
                sizes.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdDrawIndirectByteCountEXT(VkCommandBuffer commandBuffer, uint32_t instanceCount, uint32_t firstInstance, VkBuffer counterBuffer, VkDeviceSize counterBufferOffset, uint32_t counterOffset, uint32_t vertexStride)
    /// ```
    pub unsafe fn cmd_draw_indirect_byte_count(
        &self,
        command_buffer: vk::CommandBuffer,
        instance_count: uint32_t,
        first_instance: uint32_t,
        counter_buffer: vk::Buffer,
        counter_buffer_offset: DeviceSize,
        counter_offset: uint32_t,
        vertex_stride: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdDrawIndirectByteCountEXT(
                command_buffer.abi(), 
                instance_count.abi(), 
                first_instance.abi(), 
                counter_buffer.abi(), 
                counter_buffer_offset.abi(), 
                counter_offset.abi(), 
                vertex_stride.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdEndQueryIndexedEXT(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t query, uint32_t index)
    /// ```
    pub unsafe fn cmd_end_query_indexed(
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        query: uint32_t,
        index: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdEndQueryIndexedEXT(
                command_buffer.abi(), 
                query_pool.abi(), 
                query.abi(), 
                index.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdEndTransformFeedbackEXT(VkCommandBuffer commandBuffer, uint32_t firstCounterBuffer, uint32_t counterBufferCount, VkBuffer const* pCounterBuffers, VkDeviceSize const* pCounterBufferOffsets)
    /// ```
    pub unsafe fn cmd_end_transform_feedback(
        &self,
        command_buffer: vk::CommandBuffer,
        first_counter_buffer: uint32_t,
        counter_buffers: &[vk::Buffer],
        counter_buffer_offsets: Option<&[DeviceSize]>,
    ) -> () {
        unsafe {
            self.0.CmdEndTransformFeedbackEXT(
                command_buffer.abi(), 
                first_counter_buffer.abi(), 
                counter_buffers.len() as _, 
                counter_buffers.abi(), 
                counter_buffer_offsets.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::transform_feedback {
    type Commands = Device;
}

/// Device object
pub trait ExtTransformFeedbackDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::ext::transform_feedback {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::transform_feedback> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::transform_feedback {
        type Output = crate::hnd::Device<vk::extensions::ext::transform_feedback>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::transform_feedback>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::transform_feedback> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::transform_feedback> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::transform_feedback> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::transform_feedback> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtTransformFeedbackDevice for crate::hnd::Device<vk::extensions::ext::transform_feedback> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::transform_feedback, vk::Device> for crate::hnd::Device<vk::extensions::ext::transform_feedback> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait ExtTransformFeedbackCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdBeginQueryIndexedEXT(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t query, VkQueryControlFlags flags, uint32_t index)
    /// ```
    unsafe fn begin_query_indexed(
        &self,
        query_pool: vk::QueryPool,
        query: uint32_t,
        flags: QueryControlFlags,
        index: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_begin_query_indexed(
                self.raw(),
                query_pool,
                query,
                flags,
                index,
            )
        }
    }
    /// ```c
    /// void vkCmdBeginTransformFeedbackEXT(VkCommandBuffer commandBuffer, uint32_t firstCounterBuffer, uint32_t counterBufferCount, VkBuffer const* pCounterBuffers, VkDeviceSize const* pCounterBufferOffsets)
    /// ```
    unsafe fn begin_transform_feedback(
        &self,
        first_counter_buffer: uint32_t,
        counter_buffers: &[vk::Buffer],
        counter_buffer_offsets: Option<&[DeviceSize]>,
    ) -> () {
        unsafe {
            self.commands().cmd_begin_transform_feedback(
                self.raw(),
                first_counter_buffer,
                counter_buffers,
                counter_buffer_offsets,
            )
        }
    }
    /// ```c
    /// void vkCmdBindTransformFeedbackBuffersEXT(VkCommandBuffer commandBuffer, uint32_t firstBinding, uint32_t bindingCount, VkBuffer const* pBuffers, VkDeviceSize const* pOffsets, VkDeviceSize const* pSizes)
    /// ```
    unsafe fn bind_transform_feedback_buffers(
        &self,
        first_binding: uint32_t,
        buffers: &[vk::Buffer],
        offsets: &[DeviceSize],
        sizes: Option<&[DeviceSize]>,
    ) -> () {
        unsafe {
            self.commands().cmd_bind_transform_feedback_buffers(
                self.raw(),
                first_binding,
                buffers,
                offsets,
                sizes,
            )
        }
    }
    /// ```c
    /// void vkCmdDrawIndirectByteCountEXT(VkCommandBuffer commandBuffer, uint32_t instanceCount, uint32_t firstInstance, VkBuffer counterBuffer, VkDeviceSize counterBufferOffset, uint32_t counterOffset, uint32_t vertexStride)
    /// ```
    unsafe fn draw_indirect_byte_count(
        &self,
        instance_count: uint32_t,
        first_instance: uint32_t,
        counter_buffer: vk::Buffer,
        counter_buffer_offset: DeviceSize,
        counter_offset: uint32_t,
        vertex_stride: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_draw_indirect_byte_count(
                self.raw(),
                instance_count,
                first_instance,
                counter_buffer,
                counter_buffer_offset,
                counter_offset,
                vertex_stride,
            )
        }
    }
    /// ```c
    /// void vkCmdEndQueryIndexedEXT(VkCommandBuffer commandBuffer, VkQueryPool queryPool, uint32_t query, uint32_t index)
    /// ```
    unsafe fn end_query_indexed(
        &self,
        query_pool: vk::QueryPool,
        query: uint32_t,
        index: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_end_query_indexed(
                self.raw(),
                query_pool,
                query,
                index,
            )
        }
    }
    /// ```c
    /// void vkCmdEndTransformFeedbackEXT(VkCommandBuffer commandBuffer, uint32_t firstCounterBuffer, uint32_t counterBufferCount, VkBuffer const* pCounterBuffers, VkDeviceSize const* pCounterBufferOffsets)
    /// ```
    unsafe fn end_transform_feedback(
        &self,
        first_counter_buffer: uint32_t,
        counter_buffers: &[vk::Buffer],
        counter_buffer_offsets: Option<&[DeviceSize]>,
    ) -> () {
        unsafe {
            self.commands().cmd_end_transform_feedback(
                self.raw(),
                first_counter_buffer,
                counter_buffers,
                counter_buffer_offsets,
            )
        }
    }
}
