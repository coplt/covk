// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_NV_partitioned_acceleration_structure` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::nv::partitioned_acceleration_structure::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::partitioned_acceleration_structure::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdBuildPartitionedAccelerationStructuresNV(VkCommandBuffer commandBuffer, VkBuildPartitionedAccelerationStructureInfoNV const* pBuildInfo)
    /// ```
    pub unsafe fn cmd_build_partitioned_acceleration_structures(
        &self,
        command_buffer: vk::CommandBuffer,
        build_info: &BuildPartitionedAccelerationStructureInfoNV,
    ) -> () {
        unsafe {
            self.0.CmdBuildPartitionedAccelerationStructuresNV(
                command_buffer.abi(), 
                build_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetPartitionedAccelerationStructuresBuildSizesNV(VkDevice device, VkPartitionedAccelerationStructureInstancesInputNV const* pInfo, VkAccelerationStructureBuildSizesInfoKHR* pSizeInfo)
    /// ```
    pub unsafe fn get_partitioned_acceleration_structures_build_sizes(
        &self,
        device: vk::Device,
        info: &PartitionedAccelerationStructureInstancesInputNV,
        size_info: &mut AccelerationStructureBuildSizesInfoKHR,
    ) -> () {
        unsafe {
            self.0.GetPartitionedAccelerationStructuresBuildSizesNV(
                device.abi(), 
                info.abi(), 
                size_info.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::nv::partitioned_acceleration_structure {
    type Commands = Device;
}

/// Device object
pub trait NvPartitionedAccelerationStructureDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkGetPartitionedAccelerationStructuresBuildSizesNV(VkDevice device, VkPartitionedAccelerationStructureInstancesInputNV const* pInfo, VkAccelerationStructureBuildSizesInfoKHR* pSizeInfo)
    /// ```
    unsafe fn get_partitioned_acceleration_structures_build_sizes(
        &self,
        info: &PartitionedAccelerationStructureInstancesInputNV,
        size_info: &mut AccelerationStructureBuildSizesInfoKHR,
    ) -> () {
        unsafe {
            self.commands().get_partitioned_acceleration_structures_build_sizes(
                self.raw(),
                info,
                size_info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::nv::partitioned_acceleration_structure {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::nv::partitioned_acceleration_structure> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::nv::partitioned_acceleration_structure {
        type Output = crate::hnd::Device<vk::extensions::nv::partitioned_acceleration_structure>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::nv::partitioned_acceleration_structure>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::nv::partitioned_acceleration_structure> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::nv::partitioned_acceleration_structure> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::nv::partitioned_acceleration_structure> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::nv::partitioned_acceleration_structure> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvPartitionedAccelerationStructureDevice for crate::hnd::Device<vk::extensions::nv::partitioned_acceleration_structure> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::partitioned_acceleration_structure, vk::Device> for crate::hnd::Device<vk::extensions::nv::partitioned_acceleration_structure> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait NvPartitionedAccelerationStructureCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdBuildPartitionedAccelerationStructuresNV(VkCommandBuffer commandBuffer, VkBuildPartitionedAccelerationStructureInfoNV const* pBuildInfo)
    /// ```
    unsafe fn build_partitioned_acceleration_structures(
        &self,
        build_info: &BuildPartitionedAccelerationStructureInfoNV,
    ) -> () {
        unsafe {
            self.commands().cmd_build_partitioned_acceleration_structures(
                self.raw(),
                build_info,
            )
        }
    }
}
