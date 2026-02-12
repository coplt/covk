// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_HUAWEI_cluster_culling_shader` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::huawei::cluster_culling_shader::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::huawei::cluster_culling_shader::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdDrawClusterHUAWEI(VkCommandBuffer commandBuffer, uint32_t groupCountX, uint32_t groupCountY, uint32_t groupCountZ)
    /// ```
    pub unsafe fn cmd_draw_cluster(
        &self,
        command_buffer: vk::CommandBuffer,
        group_count_x: uint32_t,
        group_count_y: uint32_t,
        group_count_z: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdDrawClusterHUAWEI(
                command_buffer.abi(), 
                group_count_x.abi(), 
                group_count_y.abi(), 
                group_count_z.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdDrawClusterIndirectHUAWEI(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset)
    /// ```
    pub unsafe fn cmd_draw_cluster_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: DeviceSize,
    ) -> () {
        unsafe {
            self.0.CmdDrawClusterIndirectHUAWEI(
                command_buffer.abi(), 
                buffer.abi(), 
                offset.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::huawei::cluster_culling_shader {
    type Commands = Device;
}

/// Device object
pub trait HuaweiClusterCullingShaderDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::huawei::cluster_culling_shader {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::huawei::cluster_culling_shader> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::huawei::cluster_culling_shader {
        type Output = crate::hnd::Device<vk::extensions::huawei::cluster_culling_shader>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::huawei::cluster_culling_shader>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::huawei::cluster_culling_shader> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::huawei::cluster_culling_shader> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::huawei::cluster_culling_shader> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::huawei::cluster_culling_shader> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::HuaweiClusterCullingShaderDevice for crate::hnd::Device<vk::extensions::huawei::cluster_culling_shader> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::huawei::cluster_culling_shader, vk::Device> for crate::hnd::Device<vk::extensions::huawei::cluster_culling_shader> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait HuaweiClusterCullingShaderCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdDrawClusterHUAWEI(VkCommandBuffer commandBuffer, uint32_t groupCountX, uint32_t groupCountY, uint32_t groupCountZ)
    /// ```
    unsafe fn draw_cluster(
        &self,
        group_count_x: uint32_t,
        group_count_y: uint32_t,
        group_count_z: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_draw_cluster(
                self.raw(),
                group_count_x,
                group_count_y,
                group_count_z,
            )
        }
    }
    /// ```c
    /// void vkCmdDrawClusterIndirectHUAWEI(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset)
    /// ```
    unsafe fn draw_cluster_indirect(
        &self,
        buffer: vk::Buffer,
        offset: DeviceSize,
    ) -> () {
        unsafe {
            self.commands().cmd_draw_cluster_indirect(
                self.raw(),
                buffer,
                offset,
            )
        }
    }
}
