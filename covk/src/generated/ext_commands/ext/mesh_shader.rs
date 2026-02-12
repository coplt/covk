// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_mesh_shader` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::mesh_shader::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::mesh_shader::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdDrawMeshTasksEXT(VkCommandBuffer commandBuffer, uint32_t groupCountX, uint32_t groupCountY, uint32_t groupCountZ)
    /// ```
    pub unsafe fn cmd_draw_mesh_tasks(
        &self,
        command_buffer: vk::CommandBuffer,
        group_count_x: uint32_t,
        group_count_y: uint32_t,
        group_count_z: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdDrawMeshTasksEXT(
                command_buffer.abi(), 
                group_count_x.abi(), 
                group_count_y.abi(), 
                group_count_z.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdDrawMeshTasksIndirectCountEXT(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkBuffer countBuffer, VkDeviceSize countBufferOffset, uint32_t maxDrawCount, uint32_t stride)
    /// ```
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: uint32_t,
        stride: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdDrawMeshTasksIndirectCountEXT(
                command_buffer.abi(), 
                buffer.abi(), 
                offset.abi(), 
                count_buffer.abi(), 
                count_buffer_offset.abi(), 
                max_draw_count.abi(), 
                stride.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdDrawMeshTasksIndirectEXT(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, uint32_t drawCount, uint32_t stride)
    /// ```
    pub unsafe fn cmd_draw_mesh_tasks_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: DeviceSize,
        draw_count: uint32_t,
        stride: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdDrawMeshTasksIndirectEXT(
                command_buffer.abi(), 
                buffer.abi(), 
                offset.abi(), 
                draw_count.abi(), 
                stride.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::mesh_shader {
    type Commands = Device;
}

/// Device object
pub trait ExtMeshShaderDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::ext::mesh_shader {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::mesh_shader> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::mesh_shader {
        type Output = crate::hnd::Device<vk::extensions::ext::mesh_shader>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::mesh_shader>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::mesh_shader> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::mesh_shader> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::mesh_shader> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::mesh_shader> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtMeshShaderDevice for crate::hnd::Device<vk::extensions::ext::mesh_shader> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::mesh_shader, vk::Device> for crate::hnd::Device<vk::extensions::ext::mesh_shader> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait ExtMeshShaderCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdDrawMeshTasksEXT(VkCommandBuffer commandBuffer, uint32_t groupCountX, uint32_t groupCountY, uint32_t groupCountZ)
    /// ```
    unsafe fn draw_mesh_tasks(
        &self,
        group_count_x: uint32_t,
        group_count_y: uint32_t,
        group_count_z: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_draw_mesh_tasks(
                self.raw(),
                group_count_x,
                group_count_y,
                group_count_z,
            )
        }
    }
    /// ```c
    /// void vkCmdDrawMeshTasksIndirectCountEXT(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkBuffer countBuffer, VkDeviceSize countBufferOffset, uint32_t maxDrawCount, uint32_t stride)
    /// ```
    unsafe fn draw_mesh_tasks_indirect_count(
        &self,
        buffer: vk::Buffer,
        offset: DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: uint32_t,
        stride: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_draw_mesh_tasks_indirect_count(
                self.raw(),
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            )
        }
    }
    /// ```c
    /// void vkCmdDrawMeshTasksIndirectEXT(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, uint32_t drawCount, uint32_t stride)
    /// ```
    unsafe fn draw_mesh_tasks_indirect(
        &self,
        buffer: vk::Buffer,
        offset: DeviceSize,
        draw_count: uint32_t,
        stride: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_draw_mesh_tasks_indirect(
                self.raw(),
                buffer,
                offset,
                draw_count,
                stride,
            )
        }
    }
}
