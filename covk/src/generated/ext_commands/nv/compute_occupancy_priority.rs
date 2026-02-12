// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_NV_compute_occupancy_priority` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::nv::compute_occupancy_priority::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::compute_occupancy_priority::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdSetComputeOccupancyPriorityNV(VkCommandBuffer commandBuffer, VkComputeOccupancyPriorityParametersNV const* pParameters)
    /// ```
    pub unsafe fn cmd_set_compute_occupancy_priority(
        &self,
        command_buffer: vk::CommandBuffer,
        parameters: &ComputeOccupancyPriorityParametersNV,
    ) -> () {
        unsafe {
            self.0.CmdSetComputeOccupancyPriorityNV(
                command_buffer.abi(), 
                parameters.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::nv::compute_occupancy_priority {
    type Commands = Device;
}

/// Device object
pub trait NvComputeOccupancyPriorityDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::nv::compute_occupancy_priority {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::nv::compute_occupancy_priority> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::nv::compute_occupancy_priority {
        type Output = crate::hnd::Device<vk::extensions::nv::compute_occupancy_priority>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::nv::compute_occupancy_priority>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::nv::compute_occupancy_priority> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::nv::compute_occupancy_priority> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::nv::compute_occupancy_priority> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::nv::compute_occupancy_priority> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvComputeOccupancyPriorityDevice for crate::hnd::Device<vk::extensions::nv::compute_occupancy_priority> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::compute_occupancy_priority, vk::Device> for crate::hnd::Device<vk::extensions::nv::compute_occupancy_priority> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait NvComputeOccupancyPriorityCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdSetComputeOccupancyPriorityNV(VkCommandBuffer commandBuffer, VkComputeOccupancyPriorityParametersNV const* pParameters)
    /// ```
    unsafe fn set_compute_occupancy_priority(
        &self,
        parameters: &ComputeOccupancyPriorityParametersNV,
    ) -> () {
        unsafe {
            self.commands().cmd_set_compute_occupancy_priority(
                self.raw(),
                parameters,
            )
        }
    }
}
