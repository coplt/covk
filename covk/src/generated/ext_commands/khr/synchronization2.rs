// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_synchronization2` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::synchronization2::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::synchronization2::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdPipelineBarrier2(VkCommandBuffer commandBuffer, VkDependencyInfo const* pDependencyInfo)
    /// ```
    pub unsafe fn cmd_pipeline_barrier2(
        &self,
        command_buffer: vk::CommandBuffer,
        dependency_info: &DependencyInfo,
    ) -> () {
        unsafe {
            self.0.CmdPipelineBarrier2KHR(
                command_buffer.abi(), 
                dependency_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdResetEvent2(VkCommandBuffer commandBuffer, VkEvent event, VkPipelineStageFlags2 stageMask)
    /// ```
    pub unsafe fn cmd_reset_event2(
        &self,
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        stage_mask: PipelineStageFlags2,
    ) -> () {
        unsafe {
            self.0.CmdResetEvent2KHR(
                command_buffer.abi(), 
                event.abi(), 
                stage_mask.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetEvent2(VkCommandBuffer commandBuffer, VkEvent event, VkDependencyInfo const* pDependencyInfo)
    /// ```
    pub unsafe fn cmd_set_event2(
        &self,
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        dependency_info: &DependencyInfo,
    ) -> () {
        unsafe {
            self.0.CmdSetEvent2KHR(
                command_buffer.abi(), 
                event.abi(), 
                dependency_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdWaitEvents2(VkCommandBuffer commandBuffer, uint32_t eventCount, VkEvent const* pEvents, VkDependencyInfo const* pDependencyInfos)
    /// ```
    pub unsafe fn cmd_wait_events2(
        &self,
        command_buffer: vk::CommandBuffer,
        events: &[vk::Event],
        dependency_infos: &[DependencyInfo],
    ) -> () {
        unsafe {
            self.0.CmdWaitEvents2KHR(
                command_buffer.abi(), 
                events.len() as _, 
                events.abi(), 
                dependency_infos.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdWriteTimestamp2(VkCommandBuffer commandBuffer, VkPipelineStageFlags2 stage, VkQueryPool queryPool, uint32_t query)
    /// ```
    pub unsafe fn cmd_write_timestamp2(
        &self,
        command_buffer: vk::CommandBuffer,
        stage: PipelineStageFlags2,
        query_pool: vk::QueryPool,
        query: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdWriteTimestamp2KHR(
                command_buffer.abi(), 
                stage.abi(), 
                query_pool.abi(), 
                query.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkQueueSubmit2(VkQueue queue, uint32_t submitCount, VkSubmitInfo2 const* pSubmits, VkFence fence)
    /// ```
    pub unsafe fn queue_submit2(
        &self,
        queue: vk::Queue,
        submits: &[SubmitInfo2],
        fence: Option<vk::Fence>,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.QueueSubmit2KHR(
                queue.abi(), 
                submits.len() as _, 
                submits.abi(), 
                fence.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::synchronization2 {
    type Commands = Device;
}

/// Device object
pub trait KhrSynchronization2Device {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::khr::synchronization2 {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::synchronization2> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::synchronization2 {
        type Output = crate::hnd::Device<vk::extensions::khr::synchronization2>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::synchronization2>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::synchronization2> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::synchronization2> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::synchronization2> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::synchronization2> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrSynchronization2Device for crate::hnd::Device<vk::extensions::khr::synchronization2> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::synchronization2, vk::Device> for crate::hnd::Device<vk::extensions::khr::synchronization2> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait KhrSynchronization2CommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdPipelineBarrier2(VkCommandBuffer commandBuffer, VkDependencyInfo const* pDependencyInfo)
    /// ```
    unsafe fn pipeline_barrier2(
        &self,
        dependency_info: &DependencyInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_pipeline_barrier2(
                self.raw(),
                dependency_info,
            )
        }
    }
    /// ```c
    /// void vkCmdResetEvent2(VkCommandBuffer commandBuffer, VkEvent event, VkPipelineStageFlags2 stageMask)
    /// ```
    unsafe fn reset_event2(
        &self,
        event: vk::Event,
        stage_mask: PipelineStageFlags2,
    ) -> () {
        unsafe {
            self.commands().cmd_reset_event2(
                self.raw(),
                event,
                stage_mask,
            )
        }
    }
    /// ```c
    /// void vkCmdSetEvent2(VkCommandBuffer commandBuffer, VkEvent event, VkDependencyInfo const* pDependencyInfo)
    /// ```
    unsafe fn set_event2(
        &self,
        event: vk::Event,
        dependency_info: &DependencyInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_set_event2(
                self.raw(),
                event,
                dependency_info,
            )
        }
    }
    /// ```c
    /// void vkCmdWaitEvents2(VkCommandBuffer commandBuffer, uint32_t eventCount, VkEvent const* pEvents, VkDependencyInfo const* pDependencyInfos)
    /// ```
    unsafe fn wait_events2(
        &self,
        events: &[vk::Event],
        dependency_infos: &[DependencyInfo],
    ) -> () {
        unsafe {
            self.commands().cmd_wait_events2(
                self.raw(),
                events,
                dependency_infos,
            )
        }
    }
    /// ```c
    /// void vkCmdWriteTimestamp2(VkCommandBuffer commandBuffer, VkPipelineStageFlags2 stage, VkQueryPool queryPool, uint32_t query)
    /// ```
    unsafe fn write_timestamp2(
        &self,
        stage: PipelineStageFlags2,
        query_pool: vk::QueryPool,
        query: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_write_timestamp2(
                self.raw(),
                stage,
                query_pool,
                query,
            )
        }
    }
}

/// Queue object
pub trait KhrSynchronization2Queue {
    fn raw(&self) -> vk::Queue;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkQueueSubmit2(VkQueue queue, uint32_t submitCount, VkSubmitInfo2 const* pSubmits, VkFence fence)
    /// ```
    unsafe fn submit2(
        &self,
        submits: &[SubmitInfo2],
        fence: Option<vk::Fence>,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().queue_submit2(
                self.raw(),
                submits,
                fence,
            )
        }
    }
}
