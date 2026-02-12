// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_multi_draw` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::multi_draw::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::multi_draw::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdDrawMultiEXT(VkCommandBuffer commandBuffer, uint32_t drawCount, VkMultiDrawInfoEXT const* pVertexInfo, uint32_t instanceCount, uint32_t firstInstance, uint32_t stride)
    /// ```
    pub unsafe fn cmd_draw_multi(
        &self,
        command_buffer: vk::CommandBuffer,
        vertex_info: &[MultiDrawInfoEXT],
        instance_count: uint32_t,
        first_instance: uint32_t,
        stride: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdDrawMultiEXT(
                command_buffer.abi(), 
                vertex_info.len() as _, 
                vertex_info.abi(), 
                instance_count.abi(), 
                first_instance.abi(), 
                stride.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdDrawMultiIndexedEXT(VkCommandBuffer commandBuffer, uint32_t drawCount, VkMultiDrawIndexedInfoEXT const* pIndexInfo, uint32_t instanceCount, uint32_t firstInstance, uint32_t stride, int32_t const* pVertexOffset)
    /// ```
    pub unsafe fn cmd_draw_multi_indexed(
        &self,
        command_buffer: vk::CommandBuffer,
        index_info: &[MultiDrawIndexedInfoEXT],
        instance_count: uint32_t,
        first_instance: uint32_t,
        stride: uint32_t,
        vertex_offset: *const int32_t,
    ) -> () {
        unsafe {
            self.0.CmdDrawMultiIndexedEXT(
                command_buffer.abi(), 
                index_info.len() as _, 
                index_info.abi(), 
                instance_count.abi(), 
                first_instance.abi(), 
                stride.abi(), 
                vertex_offset.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::multi_draw {
    type Commands = Device;
}

/// Device object
pub trait ExtMultiDrawDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::ext::multi_draw {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::multi_draw> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::multi_draw {
        type Output = crate::hnd::Device<vk::extensions::ext::multi_draw>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::multi_draw>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::multi_draw> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::multi_draw> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::multi_draw> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::multi_draw> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtMultiDrawDevice for crate::hnd::Device<vk::extensions::ext::multi_draw> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::multi_draw, vk::Device> for crate::hnd::Device<vk::extensions::ext::multi_draw> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait ExtMultiDrawCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdDrawMultiEXT(VkCommandBuffer commandBuffer, uint32_t drawCount, VkMultiDrawInfoEXT const* pVertexInfo, uint32_t instanceCount, uint32_t firstInstance, uint32_t stride)
    /// ```
    unsafe fn draw_multi(
        &self,
        vertex_info: &[MultiDrawInfoEXT],
        instance_count: uint32_t,
        first_instance: uint32_t,
        stride: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_draw_multi(
                self.raw(),
                vertex_info,
                instance_count,
                first_instance,
                stride,
            )
        }
    }
    /// ```c
    /// void vkCmdDrawMultiIndexedEXT(VkCommandBuffer commandBuffer, uint32_t drawCount, VkMultiDrawIndexedInfoEXT const* pIndexInfo, uint32_t instanceCount, uint32_t firstInstance, uint32_t stride, int32_t const* pVertexOffset)
    /// ```
    unsafe fn draw_multi_indexed(
        &self,
        index_info: &[MultiDrawIndexedInfoEXT],
        instance_count: uint32_t,
        first_instance: uint32_t,
        stride: uint32_t,
        vertex_offset: *const int32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_draw_multi_indexed(
                self.raw(),
                index_info,
                instance_count,
                first_instance,
                stride,
                vertex_offset,
            )
        }
    }
}
