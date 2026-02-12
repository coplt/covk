// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_maintenance6` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::maintenance6::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::maintenance6::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdBindDescriptorBufferEmbeddedSamplers2EXT(VkCommandBuffer commandBuffer, VkBindDescriptorBufferEmbeddedSamplersInfoEXT const* pBindDescriptorBufferEmbeddedSamplersInfo)
    /// ```
    pub unsafe fn cmd_bind_descriptor_buffer_embedded_samplers2ext(
        &self,
        command_buffer: vk::CommandBuffer,
        bind_descriptor_buffer_embedded_samplers_info: &BindDescriptorBufferEmbeddedSamplersInfoEXT,
    ) -> () {
        unsafe {
            self.0.CmdBindDescriptorBufferEmbeddedSamplers2EXT(
                command_buffer.abi(), 
                bind_descriptor_buffer_embedded_samplers_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdBindDescriptorSets2(VkCommandBuffer commandBuffer, VkBindDescriptorSetsInfo const* pBindDescriptorSetsInfo)
    /// ```
    pub unsafe fn cmd_bind_descriptor_sets2(
        &self,
        command_buffer: vk::CommandBuffer,
        bind_descriptor_sets_info: &BindDescriptorSetsInfo,
    ) -> () {
        unsafe {
            self.0.CmdBindDescriptorSets2KHR(
                command_buffer.abi(), 
                bind_descriptor_sets_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdPushConstants2(VkCommandBuffer commandBuffer, VkPushConstantsInfo const* pPushConstantsInfo)
    /// ```
    pub unsafe fn cmd_push_constants2(
        &self,
        command_buffer: vk::CommandBuffer,
        push_constants_info: &PushConstantsInfo,
    ) -> () {
        unsafe {
            self.0.CmdPushConstants2KHR(
                command_buffer.abi(), 
                push_constants_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdPushDescriptorSet2(VkCommandBuffer commandBuffer, VkPushDescriptorSetInfo const* pPushDescriptorSetInfo)
    /// ```
    pub unsafe fn cmd_push_descriptor_set2(
        &self,
        command_buffer: vk::CommandBuffer,
        push_descriptor_set_info: &PushDescriptorSetInfo,
    ) -> () {
        unsafe {
            self.0.CmdPushDescriptorSet2KHR(
                command_buffer.abi(), 
                push_descriptor_set_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdPushDescriptorSetWithTemplate2(VkCommandBuffer commandBuffer, VkPushDescriptorSetWithTemplateInfo const* pPushDescriptorSetWithTemplateInfo)
    /// ```
    pub unsafe fn cmd_push_descriptor_set_with_template2(
        &self,
        command_buffer: vk::CommandBuffer,
        push_descriptor_set_with_template_info: &PushDescriptorSetWithTemplateInfo,
    ) -> () {
        unsafe {
            self.0.CmdPushDescriptorSetWithTemplate2KHR(
                command_buffer.abi(), 
                push_descriptor_set_with_template_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetDescriptorBufferOffsets2EXT(VkCommandBuffer commandBuffer, VkSetDescriptorBufferOffsetsInfoEXT const* pSetDescriptorBufferOffsetsInfo)
    /// ```
    pub unsafe fn cmd_set_descriptor_buffer_offsets2ext(
        &self,
        command_buffer: vk::CommandBuffer,
        set_descriptor_buffer_offsets_info: &SetDescriptorBufferOffsetsInfoEXT,
    ) -> () {
        unsafe {
            self.0.CmdSetDescriptorBufferOffsets2EXT(
                command_buffer.abi(), 
                set_descriptor_buffer_offsets_info.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::maintenance6 {
    type Commands = Device;
}

/// Device object
pub trait KhrMaintenance6Device {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::khr::maintenance6 {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::maintenance6> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::maintenance6 {
        type Output = crate::hnd::Device<vk::extensions::khr::maintenance6>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::maintenance6>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::maintenance6> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::maintenance6> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::maintenance6> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::maintenance6> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrMaintenance6Device for crate::hnd::Device<vk::extensions::khr::maintenance6> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::maintenance6, vk::Device> for crate::hnd::Device<vk::extensions::khr::maintenance6> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait KhrMaintenance6CommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdBindDescriptorBufferEmbeddedSamplers2EXT(VkCommandBuffer commandBuffer, VkBindDescriptorBufferEmbeddedSamplersInfoEXT const* pBindDescriptorBufferEmbeddedSamplersInfo)
    /// ```
    unsafe fn bind_descriptor_buffer_embedded_samplers2ext(
        &self,
        bind_descriptor_buffer_embedded_samplers_info: &BindDescriptorBufferEmbeddedSamplersInfoEXT,
    ) -> () {
        unsafe {
            self.commands().cmd_bind_descriptor_buffer_embedded_samplers2ext(
                self.raw(),
                bind_descriptor_buffer_embedded_samplers_info,
            )
        }
    }
    /// ```c
    /// void vkCmdBindDescriptorSets2(VkCommandBuffer commandBuffer, VkBindDescriptorSetsInfo const* pBindDescriptorSetsInfo)
    /// ```
    unsafe fn bind_descriptor_sets2(
        &self,
        bind_descriptor_sets_info: &BindDescriptorSetsInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_bind_descriptor_sets2(
                self.raw(),
                bind_descriptor_sets_info,
            )
        }
    }
    /// ```c
    /// void vkCmdPushConstants2(VkCommandBuffer commandBuffer, VkPushConstantsInfo const* pPushConstantsInfo)
    /// ```
    unsafe fn push_constants2(
        &self,
        push_constants_info: &PushConstantsInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_push_constants2(
                self.raw(),
                push_constants_info,
            )
        }
    }
    /// ```c
    /// void vkCmdPushDescriptorSet2(VkCommandBuffer commandBuffer, VkPushDescriptorSetInfo const* pPushDescriptorSetInfo)
    /// ```
    unsafe fn push_descriptor_set2(
        &self,
        push_descriptor_set_info: &PushDescriptorSetInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_push_descriptor_set2(
                self.raw(),
                push_descriptor_set_info,
            )
        }
    }
    /// ```c
    /// void vkCmdPushDescriptorSetWithTemplate2(VkCommandBuffer commandBuffer, VkPushDescriptorSetWithTemplateInfo const* pPushDescriptorSetWithTemplateInfo)
    /// ```
    unsafe fn push_descriptor_set_with_template2(
        &self,
        push_descriptor_set_with_template_info: &PushDescriptorSetWithTemplateInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_push_descriptor_set_with_template2(
                self.raw(),
                push_descriptor_set_with_template_info,
            )
        }
    }
    /// ```c
    /// void vkCmdSetDescriptorBufferOffsets2EXT(VkCommandBuffer commandBuffer, VkSetDescriptorBufferOffsetsInfoEXT const* pSetDescriptorBufferOffsetsInfo)
    /// ```
    unsafe fn set_descriptor_buffer_offsets2ext(
        &self,
        set_descriptor_buffer_offsets_info: &SetDescriptorBufferOffsetsInfoEXT,
    ) -> () {
        unsafe {
            self.commands().cmd_set_descriptor_buffer_offsets2ext(
                self.raw(),
                set_descriptor_buffer_offsets_info,
            )
        }
    }
}
