// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_NV_device_generated_commands_compute` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::nv::device_generated_commands_compute::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::device_generated_commands_compute::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdUpdatePipelineIndirectBufferNV(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipeline pipeline)
    /// ```
    pub unsafe fn cmd_update_pipeline_indirect_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: vk::Pipeline,
    ) -> () {
        unsafe {
            self.0.CmdUpdatePipelineIndirectBufferNV(
                command_buffer.abi(), 
                pipeline_bind_point.abi(), 
                pipeline.abi(), 
            );
        }
    }
    /// ```c
    /// VkDeviceAddress vkGetPipelineIndirectDeviceAddressNV(VkDevice device, VkPipelineIndirectDeviceAddressInfoNV const* pInfo)
    /// ```
    pub unsafe fn get_pipeline_indirect_device_address(
        &self,
        device: vk::Device,
        info: &PipelineIndirectDeviceAddressInfoNV,
    ) -> DeviceAddress {
        unsafe {
            let _r = self.0.GetPipelineIndirectDeviceAddressNV(
                device.abi(), 
                info.abi(), 
            );
            _r
        }
    }
    /// ```c
    /// void vkGetPipelineIndirectMemoryRequirementsNV(VkDevice device, VkComputePipelineCreateInfo const* pCreateInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    pub unsafe fn get_pipeline_indirect_memory_requirements(
        &self,
        device: vk::Device,
        create_info: &ComputePipelineCreateInfo,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self.0.GetPipelineIndirectMemoryRequirementsNV(
                device.abi(), 
                create_info.abi(), 
                memory_requirements.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::nv::device_generated_commands_compute {
    type Commands = Device;
}

/// Device object
pub trait NvDeviceGeneratedCommandsComputeDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkDeviceAddress vkGetPipelineIndirectDeviceAddressNV(VkDevice device, VkPipelineIndirectDeviceAddressInfoNV const* pInfo)
    /// ```
    unsafe fn get_pipeline_indirect_device_address(
        &self,
        info: &PipelineIndirectDeviceAddressInfoNV,
    ) -> DeviceAddress {
        unsafe {
            self.commands().get_pipeline_indirect_device_address(
                self.raw(),
                info,
            )
        }
    }
    /// ```c
    /// void vkGetPipelineIndirectMemoryRequirementsNV(VkDevice device, VkComputePipelineCreateInfo const* pCreateInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    unsafe fn get_pipeline_indirect_memory_requirements(
        &self,
        create_info: &ComputePipelineCreateInfo,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self.commands().get_pipeline_indirect_memory_requirements(
                self.raw(),
                create_info,
                memory_requirements,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::nv::device_generated_commands_compute {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::nv::device_generated_commands_compute> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::nv::device_generated_commands_compute {
        type Output = crate::hnd::Device<vk::extensions::nv::device_generated_commands_compute>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::nv::device_generated_commands_compute>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::nv::device_generated_commands_compute> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::nv::device_generated_commands_compute> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::nv::device_generated_commands_compute> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::nv::device_generated_commands_compute> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvDeviceGeneratedCommandsComputeDevice for crate::hnd::Device<vk::extensions::nv::device_generated_commands_compute> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::device_generated_commands_compute, vk::Device> for crate::hnd::Device<vk::extensions::nv::device_generated_commands_compute> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait NvDeviceGeneratedCommandsComputeCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdUpdatePipelineIndirectBufferNV(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipeline pipeline)
    /// ```
    unsafe fn update_pipeline_indirect_buffer(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: vk::Pipeline,
    ) -> () {
        unsafe {
            self.commands().cmd_update_pipeline_indirect_buffer(
                self.raw(),
                pipeline_bind_point,
                pipeline,
            )
        }
    }
}
