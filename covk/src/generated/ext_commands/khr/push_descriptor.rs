// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_push_descriptor` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::push_descriptor::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::push_descriptor::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdPushDescriptorSet(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipelineLayout layout, uint32_t set, uint32_t descriptorWriteCount, VkWriteDescriptorSet const* pDescriptorWrites)
    /// ```
    pub unsafe fn cmd_push_descriptor_set(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: vk::PipelineLayout,
        set: uint32_t,
        descriptor_writes: &[WriteDescriptorSet],
    ) -> () {
        unsafe {
            self.0.CmdPushDescriptorSetKHR(
                command_buffer.abi(), 
                pipeline_bind_point.abi(), 
                layout.abi(), 
                set.abi(), 
                descriptor_writes.len() as _, 
                descriptor_writes.abi(), 
            );
        }
    }
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
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::push_descriptor {
    type Commands = Device;
}

/// Device object
pub trait KhrPushDescriptorDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::khr::push_descriptor {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::push_descriptor> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::push_descriptor {
        type Output = crate::hnd::Device<vk::extensions::khr::push_descriptor>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::push_descriptor>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::push_descriptor> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::push_descriptor> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::push_descriptor> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::push_descriptor> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrPushDescriptorDevice for crate::hnd::Device<vk::extensions::khr::push_descriptor> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::push_descriptor, vk::Device> for crate::hnd::Device<vk::extensions::khr::push_descriptor> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait KhrPushDescriptorCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdPushDescriptorSet(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipelineLayout layout, uint32_t set, uint32_t descriptorWriteCount, VkWriteDescriptorSet const* pDescriptorWrites)
    /// ```
    unsafe fn push_descriptor_set(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        layout: vk::PipelineLayout,
        set: uint32_t,
        descriptor_writes: &[WriteDescriptorSet],
    ) -> () {
        unsafe {
            self.commands().cmd_push_descriptor_set(
                self.raw(),
                pipeline_bind_point,
                layout,
                set,
                descriptor_writes,
            )
        }
    }
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
