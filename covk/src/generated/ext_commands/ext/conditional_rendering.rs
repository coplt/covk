// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_conditional_rendering` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::conditional_rendering::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::conditional_rendering::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdBeginConditionalRenderingEXT(VkCommandBuffer commandBuffer, VkConditionalRenderingBeginInfoEXT const* pConditionalRenderingBegin)
    /// ```
    pub unsafe fn cmd_begin_conditional_rendering(
        &self,
        command_buffer: vk::CommandBuffer,
        conditional_rendering_begin: &ConditionalRenderingBeginInfoEXT,
    ) -> () {
        unsafe {
            self.0.CmdBeginConditionalRenderingEXT(
                command_buffer.abi(), 
                conditional_rendering_begin.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdEndConditionalRenderingEXT(VkCommandBuffer commandBuffer)
    /// ```
    pub unsafe fn cmd_end_conditional_rendering(
        &self,
        command_buffer: vk::CommandBuffer,
    ) -> () {
        unsafe {
            self.0.CmdEndConditionalRenderingEXT(
                command_buffer.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::conditional_rendering {
    type Commands = Device;
}

/// Device object
pub trait ExtConditionalRenderingDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::ext::conditional_rendering {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::conditional_rendering> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::conditional_rendering {
        type Output = crate::hnd::Device<vk::extensions::ext::conditional_rendering>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::conditional_rendering>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::conditional_rendering> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::conditional_rendering> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::conditional_rendering> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::conditional_rendering> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtConditionalRenderingDevice for crate::hnd::Device<vk::extensions::ext::conditional_rendering> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::conditional_rendering, vk::Device> for crate::hnd::Device<vk::extensions::ext::conditional_rendering> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait ExtConditionalRenderingCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdBeginConditionalRenderingEXT(VkCommandBuffer commandBuffer, VkConditionalRenderingBeginInfoEXT const* pConditionalRenderingBegin)
    /// ```
    unsafe fn begin_conditional_rendering(
        &self,
        conditional_rendering_begin: &ConditionalRenderingBeginInfoEXT,
    ) -> () {
        unsafe {
            self.commands().cmd_begin_conditional_rendering(
                self.raw(),
                conditional_rendering_begin,
            )
        }
    }
    /// ```c
    /// void vkCmdEndConditionalRenderingEXT(VkCommandBuffer commandBuffer)
    /// ```
    unsafe fn end_conditional_rendering(
        &self,
    ) -> () {
        unsafe {
            self.commands().cmd_end_conditional_rendering(
                self.raw(),
            )
        }
    }
}
