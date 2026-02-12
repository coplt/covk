// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_NV_scissor_exclusive` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::nv::scissor_exclusive::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::scissor_exclusive::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdSetExclusiveScissorEnableNV(VkCommandBuffer commandBuffer, uint32_t firstExclusiveScissor, uint32_t exclusiveScissorCount, VkBool32 const* pExclusiveScissorEnables)
    /// ```
    pub unsafe fn cmd_set_exclusive_scissor_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        first_exclusive_scissor: uint32_t,
        exclusive_scissor_enables: &[Bool],
    ) -> () {
        unsafe {
            self.0.CmdSetExclusiveScissorEnableNV(
                command_buffer.abi(), 
                first_exclusive_scissor.abi(), 
                exclusive_scissor_enables.len() as _, 
                exclusive_scissor_enables.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetExclusiveScissorNV(VkCommandBuffer commandBuffer, uint32_t firstExclusiveScissor, uint32_t exclusiveScissorCount, VkRect2D const* pExclusiveScissors)
    /// ```
    pub unsafe fn cmd_set_exclusive_scissor(
        &self,
        command_buffer: vk::CommandBuffer,
        first_exclusive_scissor: uint32_t,
        exclusive_scissors: &[Rect2D],
    ) -> () {
        unsafe {
            self.0.CmdSetExclusiveScissorNV(
                command_buffer.abi(), 
                first_exclusive_scissor.abi(), 
                exclusive_scissors.len() as _, 
                exclusive_scissors.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::nv::scissor_exclusive {
    type Commands = Device;
}

/// Device object
pub trait NvScissorExclusiveDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::nv::scissor_exclusive {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::nv::scissor_exclusive> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::nv::scissor_exclusive {
        type Output = crate::hnd::Device<vk::extensions::nv::scissor_exclusive>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::nv::scissor_exclusive>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::nv::scissor_exclusive> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::nv::scissor_exclusive> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::nv::scissor_exclusive> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::nv::scissor_exclusive> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvScissorExclusiveDevice for crate::hnd::Device<vk::extensions::nv::scissor_exclusive> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::scissor_exclusive, vk::Device> for crate::hnd::Device<vk::extensions::nv::scissor_exclusive> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait NvScissorExclusiveCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdSetExclusiveScissorEnableNV(VkCommandBuffer commandBuffer, uint32_t firstExclusiveScissor, uint32_t exclusiveScissorCount, VkBool32 const* pExclusiveScissorEnables)
    /// ```
    unsafe fn set_exclusive_scissor_enable(
        &self,
        first_exclusive_scissor: uint32_t,
        exclusive_scissor_enables: &[Bool],
    ) -> () {
        unsafe {
            self.commands().cmd_set_exclusive_scissor_enable(
                self.raw(),
                first_exclusive_scissor,
                exclusive_scissor_enables,
            )
        }
    }
    /// ```c
    /// void vkCmdSetExclusiveScissorNV(VkCommandBuffer commandBuffer, uint32_t firstExclusiveScissor, uint32_t exclusiveScissorCount, VkRect2D const* pExclusiveScissors)
    /// ```
    unsafe fn set_exclusive_scissor(
        &self,
        first_exclusive_scissor: uint32_t,
        exclusive_scissors: &[Rect2D],
    ) -> () {
        unsafe {
            self.commands().cmd_set_exclusive_scissor(
                self.raw(),
                first_exclusive_scissor,
                exclusive_scissors,
            )
        }
    }
}
