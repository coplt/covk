// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_device_generated_commands` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::device_generated_commands::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::device_generated_commands::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdExecuteGeneratedCommandsEXT(VkCommandBuffer commandBuffer, VkBool32 isPreprocessed, VkGeneratedCommandsInfoEXT const* pGeneratedCommandsInfo)
    /// ```
    pub unsafe fn cmd_execute_generated_commands(
        &self,
        command_buffer: vk::CommandBuffer,
        is_preprocessed: bool,
        generated_commands_info: &GeneratedCommandsInfoEXT,
    ) -> () {
        unsafe {
            self.0.CmdExecuteGeneratedCommandsEXT(
                command_buffer.abi(), 
                is_preprocessed.abi(), 
                generated_commands_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdPreprocessGeneratedCommandsEXT(VkCommandBuffer commandBuffer, VkGeneratedCommandsInfoEXT const* pGeneratedCommandsInfo, VkCommandBuffer stateCommandBuffer)
    /// ```
    pub unsafe fn cmd_preprocess_generated_commands(
        &self,
        command_buffer: vk::CommandBuffer,
        generated_commands_info: &GeneratedCommandsInfoEXT,
        state_command_buffer: vk::CommandBuffer,
    ) -> () {
        unsafe {
            self.0.CmdPreprocessGeneratedCommandsEXT(
                command_buffer.abi(), 
                generated_commands_info.abi(), 
                state_command_buffer.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkCreateIndirectCommandsLayoutEXT(VkDevice device, VkIndirectCommandsLayoutCreateInfoEXT const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkIndirectCommandsLayoutEXT* pIndirectCommandsLayout)
    /// ```
    pub unsafe fn create_indirect_commands_layout(
        &self,
        device: vk::Device,
        create_info: &IndirectCommandsLayoutCreateInfoEXT,
    ) -> crate::Result<vk::IndirectCommandsLayoutEXT> {
        unsafe {
            let mut _v: Option<vk::IndirectCommandsLayoutEXT> = Default::default();
            let _r = self.0.CreateIndirectCommandsLayoutEXT(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateIndirectExecutionSetEXT(VkDevice device, VkIndirectExecutionSetCreateInfoEXT const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkIndirectExecutionSetEXT* pIndirectExecutionSet)
    /// ```
    pub unsafe fn create_indirect_execution_set(
        &self,
        device: vk::Device,
        create_info: &IndirectExecutionSetCreateInfoEXT,
    ) -> crate::Result<vk::IndirectExecutionSetEXT> {
        unsafe {
            let mut _v: Option<vk::IndirectExecutionSetEXT> = Default::default();
            let _r = self.0.CreateIndirectExecutionSetEXT(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDestroyIndirectCommandsLayoutEXT(VkDevice device, VkIndirectCommandsLayoutEXT indirectCommandsLayout, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_indirect_commands_layout(
        &self,
        device: vk::Device,
        indirect_commands_layout: Option<vk::IndirectCommandsLayoutEXT>,
    ) -> () {
        unsafe {
            self.0.DestroyIndirectCommandsLayoutEXT(
                device.abi(), 
                indirect_commands_layout.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroyIndirectExecutionSetEXT(VkDevice device, VkIndirectExecutionSetEXT indirectExecutionSet, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_indirect_execution_set(
        &self,
        device: vk::Device,
        indirect_execution_set: Option<vk::IndirectExecutionSetEXT>,
    ) -> () {
        unsafe {
            self.0.DestroyIndirectExecutionSetEXT(
                device.abi(), 
                indirect_execution_set.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkGetGeneratedCommandsMemoryRequirementsEXT(VkDevice device, VkGeneratedCommandsMemoryRequirementsInfoEXT const* pInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    pub unsafe fn get_generated_commands_memory_requirements(
        &self,
        device: vk::Device,
        info: &GeneratedCommandsMemoryRequirementsInfoEXT,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self.0.GetGeneratedCommandsMemoryRequirementsEXT(
                device.abi(), 
                info.abi(), 
                memory_requirements.abi(), 
            );
        }
    }
    /// ```c
    /// void vkUpdateIndirectExecutionSetPipelineEXT(VkDevice device, VkIndirectExecutionSetEXT indirectExecutionSet, uint32_t executionSetWriteCount, VkWriteIndirectExecutionSetPipelineEXT const* pExecutionSetWrites)
    /// ```
    pub unsafe fn update_indirect_execution_set_pipeline(
        &self,
        device: vk::Device,
        indirect_execution_set: vk::IndirectExecutionSetEXT,
        execution_set_writes: &[WriteIndirectExecutionSetPipelineEXT],
    ) -> () {
        unsafe {
            self.0.UpdateIndirectExecutionSetPipelineEXT(
                device.abi(), 
                indirect_execution_set.abi(), 
                execution_set_writes.len() as _, 
                execution_set_writes.abi(), 
            );
        }
    }
    /// ```c
    /// void vkUpdateIndirectExecutionSetShaderEXT(VkDevice device, VkIndirectExecutionSetEXT indirectExecutionSet, uint32_t executionSetWriteCount, VkWriteIndirectExecutionSetShaderEXT const* pExecutionSetWrites)
    /// ```
    pub unsafe fn update_indirect_execution_set_shader(
        &self,
        device: vk::Device,
        indirect_execution_set: vk::IndirectExecutionSetEXT,
        execution_set_writes: &[WriteIndirectExecutionSetShaderEXT],
    ) -> () {
        unsafe {
            self.0.UpdateIndirectExecutionSetShaderEXT(
                device.abi(), 
                indirect_execution_set.abi(), 
                execution_set_writes.len() as _, 
                execution_set_writes.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::device_generated_commands {
    type Commands = Device;
}

/// Device object
pub trait ExtDeviceGeneratedCommandsDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkCreateIndirectCommandsLayoutEXT(VkDevice device, VkIndirectCommandsLayoutCreateInfoEXT const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkIndirectCommandsLayoutEXT* pIndirectCommandsLayout)
    /// ```
    unsafe fn create_indirect_commands_layout(
        &self,
        create_info: &IndirectCommandsLayoutCreateInfoEXT,
    ) -> crate::Result<vk::IndirectCommandsLayoutEXT> {
        unsafe {
            self.commands().create_indirect_commands_layout(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateIndirectExecutionSetEXT(VkDevice device, VkIndirectExecutionSetCreateInfoEXT const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkIndirectExecutionSetEXT* pIndirectExecutionSet)
    /// ```
    unsafe fn create_indirect_execution_set(
        &self,
        create_info: &IndirectExecutionSetCreateInfoEXT,
    ) -> crate::Result<vk::IndirectExecutionSetEXT> {
        unsafe {
            self.commands().create_indirect_execution_set(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// void vkDestroyIndirectCommandsLayoutEXT(VkDevice device, VkIndirectCommandsLayoutEXT indirectCommandsLayout, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_indirect_commands_layout(
        &self,
        indirect_commands_layout: Option<vk::IndirectCommandsLayoutEXT>,
    ) -> () {
        unsafe {
            self.commands().destroy_indirect_commands_layout(
                self.raw(),
                indirect_commands_layout,
            )
        }
    }
    /// ```c
    /// void vkDestroyIndirectExecutionSetEXT(VkDevice device, VkIndirectExecutionSetEXT indirectExecutionSet, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_indirect_execution_set(
        &self,
        indirect_execution_set: Option<vk::IndirectExecutionSetEXT>,
    ) -> () {
        unsafe {
            self.commands().destroy_indirect_execution_set(
                self.raw(),
                indirect_execution_set,
            )
        }
    }
    /// ```c
    /// void vkGetGeneratedCommandsMemoryRequirementsEXT(VkDevice device, VkGeneratedCommandsMemoryRequirementsInfoEXT const* pInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    unsafe fn get_generated_commands_memory_requirements(
        &self,
        info: &GeneratedCommandsMemoryRequirementsInfoEXT,
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
    /// ```c
    /// void vkUpdateIndirectExecutionSetPipelineEXT(VkDevice device, VkIndirectExecutionSetEXT indirectExecutionSet, uint32_t executionSetWriteCount, VkWriteIndirectExecutionSetPipelineEXT const* pExecutionSetWrites)
    /// ```
    unsafe fn update_indirect_execution_set_pipeline(
        &self,
        indirect_execution_set: vk::IndirectExecutionSetEXT,
        execution_set_writes: &[WriteIndirectExecutionSetPipelineEXT],
    ) -> () {
        unsafe {
            self.commands().update_indirect_execution_set_pipeline(
                self.raw(),
                indirect_execution_set,
                execution_set_writes,
            )
        }
    }
    /// ```c
    /// void vkUpdateIndirectExecutionSetShaderEXT(VkDevice device, VkIndirectExecutionSetEXT indirectExecutionSet, uint32_t executionSetWriteCount, VkWriteIndirectExecutionSetShaderEXT const* pExecutionSetWrites)
    /// ```
    unsafe fn update_indirect_execution_set_shader(
        &self,
        indirect_execution_set: vk::IndirectExecutionSetEXT,
        execution_set_writes: &[WriteIndirectExecutionSetShaderEXT],
    ) -> () {
        unsafe {
            self.commands().update_indirect_execution_set_shader(
                self.raw(),
                indirect_execution_set,
                execution_set_writes,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::ext::device_generated_commands {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::device_generated_commands> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::device_generated_commands {
        type Output = crate::hnd::Device<vk::extensions::ext::device_generated_commands>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::device_generated_commands>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::device_generated_commands> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::device_generated_commands> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::device_generated_commands> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::device_generated_commands> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtDeviceGeneratedCommandsDevice for crate::hnd::Device<vk::extensions::ext::device_generated_commands> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::device_generated_commands, vk::Device> for crate::hnd::Device<vk::extensions::ext::device_generated_commands> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait ExtDeviceGeneratedCommandsCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdExecuteGeneratedCommandsEXT(VkCommandBuffer commandBuffer, VkBool32 isPreprocessed, VkGeneratedCommandsInfoEXT const* pGeneratedCommandsInfo)
    /// ```
    unsafe fn execute_generated_commands(
        &self,
        is_preprocessed: bool,
        generated_commands_info: &GeneratedCommandsInfoEXT,
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
    /// void vkCmdPreprocessGeneratedCommandsEXT(VkCommandBuffer commandBuffer, VkGeneratedCommandsInfoEXT const* pGeneratedCommandsInfo, VkCommandBuffer stateCommandBuffer)
    /// ```
    unsafe fn preprocess_generated_commands(
        &self,
        generated_commands_info: &GeneratedCommandsInfoEXT,
        state_command_buffer: vk::CommandBuffer,
    ) -> () {
        unsafe {
            self.commands().cmd_preprocess_generated_commands(
                self.raw(),
                generated_commands_info,
                state_command_buffer,
            )
        }
    }
}

impl<O: crate::HndCtx<vk::extensions::ext::device_generated_commands, vk::Device>> crate::Owner<vk::IndirectCommandsLayoutEXT, vk::extensions::ext::device_generated_commands> for O {
    fn drop(&mut self, raw: vk::IndirectCommandsLayoutEXT) {
        unsafe {
            self.commands().destroy_indirect_commands_layout(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::ext::device_generated_commands, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::IndirectCommandsLayoutEXT, O, vk::extensions::ext::device_generated_commands>>
    where O: crate::HndCtx<vk::extensions::ext::device_generated_commands, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::IndirectCommandsLayoutEXT> for vk::extensions::ext::device_generated_commands {
    type Impl = _hs_IndirectCommandsLayoutEXT::IndirectCommandsLayoutEXT;
}


mod _hs_IndirectCommandsLayoutEXT {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct IndirectCommandsLayoutEXT(pub(crate) crate::handle::Hnd<vk::IndirectCommandsLayoutEXT, ::alloc::sync::Arc<super::Device>>);

    impl Clone for IndirectCommandsLayoutEXT {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::IndirectCommandsLayoutEXT, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::extensions::ext::device_generated_commands, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::IndirectCommandsLayoutEXT, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_indirect_commands_layout(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::IndirectCommandsLayoutEXT<vk::extensions::ext::device_generated_commands>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::ext::device_generated_commands, vk::Device>, raw: vk::IndirectCommandsLayoutEXT) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::ext::device_generated_commands, vk::Device>, raw: vk::IndirectCommandsLayoutEXT, dep: impl FnOnce() -> Dep) -> Self {
            Self(IndirectCommandsLayoutEXT(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::IndirectCommandsLayoutEXT<vk::extensions::ext::device_generated_commands> {
        pub fn raw(&self) -> vk::IndirectCommandsLayoutEXT { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::IndirectCommandsLayoutEXT<vk::extensions::ext::device_generated_commands> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("IndirectCommandsLayoutEXT({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::IndirectCommandsLayoutEXT<vk::extensions::ext::device_generated_commands> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::ext::device_generated_commands> for vk::IndirectCommandsLayoutEXT
        where Ctx: crate::HndCtx<vk::extensions::ext::device_generated_commands, vk::Device>,
    {
        type Output = crate::hnd::IndirectCommandsLayoutEXT<vk::extensions::ext::device_generated_commands>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::IndirectCommandsLayoutEXT::<vk::extensions::ext::device_generated_commands>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::extensions::ext::device_generated_commands, vk::Device>> crate::Owner<vk::IndirectExecutionSetEXT, vk::extensions::ext::device_generated_commands> for O {
    fn drop(&mut self, raw: vk::IndirectExecutionSetEXT) {
        unsafe {
            self.commands().destroy_indirect_execution_set(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::ext::device_generated_commands, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::IndirectExecutionSetEXT, O, vk::extensions::ext::device_generated_commands>>
    where O: crate::HndCtx<vk::extensions::ext::device_generated_commands, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::IndirectExecutionSetEXT> for vk::extensions::ext::device_generated_commands {
    type Impl = _hs_IndirectExecutionSetEXT::IndirectExecutionSetEXT;
}


mod _hs_IndirectExecutionSetEXT {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct IndirectExecutionSetEXT(pub(crate) crate::handle::Hnd<vk::IndirectExecutionSetEXT, ::alloc::sync::Arc<super::Device>>);

    impl Clone for IndirectExecutionSetEXT {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::IndirectExecutionSetEXT, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::extensions::ext::device_generated_commands, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::IndirectExecutionSetEXT, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_indirect_execution_set(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::IndirectExecutionSetEXT<vk::extensions::ext::device_generated_commands>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::ext::device_generated_commands, vk::Device>, raw: vk::IndirectExecutionSetEXT) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::ext::device_generated_commands, vk::Device>, raw: vk::IndirectExecutionSetEXT, dep: impl FnOnce() -> Dep) -> Self {
            Self(IndirectExecutionSetEXT(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::IndirectExecutionSetEXT<vk::extensions::ext::device_generated_commands> {
        pub fn raw(&self) -> vk::IndirectExecutionSetEXT { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::IndirectExecutionSetEXT<vk::extensions::ext::device_generated_commands> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("IndirectExecutionSetEXT({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::IndirectExecutionSetEXT<vk::extensions::ext::device_generated_commands> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::ext::device_generated_commands> for vk::IndirectExecutionSetEXT
        where Ctx: crate::HndCtx<vk::extensions::ext::device_generated_commands, vk::Device>,
    {
        type Output = crate::hnd::IndirectExecutionSetEXT<vk::extensions::ext::device_generated_commands>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::IndirectExecutionSetEXT::<vk::extensions::ext::device_generated_commands>::new_with(ctx, self, dep) }
        }
    }
}
