// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_ray_tracing_pipeline` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::ray_tracing_pipeline::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::ray_tracing_pipeline::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdSetRayTracingPipelineStackSizeKHR(VkCommandBuffer commandBuffer, uint32_t pipelineStackSize)
    /// ```
    pub unsafe fn cmd_set_ray_tracing_pipeline_stack_size(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_stack_size: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdSetRayTracingPipelineStackSizeKHR(
                command_buffer.abi(), 
                pipeline_stack_size.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdTraceRaysIndirectKHR(VkCommandBuffer commandBuffer, VkStridedDeviceAddressRegionKHR const* pRaygenShaderBindingTable, VkStridedDeviceAddressRegionKHR const* pMissShaderBindingTable, VkStridedDeviceAddressRegionKHR const* pHitShaderBindingTable, VkStridedDeviceAddressRegionKHR const* pCallableShaderBindingTable, VkDeviceAddress indirectDeviceAddress)
    /// ```
    pub unsafe fn cmd_trace_rays_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        raygen_shader_binding_table: &StridedDeviceAddressRegionKHR,
        miss_shader_binding_table: &StridedDeviceAddressRegionKHR,
        hit_shader_binding_table: &StridedDeviceAddressRegionKHR,
        callable_shader_binding_table: &StridedDeviceAddressRegionKHR,
        indirect_device_address: DeviceAddress,
    ) -> () {
        unsafe {
            self.0.CmdTraceRaysIndirectKHR(
                command_buffer.abi(), 
                raygen_shader_binding_table.abi(), 
                miss_shader_binding_table.abi(), 
                hit_shader_binding_table.abi(), 
                callable_shader_binding_table.abi(), 
                indirect_device_address.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdTraceRaysKHR(VkCommandBuffer commandBuffer, VkStridedDeviceAddressRegionKHR const* pRaygenShaderBindingTable, VkStridedDeviceAddressRegionKHR const* pMissShaderBindingTable, VkStridedDeviceAddressRegionKHR const* pHitShaderBindingTable, VkStridedDeviceAddressRegionKHR const* pCallableShaderBindingTable, uint32_t width, uint32_t height, uint32_t depth)
    /// ```
    pub unsafe fn cmd_trace_rays(
        &self,
        command_buffer: vk::CommandBuffer,
        raygen_shader_binding_table: &StridedDeviceAddressRegionKHR,
        miss_shader_binding_table: &StridedDeviceAddressRegionKHR,
        hit_shader_binding_table: &StridedDeviceAddressRegionKHR,
        callable_shader_binding_table: &StridedDeviceAddressRegionKHR,
        width: uint32_t,
        height: uint32_t,
        depth: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdTraceRaysKHR(
                command_buffer.abi(), 
                raygen_shader_binding_table.abi(), 
                miss_shader_binding_table.abi(), 
                hit_shader_binding_table.abi(), 
                callable_shader_binding_table.abi(), 
                width.abi(), 
                height.abi(), 
                depth.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkCreateRayTracingPipelinesKHR(VkDevice device, VkDeferredOperationKHR deferredOperation, VkPipelineCache pipelineCache, uint32_t createInfoCount, VkRayTracingPipelineCreateInfoKHR const* pCreateInfos, VkAllocationCallbacks const* pAllocator, VkPipeline* pPipelines)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::OperationDeferredKhr], [Result::OperationNotDeferredKhr], [Result::PipelineCompileRequiredExt]
    pub unsafe fn create_ray_tracing_pipelines(
        &self,
        device: vk::Device,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        pipeline_cache: Option<vk::PipelineCache>,
        create_infos: &[RayTracingPipelineCreateInfoKHR],
        pipelines: &mut [Option<vk::Pipeline>],
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.CreateRayTracingPipelinesKHR(
                device.abi(), 
                deferred_operation.abi(), 
                pipeline_cache.abi(), 
                create_infos.len() as _, 
                create_infos.abi(), 
                Default::default(), 
                pipelines.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// VkResult vkGetRayTracingCaptureReplayShaderGroupHandlesKHR(VkDevice device, VkPipeline pipeline, uint32_t firstGroup, uint32_t groupCount, size_t dataSize, void* pData)
    /// ```
    pub unsafe fn get_ray_tracing_capture_replay_shader_group_handles(
        &self,
        device: vk::Device,
        pipeline: vk::Pipeline,
        first_group: uint32_t,
        group_count: uint32_t,
        data_size: size_t,
        data: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetRayTracingCaptureReplayShaderGroupHandlesKHR(
                device.abi(), 
                pipeline.abi(), 
                first_group.abi(), 
                group_count.abi(), 
                data_size.abi(), 
                data.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkGetRayTracingShaderGroupHandlesKHR(VkDevice device, VkPipeline pipeline, uint32_t firstGroup, uint32_t groupCount, size_t dataSize, void* pData)
    /// ```
    pub unsafe fn get_ray_tracing_shader_group_handles(
        &self,
        device: vk::Device,
        pipeline: vk::Pipeline,
        first_group: uint32_t,
        group_count: uint32_t,
        data_size: size_t,
        data: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetRayTracingShaderGroupHandlesKHR(
                device.abi(), 
                pipeline.abi(), 
                first_group.abi(), 
                group_count.abi(), 
                data_size.abi(), 
                data.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkDeviceSize vkGetRayTracingShaderGroupStackSizeKHR(VkDevice device, VkPipeline pipeline, uint32_t group, VkShaderGroupShaderKHR groupShader)
    /// ```
    pub unsafe fn get_ray_tracing_shader_group_stack_size(
        &self,
        device: vk::Device,
        pipeline: vk::Pipeline,
        group: uint32_t,
        group_shader: ShaderGroupShaderKHR,
    ) -> DeviceSize {
        unsafe {
            let _r = self.0.GetRayTracingShaderGroupStackSizeKHR(
                device.abi(), 
                pipeline.abi(), 
                group.abi(), 
                group_shader.abi(), 
            );
            _r
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::ray_tracing_pipeline {
    type Commands = Device;
}

/// Device object
pub trait KhrRayTracingPipelineDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkCreateRayTracingPipelinesKHR(VkDevice device, VkDeferredOperationKHR deferredOperation, VkPipelineCache pipelineCache, uint32_t createInfoCount, VkRayTracingPipelineCreateInfoKHR const* pCreateInfos, VkAllocationCallbacks const* pAllocator, VkPipeline* pPipelines)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::OperationDeferredKhr], [Result::OperationNotDeferredKhr], [Result::PipelineCompileRequiredExt]
    unsafe fn create_ray_tracing_pipelines(
        &self,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        pipeline_cache: Option<vk::PipelineCache>,
        create_infos: &[RayTracingPipelineCreateInfoKHR],
        pipelines: &mut [Option<vk::Pipeline>],
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().create_ray_tracing_pipelines(
                self.raw(),
                deferred_operation,
                pipeline_cache,
                create_infos,
                pipelines,
            )
        }
    }
    /// ```c
    /// VkResult vkGetRayTracingCaptureReplayShaderGroupHandlesKHR(VkDevice device, VkPipeline pipeline, uint32_t firstGroup, uint32_t groupCount, size_t dataSize, void* pData)
    /// ```
    unsafe fn get_ray_tracing_capture_replay_shader_group_handles(
        &self,
        pipeline: vk::Pipeline,
        first_group: uint32_t,
        group_count: uint32_t,
        data_size: size_t,
        data: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_ray_tracing_capture_replay_shader_group_handles(
                self.raw(),
                pipeline,
                first_group,
                group_count,
                data_size,
                data,
            )
        }
    }
    /// ```c
    /// VkResult vkGetRayTracingShaderGroupHandlesKHR(VkDevice device, VkPipeline pipeline, uint32_t firstGroup, uint32_t groupCount, size_t dataSize, void* pData)
    /// ```
    unsafe fn get_ray_tracing_shader_group_handles(
        &self,
        pipeline: vk::Pipeline,
        first_group: uint32_t,
        group_count: uint32_t,
        data_size: size_t,
        data: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_ray_tracing_shader_group_handles(
                self.raw(),
                pipeline,
                first_group,
                group_count,
                data_size,
                data,
            )
        }
    }
    /// ```c
    /// VkDeviceSize vkGetRayTracingShaderGroupStackSizeKHR(VkDevice device, VkPipeline pipeline, uint32_t group, VkShaderGroupShaderKHR groupShader)
    /// ```
    unsafe fn get_ray_tracing_shader_group_stack_size(
        &self,
        pipeline: vk::Pipeline,
        group: uint32_t,
        group_shader: ShaderGroupShaderKHR,
    ) -> DeviceSize {
        unsafe {
            self.commands().get_ray_tracing_shader_group_stack_size(
                self.raw(),
                pipeline,
                group,
                group_shader,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::ray_tracing_pipeline {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::ray_tracing_pipeline> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::ray_tracing_pipeline {
        type Output = crate::hnd::Device<vk::extensions::khr::ray_tracing_pipeline>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::ray_tracing_pipeline>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::ray_tracing_pipeline> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::ray_tracing_pipeline> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::ray_tracing_pipeline> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::ray_tracing_pipeline> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrRayTracingPipelineDevice for crate::hnd::Device<vk::extensions::khr::ray_tracing_pipeline> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::ray_tracing_pipeline, vk::Device> for crate::hnd::Device<vk::extensions::khr::ray_tracing_pipeline> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait KhrRayTracingPipelineCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdSetRayTracingPipelineStackSizeKHR(VkCommandBuffer commandBuffer, uint32_t pipelineStackSize)
    /// ```
    unsafe fn set_ray_tracing_pipeline_stack_size(
        &self,
        pipeline_stack_size: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_set_ray_tracing_pipeline_stack_size(
                self.raw(),
                pipeline_stack_size,
            )
        }
    }
    /// ```c
    /// void vkCmdTraceRaysIndirectKHR(VkCommandBuffer commandBuffer, VkStridedDeviceAddressRegionKHR const* pRaygenShaderBindingTable, VkStridedDeviceAddressRegionKHR const* pMissShaderBindingTable, VkStridedDeviceAddressRegionKHR const* pHitShaderBindingTable, VkStridedDeviceAddressRegionKHR const* pCallableShaderBindingTable, VkDeviceAddress indirectDeviceAddress)
    /// ```
    unsafe fn trace_rays_indirect(
        &self,
        raygen_shader_binding_table: &StridedDeviceAddressRegionKHR,
        miss_shader_binding_table: &StridedDeviceAddressRegionKHR,
        hit_shader_binding_table: &StridedDeviceAddressRegionKHR,
        callable_shader_binding_table: &StridedDeviceAddressRegionKHR,
        indirect_device_address: DeviceAddress,
    ) -> () {
        unsafe {
            self.commands().cmd_trace_rays_indirect(
                self.raw(),
                raygen_shader_binding_table,
                miss_shader_binding_table,
                hit_shader_binding_table,
                callable_shader_binding_table,
                indirect_device_address,
            )
        }
    }
    /// ```c
    /// void vkCmdTraceRaysKHR(VkCommandBuffer commandBuffer, VkStridedDeviceAddressRegionKHR const* pRaygenShaderBindingTable, VkStridedDeviceAddressRegionKHR const* pMissShaderBindingTable, VkStridedDeviceAddressRegionKHR const* pHitShaderBindingTable, VkStridedDeviceAddressRegionKHR const* pCallableShaderBindingTable, uint32_t width, uint32_t height, uint32_t depth)
    /// ```
    unsafe fn trace_rays(
        &self,
        raygen_shader_binding_table: &StridedDeviceAddressRegionKHR,
        miss_shader_binding_table: &StridedDeviceAddressRegionKHR,
        hit_shader_binding_table: &StridedDeviceAddressRegionKHR,
        callable_shader_binding_table: &StridedDeviceAddressRegionKHR,
        width: uint32_t,
        height: uint32_t,
        depth: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_trace_rays(
                self.raw(),
                raygen_shader_binding_table,
                miss_shader_binding_table,
                hit_shader_binding_table,
                callable_shader_binding_table,
                width,
                height,
                depth,
            )
        }
    }
}
