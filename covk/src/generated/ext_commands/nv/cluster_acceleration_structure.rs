// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_NV_cluster_acceleration_structure` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::nv::cluster_acceleration_structure::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::cluster_acceleration_structure::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdBuildClusterAccelerationStructureIndirectNV(VkCommandBuffer commandBuffer, VkClusterAccelerationStructureCommandsInfoNV const* pCommandInfos)
    /// ```
    pub unsafe fn cmd_build_cluster_acceleration_structure_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        command_infos: &ClusterAccelerationStructureCommandsInfoNV,
    ) -> () {
        unsafe {
            self.0.CmdBuildClusterAccelerationStructureIndirectNV(
                command_buffer.abi(), 
                command_infos.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetClusterAccelerationStructureBuildSizesNV(VkDevice device, VkClusterAccelerationStructureInputInfoNV const* pInfo, VkAccelerationStructureBuildSizesInfoKHR* pSizeInfo)
    /// ```
    pub unsafe fn get_cluster_acceleration_structure_build_sizes(
        &self,
        device: vk::Device,
        info: &ClusterAccelerationStructureInputInfoNV,
        size_info: &mut AccelerationStructureBuildSizesInfoKHR,
    ) -> () {
        unsafe {
            self.0.GetClusterAccelerationStructureBuildSizesNV(
                device.abi(), 
                info.abi(), 
                size_info.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::nv::cluster_acceleration_structure {
    type Commands = Device;
}

/// Device object
pub trait NvClusterAccelerationStructureDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkGetClusterAccelerationStructureBuildSizesNV(VkDevice device, VkClusterAccelerationStructureInputInfoNV const* pInfo, VkAccelerationStructureBuildSizesInfoKHR* pSizeInfo)
    /// ```
    unsafe fn get_cluster_acceleration_structure_build_sizes(
        &self,
        info: &ClusterAccelerationStructureInputInfoNV,
        size_info: &mut AccelerationStructureBuildSizesInfoKHR,
    ) -> () {
        unsafe {
            self.commands().get_cluster_acceleration_structure_build_sizes(
                self.raw(),
                info,
                size_info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::nv::cluster_acceleration_structure {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::nv::cluster_acceleration_structure> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::nv::cluster_acceleration_structure {
        type Output = crate::hnd::Device<vk::extensions::nv::cluster_acceleration_structure>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::nv::cluster_acceleration_structure>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::nv::cluster_acceleration_structure> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::nv::cluster_acceleration_structure> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::nv::cluster_acceleration_structure> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::nv::cluster_acceleration_structure> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvClusterAccelerationStructureDevice for crate::hnd::Device<vk::extensions::nv::cluster_acceleration_structure> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::cluster_acceleration_structure, vk::Device> for crate::hnd::Device<vk::extensions::nv::cluster_acceleration_structure> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait NvClusterAccelerationStructureCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdBuildClusterAccelerationStructureIndirectNV(VkCommandBuffer commandBuffer, VkClusterAccelerationStructureCommandsInfoNV const* pCommandInfos)
    /// ```
    unsafe fn build_cluster_acceleration_structure_indirect(
        &self,
        command_infos: &ClusterAccelerationStructureCommandsInfoNV,
    ) -> () {
        unsafe {
            self.commands().cmd_build_cluster_acceleration_structure_indirect(
                self.raw(),
                command_infos,
            )
        }
    }
}
