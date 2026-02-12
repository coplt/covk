// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_NV_device_generated_commands` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::nv::device_generated_commands::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::device_generated_commands::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdBindPipelineShaderGroupNV(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipeline pipeline, uint32_t groupIndex)
    /// ```
    pub unsafe fn cmd_bind_pipeline_shader_group(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: vk::Pipeline,
        group_index: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdBindPipelineShaderGroupNV(
                command_buffer.abi(), 
                pipeline_bind_point.abi(), 
                pipeline.abi(), 
                group_index.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdExecuteGeneratedCommandsNV(VkCommandBuffer commandBuffer, VkBool32 isPreprocessed, VkGeneratedCommandsInfoNV const* pGeneratedCommandsInfo)
    /// ```
    pub unsafe fn cmd_execute_generated_commands(
        &self,
        command_buffer: vk::CommandBuffer,
        is_preprocessed: bool,
        generated_commands_info: &GeneratedCommandsInfoNV,
    ) -> () {
        unsafe {
            self.0.CmdExecuteGeneratedCommandsNV(
                command_buffer.abi(), 
                is_preprocessed.abi(), 
                generated_commands_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdPreprocessGeneratedCommandsNV(VkCommandBuffer commandBuffer, VkGeneratedCommandsInfoNV const* pGeneratedCommandsInfo)
    /// ```
    pub unsafe fn cmd_preprocess_generated_commands(
        &self,
        command_buffer: vk::CommandBuffer,
        generated_commands_info: &GeneratedCommandsInfoNV,
    ) -> () {
        unsafe {
            self.0.CmdPreprocessGeneratedCommandsNV(
                command_buffer.abi(), 
                generated_commands_info.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkCreateIndirectCommandsLayoutNV(VkDevice device, VkIndirectCommandsLayoutCreateInfoNV const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkIndirectCommandsLayoutNV* pIndirectCommandsLayout)
    /// ```
    pub unsafe fn create_indirect_commands_layout(
        &self,
        device: vk::Device,
        create_info: &IndirectCommandsLayoutCreateInfoNV,
    ) -> crate::Result<vk::IndirectCommandsLayoutNV> {
        unsafe {
            let mut _v: Option<vk::IndirectCommandsLayoutNV> = Default::default();
            let _r = self.0.CreateIndirectCommandsLayoutNV(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDestroyIndirectCommandsLayoutNV(VkDevice device, VkIndirectCommandsLayoutNV indirectCommandsLayout, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_indirect_commands_layout(
        &self,
        device: vk::Device,
        indirect_commands_layout: Option<vk::IndirectCommandsLayoutNV>,
    ) -> () {
        unsafe {
            self.0.DestroyIndirectCommandsLayoutNV(
                device.abi(), 
                indirect_commands_layout.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkGetGeneratedCommandsMemoryRequirementsNV(VkDevice device, VkGeneratedCommandsMemoryRequirementsInfoNV const* pInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    pub unsafe fn get_generated_commands_memory_requirements(
        &self,
        device: vk::Device,
        info: &GeneratedCommandsMemoryRequirementsInfoNV,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self.0.GetGeneratedCommandsMemoryRequirementsNV(
                device.abi(), 
                info.abi(), 
                memory_requirements.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::nv::device_generated_commands {
    type Commands = Device;
}

/// Device object
pub trait NvDeviceGeneratedCommandsDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkCreateIndirectCommandsLayoutNV(VkDevice device, VkIndirectCommandsLayoutCreateInfoNV const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkIndirectCommandsLayoutNV* pIndirectCommandsLayout)
    /// ```
    unsafe fn create_indirect_commands_layout(
        &self,
        create_info: &IndirectCommandsLayoutCreateInfoNV,
    ) -> crate::Result<vk::IndirectCommandsLayoutNV> {
        unsafe {
            self.commands().create_indirect_commands_layout(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// void vkDestroyIndirectCommandsLayoutNV(VkDevice device, VkIndirectCommandsLayoutNV indirectCommandsLayout, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_indirect_commands_layout(
        &self,
        indirect_commands_layout: Option<vk::IndirectCommandsLayoutNV>,
    ) -> () {
        unsafe {
            self.commands().destroy_indirect_commands_layout(
                self.raw(),
                indirect_commands_layout,
            )
        }
    }
    /// ```c
    /// void vkGetGeneratedCommandsMemoryRequirementsNV(VkDevice device, VkGeneratedCommandsMemoryRequirementsInfoNV const* pInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    unsafe fn get_generated_commands_memory_requirements(
        &self,
        info: &GeneratedCommandsMemoryRequirementsInfoNV,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self.commands().get_generated_commands_memory_requirements(
                self.raw(),
                info,
                memory_requirements,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::nv::device_generated_commands {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::nv::device_generated_commands> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::nv::device_generated_commands {
        type Output = crate::hnd::Device<vk::extensions::nv::device_generated_commands>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::nv::device_generated_commands>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::nv::device_generated_commands> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::nv::device_generated_commands> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::nv::device_generated_commands> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::nv::device_generated_commands> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvDeviceGeneratedCommandsDevice for crate::hnd::Device<vk::extensions::nv::device_generated_commands> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::device_generated_commands, vk::Device> for crate::hnd::Device<vk::extensions::nv::device_generated_commands> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait NvDeviceGeneratedCommandsCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdBindPipelineShaderGroupNV(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipeline pipeline, uint32_t groupIndex)
    /// ```
    unsafe fn bind_pipeline_shader_group(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: vk::Pipeline,
        group_index: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_bind_pipeline_shader_group(
                self.raw(),
                pipeline_bind_point,
                pipeline,
                group_index,
            )
        }
    }
    /// ```c
    /// void vkCmdExecuteGeneratedCommandsNV(VkCommandBuffer commandBuffer, VkBool32 isPreprocessed, VkGeneratedCommandsInfoNV const* pGeneratedCommandsInfo)
    /// ```
    unsafe fn execute_generated_commands(
        &self,
        is_preprocessed: bool,
        generated_commands_info: &GeneratedCommandsInfoNV,
    ) -> () {
        unsafe {
            self.commands().cmd_execute_generated_commands(
                self.raw(),
                is_preprocessed,
                generated_commands_info,
            )
        }
    }
    /// ```c
    /// void vkCmdPreprocessGeneratedCommandsNV(VkCommandBuffer commandBuffer, VkGeneratedCommandsInfoNV const* pGeneratedCommandsInfo)
    /// ```
    unsafe fn preprocess_generated_commands(
        &self,
        generated_commands_info: &GeneratedCommandsInfoNV,
    ) -> () {
        unsafe {
            self.commands().cmd_preprocess_generated_commands(
                self.raw(),
                generated_commands_info,
            )
        }
    }
}

impl<O: crate::HndCtx<vk::extensions::nv::device_generated_commands, vk::Device>> crate::Owner<vk::IndirectCommandsLayoutNV, vk::extensions::nv::device_generated_commands> for O {
    fn drop(&mut self, raw: vk::IndirectCommandsLayoutNV) {
        unsafe {
            self.commands().destroy_indirect_commands_layout(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::nv::device_generated_commands, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::IndirectCommandsLayoutNV, O, vk::extensions::nv::device_generated_commands>>
    where O: crate::HndCtx<vk::extensions::nv::device_generated_commands, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::IndirectCommandsLayoutNV> for vk::extensions::nv::device_generated_commands {
    type Impl = _hs_IndirectCommandsLayoutNV::IndirectCommandsLayoutNV;
}


mod _hs_IndirectCommandsLayoutNV {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct IndirectCommandsLayoutNV(pub(crate) crate::handle::Hnd<vk::IndirectCommandsLayoutNV, ::alloc::sync::Arc<super::Device>>);

    impl Clone for IndirectCommandsLayoutNV {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::IndirectCommandsLayoutNV, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::extensions::nv::device_generated_commands, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::IndirectCommandsLayoutNV, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_indirect_commands_layout(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::IndirectCommandsLayoutNV<vk::extensions::nv::device_generated_commands>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::nv::device_generated_commands, vk::Device>, raw: vk::IndirectCommandsLayoutNV) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::nv::device_generated_commands, vk::Device>, raw: vk::IndirectCommandsLayoutNV, dep: impl FnOnce() -> Dep) -> Self {
            Self(IndirectCommandsLayoutNV(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::IndirectCommandsLayoutNV<vk::extensions::nv::device_generated_commands> {
        pub fn raw(&self) -> vk::IndirectCommandsLayoutNV { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::IndirectCommandsLayoutNV<vk::extensions::nv::device_generated_commands> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("IndirectCommandsLayoutNV({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::IndirectCommandsLayoutNV<vk::extensions::nv::device_generated_commands> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::nv::device_generated_commands> for vk::IndirectCommandsLayoutNV
        where Ctx: crate::HndCtx<vk::extensions::nv::device_generated_commands, vk::Device>,
    {
        type Output = crate::hnd::IndirectCommandsLayoutNV<vk::extensions::nv::device_generated_commands>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::IndirectCommandsLayoutNV::<vk::extensions::nv::device_generated_commands>::new_with(ctx, self, dep) }
        }
    }
}
