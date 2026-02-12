// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_dynamic_rendering_local_read` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::dynamic_rendering_local_read::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::dynamic_rendering_local_read::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdSetRenderingAttachmentLocations(VkCommandBuffer commandBuffer, VkRenderingAttachmentLocationInfo const* pLocationInfo)
    /// ```
    pub unsafe fn cmd_set_rendering_attachment_locations(
        &self,
        command_buffer: vk::CommandBuffer,
        location_info: &RenderingAttachmentLocationInfo,
    ) -> () {
        unsafe {
            self.0.CmdSetRenderingAttachmentLocationsKHR(
                command_buffer.abi(), 
                location_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetRenderingInputAttachmentIndices(VkCommandBuffer commandBuffer, VkRenderingInputAttachmentIndexInfo const* pInputAttachmentIndexInfo)
    /// ```
    pub unsafe fn cmd_set_rendering_input_attachment_indices(
        &self,
        command_buffer: vk::CommandBuffer,
        input_attachment_index_info: &RenderingInputAttachmentIndexInfo,
    ) -> () {
        unsafe {
            self.0.CmdSetRenderingInputAttachmentIndicesKHR(
                command_buffer.abi(), 
                input_attachment_index_info.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::dynamic_rendering_local_read {
    type Commands = Device;
}

/// Device object
pub trait KhrDynamicRenderingLocalReadDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::khr::dynamic_rendering_local_read {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::dynamic_rendering_local_read> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::dynamic_rendering_local_read {
        type Output = crate::hnd::Device<vk::extensions::khr::dynamic_rendering_local_read>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::dynamic_rendering_local_read>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::dynamic_rendering_local_read> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::dynamic_rendering_local_read> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::dynamic_rendering_local_read> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::dynamic_rendering_local_read> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrDynamicRenderingLocalReadDevice for crate::hnd::Device<vk::extensions::khr::dynamic_rendering_local_read> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::dynamic_rendering_local_read, vk::Device> for crate::hnd::Device<vk::extensions::khr::dynamic_rendering_local_read> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait KhrDynamicRenderingLocalReadCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdSetRenderingAttachmentLocations(VkCommandBuffer commandBuffer, VkRenderingAttachmentLocationInfo const* pLocationInfo)
    /// ```
    unsafe fn set_rendering_attachment_locations(
        &self,
        location_info: &RenderingAttachmentLocationInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_set_rendering_attachment_locations(
                self.raw(),
                location_info,
            )
        }
    }
    /// ```c
    /// void vkCmdSetRenderingInputAttachmentIndices(VkCommandBuffer commandBuffer, VkRenderingInputAttachmentIndexInfo const* pInputAttachmentIndexInfo)
    /// ```
    unsafe fn set_rendering_input_attachment_indices(
        &self,
        input_attachment_index_info: &RenderingInputAttachmentIndexInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_set_rendering_input_attachment_indices(
                self.raw(),
                input_attachment_index_info,
            )
        }
    }
}
