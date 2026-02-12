// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_dynamic_rendering` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::dynamic_rendering::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::dynamic_rendering::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdBeginRendering(VkCommandBuffer commandBuffer, VkRenderingInfo const* pRenderingInfo)
    /// ```
    pub unsafe fn cmd_begin_rendering(
        &self,
        command_buffer: vk::CommandBuffer,
        rendering_info: &RenderingInfo,
    ) -> () {
        unsafe {
            self.0.CmdBeginRenderingKHR(
                command_buffer.abi(), 
                rendering_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdEndRendering(VkCommandBuffer commandBuffer)
    /// ```
    pub unsafe fn cmd_end_rendering(
        &self,
        command_buffer: vk::CommandBuffer,
    ) -> () {
        unsafe {
            self.0.CmdEndRenderingKHR(
                command_buffer.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::dynamic_rendering {
    type Commands = Device;
}

/// Device object
pub trait KhrDynamicRenderingDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::khr::dynamic_rendering {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::dynamic_rendering> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::dynamic_rendering {
        type Output = crate::hnd::Device<vk::extensions::khr::dynamic_rendering>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::dynamic_rendering>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::dynamic_rendering> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::dynamic_rendering> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::dynamic_rendering> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::dynamic_rendering> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrDynamicRenderingDevice for crate::hnd::Device<vk::extensions::khr::dynamic_rendering> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::dynamic_rendering, vk::Device> for crate::hnd::Device<vk::extensions::khr::dynamic_rendering> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait KhrDynamicRenderingCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdBeginRendering(VkCommandBuffer commandBuffer, VkRenderingInfo const* pRenderingInfo)
    /// ```
    unsafe fn begin_rendering(
        &self,
        rendering_info: &RenderingInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_begin_rendering(
                self.raw(),
                rendering_info,
            )
        }
    }
    /// ```c
    /// void vkCmdEndRendering(VkCommandBuffer commandBuffer)
    /// ```
    unsafe fn end_rendering(
        &self,
    ) -> () {
        unsafe {
            self.commands().cmd_end_rendering(
                self.raw(),
            )
        }
    }
}
