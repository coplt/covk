// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_AMDX_shader_enqueue` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::amdx::shader_enqueue::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::amdx::shader_enqueue::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdDispatchGraphAMDX(VkCommandBuffer commandBuffer, VkDeviceAddress scratch, VkDeviceSize scratchSize, VkDispatchGraphCountInfoAMDX const* pCountInfo)
    /// ```
    pub unsafe fn cmd_dispatch_graph(
        &self,
        command_buffer: vk::CommandBuffer,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: &DispatchGraphCountInfoAMDX,
    ) -> () {
        unsafe {
            self.0.CmdDispatchGraphAMDX(
                command_buffer.abi(), 
                scratch.abi(), 
                scratch_size.abi(), 
                count_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdDispatchGraphIndirectAMDX(VkCommandBuffer commandBuffer, VkDeviceAddress scratch, VkDeviceSize scratchSize, VkDispatchGraphCountInfoAMDX const* pCountInfo)
    /// ```
    pub unsafe fn cmd_dispatch_graph_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: &DispatchGraphCountInfoAMDX,
    ) -> () {
        unsafe {
            self.0.CmdDispatchGraphIndirectAMDX(
                command_buffer.abi(), 
                scratch.abi(), 
                scratch_size.abi(), 
                count_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdDispatchGraphIndirectCountAMDX(VkCommandBuffer commandBuffer, VkDeviceAddress scratch, VkDeviceSize scratchSize, VkDeviceAddress countInfo)
    /// ```
    pub unsafe fn cmd_dispatch_graph_indirect_count(
        &self,
        command_buffer: vk::CommandBuffer,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: DeviceAddress,
    ) -> () {
        unsafe {
            self.0.CmdDispatchGraphIndirectCountAMDX(
                command_buffer.abi(), 
                scratch.abi(), 
                scratch_size.abi(), 
                count_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdInitializeGraphScratchMemoryAMDX(VkCommandBuffer commandBuffer, VkPipeline executionGraph, VkDeviceAddress scratch, VkDeviceSize scratchSize)
    /// ```
    pub unsafe fn cmd_initialize_graph_scratch_memory(
        &self,
        command_buffer: vk::CommandBuffer,
        execution_graph: vk::Pipeline,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
    ) -> () {
        unsafe {
            self.0.CmdInitializeGraphScratchMemoryAMDX(
                command_buffer.abi(), 
                execution_graph.abi(), 
                scratch.abi(), 
                scratch_size.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkCreateExecutionGraphPipelinesAMDX(VkDevice device, VkPipelineCache pipelineCache, uint32_t createInfoCount, VkExecutionGraphPipelineCreateInfoAMDX const* pCreateInfos, VkAllocationCallbacks const* pAllocator, VkPipeline* pPipelines)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::PipelineCompileRequiredExt]
    pub unsafe fn create_execution_graph_pipelines(
        &self,
        device: vk::Device,
        pipeline_cache: Option<vk::PipelineCache>,
        create_infos: &[ExecutionGraphPipelineCreateInfoAMDX],
        pipelines: &mut [Option<vk::Pipeline>],
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.CreateExecutionGraphPipelinesAMDX(
                device.abi(), 
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
    /// VkResult vkGetExecutionGraphPipelineNodeIndexAMDX(VkDevice device, VkPipeline executionGraph, VkPipelineShaderStageNodeCreateInfoAMDX const* pNodeInfo, uint32_t* pNodeIndex)
    /// ```
    pub unsafe fn get_execution_graph_pipeline_node_index(
        &self,
        device: vk::Device,
        execution_graph: vk::Pipeline,
        node_info: &PipelineShaderStageNodeCreateInfoAMDX,
    ) -> crate::Result<uint32_t> {
        unsafe {
            let mut _v: uint32_t = Default::default();
            let _r = self.0.GetExecutionGraphPipelineNodeIndexAMDX(
                device.abi(), 
                execution_graph.abi(), 
                node_info.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
    /// ```c
    /// VkResult vkGetExecutionGraphPipelineScratchSizeAMDX(VkDevice device, VkPipeline executionGraph, VkExecutionGraphPipelineScratchSizeAMDX* pSizeInfo)
    /// ```
    pub unsafe fn get_execution_graph_pipeline_scratch_size(
        &self,
        device: vk::Device,
        execution_graph: vk::Pipeline,
        size_info: &mut ExecutionGraphPipelineScratchSizeAMDX,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetExecutionGraphPipelineScratchSizeAMDX(
                device.abi(), 
                execution_graph.abi(), 
                size_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::amdx::shader_enqueue {
    type Commands = Device;
}

/// Device object
pub trait AmdxShaderEnqueueDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkCreateExecutionGraphPipelinesAMDX(VkDevice device, VkPipelineCache pipelineCache, uint32_t createInfoCount, VkExecutionGraphPipelineCreateInfoAMDX const* pCreateInfos, VkAllocationCallbacks const* pAllocator, VkPipeline* pPipelines)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::PipelineCompileRequiredExt]
    unsafe fn create_execution_graph_pipelines(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        create_infos: &[ExecutionGraphPipelineCreateInfoAMDX],
        pipelines: &mut [Option<vk::Pipeline>],
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().create_execution_graph_pipelines(
                self.raw(),
                pipeline_cache,
                create_infos,
                pipelines,
            )
        }
    }
    /// ```c
    /// VkResult vkGetExecutionGraphPipelineNodeIndexAMDX(VkDevice device, VkPipeline executionGraph, VkPipelineShaderStageNodeCreateInfoAMDX const* pNodeInfo, uint32_t* pNodeIndex)
    /// ```
    unsafe fn get_execution_graph_pipeline_node_index(
        &self,
        execution_graph: vk::Pipeline,
        node_info: &PipelineShaderStageNodeCreateInfoAMDX,
    ) -> crate::Result<uint32_t> {
        unsafe {
            self.commands().get_execution_graph_pipeline_node_index(
                self.raw(),
                execution_graph,
                node_info,
            )
        }
    }
    /// ```c
    /// VkResult vkGetExecutionGraphPipelineScratchSizeAMDX(VkDevice device, VkPipeline executionGraph, VkExecutionGraphPipelineScratchSizeAMDX* pSizeInfo)
    /// ```
    unsafe fn get_execution_graph_pipeline_scratch_size(
        &self,
        execution_graph: vk::Pipeline,
        size_info: &mut ExecutionGraphPipelineScratchSizeAMDX,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_execution_graph_pipeline_scratch_size(
                self.raw(),
                execution_graph,
                size_info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::amdx::shader_enqueue {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::amdx::shader_enqueue> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::amdx::shader_enqueue {
        type Output = crate::hnd::Device<vk::extensions::amdx::shader_enqueue>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::amdx::shader_enqueue>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::amdx::shader_enqueue> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::amdx::shader_enqueue> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::amdx::shader_enqueue> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::amdx::shader_enqueue> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::AmdxShaderEnqueueDevice for crate::hnd::Device<vk::extensions::amdx::shader_enqueue> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::amdx::shader_enqueue, vk::Device> for crate::hnd::Device<vk::extensions::amdx::shader_enqueue> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait AmdxShaderEnqueueCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdDispatchGraphAMDX(VkCommandBuffer commandBuffer, VkDeviceAddress scratch, VkDeviceSize scratchSize, VkDispatchGraphCountInfoAMDX const* pCountInfo)
    /// ```
    unsafe fn dispatch_graph(
        &self,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: &DispatchGraphCountInfoAMDX,
    ) -> () {
        unsafe {
            self.commands().cmd_dispatch_graph(
                self.raw(),
                scratch,
                scratch_size,
                count_info,
            )
        }
    }
    /// ```c
    /// void vkCmdDispatchGraphIndirectAMDX(VkCommandBuffer commandBuffer, VkDeviceAddress scratch, VkDeviceSize scratchSize, VkDispatchGraphCountInfoAMDX const* pCountInfo)
    /// ```
    unsafe fn dispatch_graph_indirect(
        &self,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: &DispatchGraphCountInfoAMDX,
    ) -> () {
        unsafe {
            self.commands().cmd_dispatch_graph_indirect(
                self.raw(),
                scratch,
                scratch_size,
                count_info,
            )
        }
    }
    /// ```c
    /// void vkCmdDispatchGraphIndirectCountAMDX(VkCommandBuffer commandBuffer, VkDeviceAddress scratch, VkDeviceSize scratchSize, VkDeviceAddress countInfo)
    /// ```
    unsafe fn dispatch_graph_indirect_count(
        &self,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: DeviceAddress,
    ) -> () {
        unsafe {
            self.commands().cmd_dispatch_graph_indirect_count(
                self.raw(),
                scratch,
                scratch_size,
                count_info,
            )
        }
    }
    /// ```c
    /// void vkCmdInitializeGraphScratchMemoryAMDX(VkCommandBuffer commandBuffer, VkPipeline executionGraph, VkDeviceAddress scratch, VkDeviceSize scratchSize)
    /// ```
    unsafe fn initialize_graph_scratch_memory(
        &self,
        execution_graph: vk::Pipeline,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
    ) -> () {
        unsafe {
            self.commands().cmd_initialize_graph_scratch_memory(
                self.raw(),
                execution_graph,
                scratch,
                scratch_size,
            )
        }
    }
}
