// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_NV_ray_tracing` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::nv::ray_tracing::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::ray_tracing::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkBindAccelerationStructureMemoryNV(VkDevice device, uint32_t bindInfoCount, VkBindAccelerationStructureMemoryInfoNV const* pBindInfos)
    /// ```
    pub unsafe fn bind_acceleration_structure_memory(
        &self,
        device: vk::Device,
        bind_infos: &[BindAccelerationStructureMemoryInfoNV],
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.BindAccelerationStructureMemoryNV(
                device.abi(), 
                bind_infos.len() as _, 
                bind_infos.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// void vkCmdBuildAccelerationStructureNV(VkCommandBuffer commandBuffer, VkAccelerationStructureInfoNV const* pInfo, VkBuffer instanceData, VkDeviceSize instanceOffset, VkBool32 update, VkAccelerationStructureNV dst, VkAccelerationStructureNV src, VkBuffer scratch, VkDeviceSize scratchOffset)
    /// ```
    pub unsafe fn cmd_build_acceleration_structure(
        &self,
        command_buffer: vk::CommandBuffer,
        info: &AccelerationStructureInfoNV,
        instance_data: Option<vk::Buffer>,
        instance_offset: DeviceSize,
        update: bool,
        dst: vk::AccelerationStructureNV,
        src: Option<vk::AccelerationStructureNV>,
        scratch: vk::Buffer,
        scratch_offset: DeviceSize,
    ) -> () {
        unsafe {
            self.0.CmdBuildAccelerationStructureNV(
                command_buffer.abi(), 
                info.abi(), 
                instance_data.abi(), 
                instance_offset.abi(), 
                update.abi(), 
                dst.abi(), 
                src.abi(), 
                scratch.abi(), 
                scratch_offset.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdCopyAccelerationStructureNV(VkCommandBuffer commandBuffer, VkAccelerationStructureNV dst, VkAccelerationStructureNV src, VkCopyAccelerationStructureModeKHR mode)
    /// ```
    pub unsafe fn cmd_copy_acceleration_structure(
        &self,
        command_buffer: vk::CommandBuffer,
        dst: vk::AccelerationStructureNV,
        src: vk::AccelerationStructureNV,
        mode: CopyAccelerationStructureModeKHR,
    ) -> () {
        unsafe {
            self.0.CmdCopyAccelerationStructureNV(
                command_buffer.abi(), 
                dst.abi(), 
                src.abi(), 
                mode.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdTraceRaysNV(VkCommandBuffer commandBuffer, VkBuffer raygenShaderBindingTableBuffer, VkDeviceSize raygenShaderBindingOffset, VkBuffer missShaderBindingTableBuffer, VkDeviceSize missShaderBindingOffset, VkDeviceSize missShaderBindingStride, VkBuffer hitShaderBindingTableBuffer, VkDeviceSize hitShaderBindingOffset, VkDeviceSize hitShaderBindingStride, VkBuffer callableShaderBindingTableBuffer, VkDeviceSize callableShaderBindingOffset, VkDeviceSize callableShaderBindingStride, uint32_t width, uint32_t height, uint32_t depth)
    /// ```
    pub unsafe fn cmd_trace_rays(
        &self,
        command_buffer: vk::CommandBuffer,
        raygen_shader_binding_table_buffer: vk::Buffer,
        raygen_shader_binding_offset: DeviceSize,
        miss_shader_binding_table_buffer: Option<vk::Buffer>,
        miss_shader_binding_offset: DeviceSize,
        miss_shader_binding_stride: DeviceSize,
        hit_shader_binding_table_buffer: Option<vk::Buffer>,
        hit_shader_binding_offset: DeviceSize,
        hit_shader_binding_stride: DeviceSize,
        callable_shader_binding_table_buffer: Option<vk::Buffer>,
        callable_shader_binding_offset: DeviceSize,
        callable_shader_binding_stride: DeviceSize,
        width: uint32_t,
        height: uint32_t,
        depth: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdTraceRaysNV(
                command_buffer.abi(), 
                raygen_shader_binding_table_buffer.abi(), 
                raygen_shader_binding_offset.abi(), 
                miss_shader_binding_table_buffer.abi(), 
                miss_shader_binding_offset.abi(), 
                miss_shader_binding_stride.abi(), 
                hit_shader_binding_table_buffer.abi(), 
                hit_shader_binding_offset.abi(), 
                hit_shader_binding_stride.abi(), 
                callable_shader_binding_table_buffer.abi(), 
                callable_shader_binding_offset.abi(), 
                callable_shader_binding_stride.abi(), 
                width.abi(), 
                height.abi(), 
                depth.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdWriteAccelerationStructuresPropertiesNV(VkCommandBuffer commandBuffer, uint32_t accelerationStructureCount, VkAccelerationStructureNV const* pAccelerationStructures, VkQueryType queryType, VkQueryPool queryPool, uint32_t firstQuery)
    /// ```
    pub unsafe fn cmd_write_acceleration_structures_properties(
        &self,
        command_buffer: vk::CommandBuffer,
        acceleration_structures: &[vk::AccelerationStructureNV],
        query_type: QueryType,
        query_pool: vk::QueryPool,
        first_query: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdWriteAccelerationStructuresPropertiesNV(
                command_buffer.abi(), 
                acceleration_structures.len() as _, 
                acceleration_structures.abi(), 
                query_type.abi(), 
                query_pool.abi(), 
                first_query.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkCompileDeferredNV(VkDevice device, VkPipeline pipeline, uint32_t shader)
    /// ```
    pub unsafe fn compile_deferred(
        &self,
        device: vk::Device,
        pipeline: vk::Pipeline,
        shader: uint32_t,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.CompileDeferredNV(
                device.abi(), 
                pipeline.abi(), 
                shader.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkCreateAccelerationStructureNV(VkDevice device, VkAccelerationStructureCreateInfoNV const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkAccelerationStructureNV* pAccelerationStructure)
    /// ```
    pub unsafe fn create_acceleration_structure(
        &self,
        device: vk::Device,
        create_info: &AccelerationStructureCreateInfoNV,
    ) -> crate::Result<vk::AccelerationStructureNV> {
        unsafe {
            let mut _v: Option<vk::AccelerationStructureNV> = Default::default();
            let _r = self.0.CreateAccelerationStructureNV(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateRayTracingPipelinesNV(VkDevice device, VkPipelineCache pipelineCache, uint32_t createInfoCount, VkRayTracingPipelineCreateInfoNV const* pCreateInfos, VkAllocationCallbacks const* pAllocator, VkPipeline* pPipelines)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::PipelineCompileRequiredExt]
    pub unsafe fn create_ray_tracing_pipelines(
        &self,
        device: vk::Device,
        pipeline_cache: Option<vk::PipelineCache>,
        create_infos: &[RayTracingPipelineCreateInfoNV],
        pipelines: &mut [Option<vk::Pipeline>],
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.CreateRayTracingPipelinesNV(
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
    /// void vkDestroyAccelerationStructureNV(VkDevice device, VkAccelerationStructureNV accelerationStructure, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_acceleration_structure(
        &self,
        device: vk::Device,
        acceleration_structure: Option<vk::AccelerationStructureNV>,
    ) -> () {
        unsafe {
            self.0.DestroyAccelerationStructureNV(
                device.abi(), 
                acceleration_structure.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// VkResult vkGetAccelerationStructureHandleNV(VkDevice device, VkAccelerationStructureNV accelerationStructure, size_t dataSize, void* pData)
    /// ```
    pub unsafe fn get_acceleration_structure_handle(
        &self,
        device: vk::Device,
        acceleration_structure: vk::AccelerationStructureNV,
        data_size: size_t,
        data: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetAccelerationStructureHandleNV(
                device.abi(), 
                acceleration_structure.abi(), 
                data_size.abi(), 
                data.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// void vkGetAccelerationStructureMemoryRequirementsNV(VkDevice device, VkAccelerationStructureMemoryRequirementsInfoNV const* pInfo, VkMemoryRequirements2KHR* pMemoryRequirements)
    /// ```
    pub unsafe fn get_acceleration_structure_memory_requirements(
        &self,
        device: vk::Device,
        info: &AccelerationStructureMemoryRequirementsInfoNV,
    ) -> MemoryRequirements2KHR {
        unsafe {
            let mut _v: MemoryRequirements2KHR = Default::default();
            self.0.GetAccelerationStructureMemoryRequirementsNV(
                device.abi(), 
                info.abi(), 
                (&mut _v).abi(), 
            );
            _v
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
            let _r = self.0.GetRayTracingShaderGroupHandlesNV(
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
}

impl crate::CommandScope<vk::Device> for vk::extensions::nv::ray_tracing {
    type Commands = Device;
}

/// Device object
pub trait NvRayTracingDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkBindAccelerationStructureMemoryNV(VkDevice device, uint32_t bindInfoCount, VkBindAccelerationStructureMemoryInfoNV const* pBindInfos)
    /// ```
    unsafe fn bind_acceleration_structure_memory(
        &self,
        bind_infos: &[BindAccelerationStructureMemoryInfoNV],
    ) -> crate::Result<()> {
        unsafe {
            self.commands().bind_acceleration_structure_memory(
                self.raw(),
                bind_infos,
            )
        }
    }
    /// ```c
    /// VkResult vkCompileDeferredNV(VkDevice device, VkPipeline pipeline, uint32_t shader)
    /// ```
    unsafe fn compile_deferred(
        &self,
        pipeline: vk::Pipeline,
        shader: uint32_t,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().compile_deferred(
                self.raw(),
                pipeline,
                shader,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateAccelerationStructureNV(VkDevice device, VkAccelerationStructureCreateInfoNV const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkAccelerationStructureNV* pAccelerationStructure)
    /// ```
    unsafe fn create_acceleration_structure(
        &self,
        create_info: &AccelerationStructureCreateInfoNV,
    ) -> crate::Result<vk::AccelerationStructureNV> {
        unsafe {
            self.commands().create_acceleration_structure(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateRayTracingPipelinesNV(VkDevice device, VkPipelineCache pipelineCache, uint32_t createInfoCount, VkRayTracingPipelineCreateInfoNV const* pCreateInfos, VkAllocationCallbacks const* pAllocator, VkPipeline* pPipelines)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::PipelineCompileRequiredExt]
    unsafe fn create_ray_tracing_pipelines(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        create_infos: &[RayTracingPipelineCreateInfoNV],
        pipelines: &mut [Option<vk::Pipeline>],
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().create_ray_tracing_pipelines(
                self.raw(),
                pipeline_cache,
                create_infos,
                pipelines,
            )
        }
    }
    /// ```c
    /// void vkDestroyAccelerationStructureNV(VkDevice device, VkAccelerationStructureNV accelerationStructure, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_acceleration_structure(
        &self,
        acceleration_structure: Option<vk::AccelerationStructureNV>,
    ) -> () {
        unsafe {
            self.commands().destroy_acceleration_structure(
                self.raw(),
                acceleration_structure,
            )
        }
    }
    /// ```c
    /// VkResult vkGetAccelerationStructureHandleNV(VkDevice device, VkAccelerationStructureNV accelerationStructure, size_t dataSize, void* pData)
    /// ```
    unsafe fn get_acceleration_structure_handle(
        &self,
        acceleration_structure: vk::AccelerationStructureNV,
        data_size: size_t,
        data: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_acceleration_structure_handle(
                self.raw(),
                acceleration_structure,
                data_size,
                data,
            )
        }
    }
    /// ```c
    /// void vkGetAccelerationStructureMemoryRequirementsNV(VkDevice device, VkAccelerationStructureMemoryRequirementsInfoNV const* pInfo, VkMemoryRequirements2KHR* pMemoryRequirements)
    /// ```
    unsafe fn get_acceleration_structure_memory_requirements(
        &self,
        info: &AccelerationStructureMemoryRequirementsInfoNV,
    ) -> MemoryRequirements2KHR {
        unsafe {
            self.commands().get_acceleration_structure_memory_requirements(
                self.raw(),
                info,
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
}

impl crate::HndScope<vk::Device> for vk::extensions::nv::ray_tracing {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::nv::ray_tracing> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::nv::ray_tracing {
        type Output = crate::hnd::Device<vk::extensions::nv::ray_tracing>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::nv::ray_tracing>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::nv::ray_tracing> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::nv::ray_tracing> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::nv::ray_tracing> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::nv::ray_tracing> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvRayTracingDevice for crate::hnd::Device<vk::extensions::nv::ray_tracing> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::ray_tracing, vk::Device> for crate::hnd::Device<vk::extensions::nv::ray_tracing> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait NvRayTracingCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdBuildAccelerationStructureNV(VkCommandBuffer commandBuffer, VkAccelerationStructureInfoNV const* pInfo, VkBuffer instanceData, VkDeviceSize instanceOffset, VkBool32 update, VkAccelerationStructureNV dst, VkAccelerationStructureNV src, VkBuffer scratch, VkDeviceSize scratchOffset)
    /// ```
    unsafe fn build_acceleration_structure(
        &self,
        info: &AccelerationStructureInfoNV,
        instance_data: Option<vk::Buffer>,
        instance_offset: DeviceSize,
        update: bool,
        dst: vk::AccelerationStructureNV,
        src: Option<vk::AccelerationStructureNV>,
        scratch: vk::Buffer,
        scratch_offset: DeviceSize,
    ) -> () {
        unsafe {
            self.commands().cmd_build_acceleration_structure(
                self.raw(),
                info,
                instance_data,
                instance_offset,
                update,
                dst,
                src,
                scratch,
                scratch_offset,
            )
        }
    }
    /// ```c
    /// void vkCmdCopyAccelerationStructureNV(VkCommandBuffer commandBuffer, VkAccelerationStructureNV dst, VkAccelerationStructureNV src, VkCopyAccelerationStructureModeKHR mode)
    /// ```
    unsafe fn copy_acceleration_structure(
        &self,
        dst: vk::AccelerationStructureNV,
        src: vk::AccelerationStructureNV,
        mode: CopyAccelerationStructureModeKHR,
    ) -> () {
        unsafe {
            self.commands().cmd_copy_acceleration_structure(
                self.raw(),
                dst,
                src,
                mode,
            )
        }
    }
    /// ```c
    /// void vkCmdTraceRaysNV(VkCommandBuffer commandBuffer, VkBuffer raygenShaderBindingTableBuffer, VkDeviceSize raygenShaderBindingOffset, VkBuffer missShaderBindingTableBuffer, VkDeviceSize missShaderBindingOffset, VkDeviceSize missShaderBindingStride, VkBuffer hitShaderBindingTableBuffer, VkDeviceSize hitShaderBindingOffset, VkDeviceSize hitShaderBindingStride, VkBuffer callableShaderBindingTableBuffer, VkDeviceSize callableShaderBindingOffset, VkDeviceSize callableShaderBindingStride, uint32_t width, uint32_t height, uint32_t depth)
    /// ```
    unsafe fn trace_rays(
        &self,
        raygen_shader_binding_table_buffer: vk::Buffer,
        raygen_shader_binding_offset: DeviceSize,
        miss_shader_binding_table_buffer: Option<vk::Buffer>,
        miss_shader_binding_offset: DeviceSize,
        miss_shader_binding_stride: DeviceSize,
        hit_shader_binding_table_buffer: Option<vk::Buffer>,
        hit_shader_binding_offset: DeviceSize,
        hit_shader_binding_stride: DeviceSize,
        callable_shader_binding_table_buffer: Option<vk::Buffer>,
        callable_shader_binding_offset: DeviceSize,
        callable_shader_binding_stride: DeviceSize,
        width: uint32_t,
        height: uint32_t,
        depth: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_trace_rays(
                self.raw(),
                raygen_shader_binding_table_buffer,
                raygen_shader_binding_offset,
                miss_shader_binding_table_buffer,
                miss_shader_binding_offset,
                miss_shader_binding_stride,
                hit_shader_binding_table_buffer,
                hit_shader_binding_offset,
                hit_shader_binding_stride,
                callable_shader_binding_table_buffer,
                callable_shader_binding_offset,
                callable_shader_binding_stride,
                width,
                height,
                depth,
            )
        }
    }
    /// ```c
    /// void vkCmdWriteAccelerationStructuresPropertiesNV(VkCommandBuffer commandBuffer, uint32_t accelerationStructureCount, VkAccelerationStructureNV const* pAccelerationStructures, VkQueryType queryType, VkQueryPool queryPool, uint32_t firstQuery)
    /// ```
    unsafe fn write_acceleration_structures_properties(
        &self,
        acceleration_structures: &[vk::AccelerationStructureNV],
        query_type: QueryType,
        query_pool: vk::QueryPool,
        first_query: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_write_acceleration_structures_properties(
                self.raw(),
                acceleration_structures,
                query_type,
                query_pool,
                first_query,
            )
        }
    }
}

impl<O: crate::HndCtx<vk::extensions::nv::ray_tracing, vk::Device>> crate::Owner<vk::AccelerationStructureNV, vk::extensions::nv::ray_tracing> for O {
    fn drop(&mut self, raw: vk::AccelerationStructureNV) {
        unsafe {
            self.commands().destroy_acceleration_structure(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::nv::ray_tracing, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::AccelerationStructureNV, O, vk::extensions::nv::ray_tracing>>
    where O: crate::HndCtx<vk::extensions::nv::ray_tracing, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::AccelerationStructureNV> for vk::extensions::nv::ray_tracing {
    type Impl = _hs_AccelerationStructureNV::AccelerationStructureNV;
}


mod _hs_AccelerationStructureNV {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct AccelerationStructureNV(pub(crate) crate::handle::Hnd<vk::AccelerationStructureNV, ::alloc::sync::Arc<super::Device>>);

    impl Clone for AccelerationStructureNV {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::AccelerationStructureNV, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::extensions::nv::ray_tracing, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::AccelerationStructureNV, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_acceleration_structure(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::AccelerationStructureNV<vk::extensions::nv::ray_tracing>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::nv::ray_tracing, vk::Device>, raw: vk::AccelerationStructureNV) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::nv::ray_tracing, vk::Device>, raw: vk::AccelerationStructureNV, dep: impl FnOnce() -> Dep) -> Self {
            Self(AccelerationStructureNV(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::AccelerationStructureNV<vk::extensions::nv::ray_tracing> {
        pub fn raw(&self) -> vk::AccelerationStructureNV { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::AccelerationStructureNV<vk::extensions::nv::ray_tracing> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("AccelerationStructureNV({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::AccelerationStructureNV<vk::extensions::nv::ray_tracing> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::nv::ray_tracing> for vk::AccelerationStructureNV
        where Ctx: crate::HndCtx<vk::extensions::nv::ray_tracing, vk::Device>,
    {
        type Output = crate::hnd::AccelerationStructureNV<vk::extensions::nv::ray_tracing>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::AccelerationStructureNV::<vk::extensions::nv::ray_tracing>::new_with(ctx, self, dep) }
        }
    }
}
