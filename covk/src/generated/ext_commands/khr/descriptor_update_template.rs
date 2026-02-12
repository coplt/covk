// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_descriptor_update_template` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::descriptor_update_template::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::descriptor_update_template::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdPushDescriptorSetWithTemplate(VkCommandBuffer commandBuffer, VkDescriptorUpdateTemplate descriptorUpdateTemplate, VkPipelineLayout layout, uint32_t set, void const* pData)
    /// ```
    pub unsafe fn cmd_push_descriptor_set_with_template(
        &self,
        command_buffer: vk::CommandBuffer,
        descriptor_update_template: vk::DescriptorUpdateTemplate,
        layout: vk::PipelineLayout,
        set: uint32_t,
        data: *const void,
    ) -> () {
        unsafe {
            self.0.CmdPushDescriptorSetWithTemplateKHR(
                command_buffer.abi(), 
                descriptor_update_template.abi(), 
                layout.abi(), 
                set.abi(), 
                data.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkCreateDescriptorUpdateTemplate(VkDevice device, VkDescriptorUpdateTemplateCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkDescriptorUpdateTemplate* pDescriptorUpdateTemplate)
    /// ```
    pub unsafe fn create_descriptor_update_template(
        &self,
        device: vk::Device,
        create_info: &DescriptorUpdateTemplateCreateInfo,
    ) -> crate::Result<vk::DescriptorUpdateTemplate> {
        unsafe {
            let mut _v: Option<vk::DescriptorUpdateTemplate> = Default::default();
            let _r = self.0.CreateDescriptorUpdateTemplateKHR(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDestroyDescriptorUpdateTemplate(VkDevice device, VkDescriptorUpdateTemplate descriptorUpdateTemplate, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_descriptor_update_template(
        &self,
        device: vk::Device,
        descriptor_update_template: Option<vk::DescriptorUpdateTemplate>,
    ) -> () {
        unsafe {
            self.0.DestroyDescriptorUpdateTemplateKHR(
                device.abi(), 
                descriptor_update_template.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkUpdateDescriptorSetWithTemplate(VkDevice device, VkDescriptorSet descriptorSet, VkDescriptorUpdateTemplate descriptorUpdateTemplate, void const* pData)
    /// ```
    pub unsafe fn update_descriptor_set_with_template(
        &self,
        device: vk::Device,
        descriptor_set: vk::DescriptorSet,
        descriptor_update_template: vk::DescriptorUpdateTemplate,
        data: *const void,
    ) -> () {
        unsafe {
            self.0.UpdateDescriptorSetWithTemplateKHR(
                device.abi(), 
                descriptor_set.abi(), 
                descriptor_update_template.abi(), 
                data.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::descriptor_update_template {
    type Commands = Device;
}

/// Device object
pub trait KhrDescriptorUpdateTemplateDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkCreateDescriptorUpdateTemplate(VkDevice device, VkDescriptorUpdateTemplateCreateInfo const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkDescriptorUpdateTemplate* pDescriptorUpdateTemplate)
    /// ```
    unsafe fn create_descriptor_update_template(
        &self,
        create_info: &DescriptorUpdateTemplateCreateInfo,
    ) -> crate::Result<vk::DescriptorUpdateTemplate> {
        unsafe {
            self.commands().create_descriptor_update_template(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// void vkDestroyDescriptorUpdateTemplate(VkDevice device, VkDescriptorUpdateTemplate descriptorUpdateTemplate, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_descriptor_update_template(
        &self,
        descriptor_update_template: Option<vk::DescriptorUpdateTemplate>,
    ) -> () {
        unsafe {
            self.commands().destroy_descriptor_update_template(
                self.raw(),
                descriptor_update_template,
            )
        }
    }
    /// ```c
    /// void vkUpdateDescriptorSetWithTemplate(VkDevice device, VkDescriptorSet descriptorSet, VkDescriptorUpdateTemplate descriptorUpdateTemplate, void const* pData)
    /// ```
    unsafe fn update_descriptor_set_with_template(
        &self,
        descriptor_set: vk::DescriptorSet,
        descriptor_update_template: vk::DescriptorUpdateTemplate,
        data: *const void,
    ) -> () {
        unsafe {
            self.commands().update_descriptor_set_with_template(
                self.raw(),
                descriptor_set,
                descriptor_update_template,
                data,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::descriptor_update_template {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::descriptor_update_template> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::descriptor_update_template {
        type Output = crate::hnd::Device<vk::extensions::khr::descriptor_update_template>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::descriptor_update_template>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::descriptor_update_template> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::descriptor_update_template> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::descriptor_update_template> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::descriptor_update_template> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrDescriptorUpdateTemplateDevice for crate::hnd::Device<vk::extensions::khr::descriptor_update_template> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::descriptor_update_template, vk::Device> for crate::hnd::Device<vk::extensions::khr::descriptor_update_template> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait KhrDescriptorUpdateTemplateCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdPushDescriptorSetWithTemplate(VkCommandBuffer commandBuffer, VkDescriptorUpdateTemplate descriptorUpdateTemplate, VkPipelineLayout layout, uint32_t set, void const* pData)
    /// ```
    unsafe fn push_descriptor_set_with_template(
        &self,
        descriptor_update_template: vk::DescriptorUpdateTemplate,
        layout: vk::PipelineLayout,
        set: uint32_t,
        data: *const void,
    ) -> () {
        unsafe {
            self.commands().cmd_push_descriptor_set_with_template(
                self.raw(),
                descriptor_update_template,
                layout,
                set,
                data,
            )
        }
    }
}

impl<O: crate::HndCtx<vk::extensions::khr::descriptor_update_template, vk::Device>> crate::Owner<vk::DescriptorUpdateTemplate, vk::extensions::khr::descriptor_update_template> for O {
    fn drop(&mut self, raw: vk::DescriptorUpdateTemplate) {
        unsafe {
            self.commands().destroy_descriptor_update_template(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::khr::descriptor_update_template, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::DescriptorUpdateTemplate, O, vk::extensions::khr::descriptor_update_template>>
    where O: crate::HndCtx<vk::extensions::khr::descriptor_update_template, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::DescriptorUpdateTemplate> for vk::extensions::khr::descriptor_update_template {
    type Impl = _hs_DescriptorUpdateTemplate::DescriptorUpdateTemplate;
}


mod _hs_DescriptorUpdateTemplate {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct DescriptorUpdateTemplate(pub(crate) crate::handle::Hnd<vk::DescriptorUpdateTemplate, ::alloc::sync::Arc<super::Device>>);

    impl Clone for DescriptorUpdateTemplate {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::DescriptorUpdateTemplate, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::extensions::khr::descriptor_update_template, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::DescriptorUpdateTemplate, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_descriptor_update_template(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::DescriptorUpdateTemplate<vk::extensions::khr::descriptor_update_template>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::khr::descriptor_update_template, vk::Device>, raw: vk::DescriptorUpdateTemplate) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::khr::descriptor_update_template, vk::Device>, raw: vk::DescriptorUpdateTemplate, dep: impl FnOnce() -> Dep) -> Self {
            Self(DescriptorUpdateTemplate(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::DescriptorUpdateTemplate<vk::extensions::khr::descriptor_update_template> {
        pub fn raw(&self) -> vk::DescriptorUpdateTemplate { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::DescriptorUpdateTemplate<vk::extensions::khr::descriptor_update_template> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("DescriptorUpdateTemplate({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::DescriptorUpdateTemplate<vk::extensions::khr::descriptor_update_template> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::khr::descriptor_update_template> for vk::DescriptorUpdateTemplate
        where Ctx: crate::HndCtx<vk::extensions::khr::descriptor_update_template, vk::Device>,
    {
        type Output = crate::hnd::DescriptorUpdateTemplate<vk::extensions::khr::descriptor_update_template>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::DescriptorUpdateTemplate::<vk::extensions::khr::descriptor_update_template>::new_with(ctx, self, dep) }
        }
    }
}
