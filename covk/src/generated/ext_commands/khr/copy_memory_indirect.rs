// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_copy_memory_indirect` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::copy_memory_indirect::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::copy_memory_indirect::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdCopyMemoryIndirectKHR(VkCommandBuffer commandBuffer, VkCopyMemoryIndirectInfoKHR const* pCopyMemoryIndirectInfo)
    /// ```
    pub unsafe fn cmd_copy_memory_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_memory_indirect_info: &CopyMemoryIndirectInfoKHR,
    ) -> () {
        unsafe {
            self.0.CmdCopyMemoryIndirectKHR(
                command_buffer.abi(), 
                copy_memory_indirect_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdCopyMemoryToImageIndirectKHR(VkCommandBuffer commandBuffer, VkCopyMemoryToImageIndirectInfoKHR const* pCopyMemoryToImageIndirectInfo)
    /// ```
    pub unsafe fn cmd_copy_memory_to_image_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_memory_to_image_indirect_info: &CopyMemoryToImageIndirectInfoKHR,
    ) -> () {
        unsafe {
            self.0.CmdCopyMemoryToImageIndirectKHR(
                command_buffer.abi(), 
                copy_memory_to_image_indirect_info.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::copy_memory_indirect {
    type Commands = Device;
}

/// Device object
pub trait KhrCopyMemoryIndirectDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::khr::copy_memory_indirect {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::copy_memory_indirect> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::copy_memory_indirect {
        type Output = crate::hnd::Device<vk::extensions::khr::copy_memory_indirect>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::copy_memory_indirect>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::copy_memory_indirect> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::copy_memory_indirect> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::copy_memory_indirect> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::copy_memory_indirect> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrCopyMemoryIndirectDevice for crate::hnd::Device<vk::extensions::khr::copy_memory_indirect> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::copy_memory_indirect, vk::Device> for crate::hnd::Device<vk::extensions::khr::copy_memory_indirect> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait KhrCopyMemoryIndirectCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdCopyMemoryIndirectKHR(VkCommandBuffer commandBuffer, VkCopyMemoryIndirectInfoKHR const* pCopyMemoryIndirectInfo)
    /// ```
    unsafe fn copy_memory_indirect(
        &self,
        copy_memory_indirect_info: &CopyMemoryIndirectInfoKHR,
    ) -> () {
        unsafe {
            self.commands().cmd_copy_memory_indirect(
                self.raw(),
                copy_memory_indirect_info,
            )
        }
    }
    /// ```c
    /// void vkCmdCopyMemoryToImageIndirectKHR(VkCommandBuffer commandBuffer, VkCopyMemoryToImageIndirectInfoKHR const* pCopyMemoryToImageIndirectInfo)
    /// ```
    unsafe fn copy_memory_to_image_indirect(
        &self,
        copy_memory_to_image_indirect_info: &CopyMemoryToImageIndirectInfoKHR,
    ) -> () {
        unsafe {
            self.commands().cmd_copy_memory_to_image_indirect(
                self.raw(),
                copy_memory_to_image_indirect_info,
            )
        }
    }
}
