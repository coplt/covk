// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_NV_device_diagnostic_checkpoints` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::nv::device_diagnostic_checkpoints::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::device_diagnostic_checkpoints::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdSetCheckpointNV(VkCommandBuffer commandBuffer, void const* pCheckpointMarker)
    /// ```
    pub unsafe fn cmd_set_checkpoint(
        &self,
        command_buffer: vk::CommandBuffer,
        checkpoint_marker: *const void,
    ) -> () {
        unsafe {
            self.0.CmdSetCheckpointNV(
                command_buffer.abi(), 
                checkpoint_marker.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetQueueCheckpointData2NV(VkQueue queue, uint32_t* pCheckpointDataCount, VkCheckpointData2NV* pCheckpointData)
    /// ```
    pub unsafe fn get_queue_checkpoint_data2(
        &self,
        queue: vk::Queue,
        checkpoint_data: Option<&mut ::alloc::vec::Vec<CheckpointData2NV>>,
    ) -> uint32_t {
        unsafe {
            let mut _c: uint32_t = Default::default();
            self.0.GetQueueCheckpointData2NV(
                queue.abi(), 
                &mut _c, 
                Default::default(), 
            );
            if let Some(_b) = checkpoint_data {
                _b.reserve(_c as usize);
                self.0.GetQueueCheckpointData2NV(
                    queue.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                );
                _b.set_len(_b.len() + _c as usize);
            }
            _c
        }
    }
    /// ```c
    /// void vkGetQueueCheckpointDataNV(VkQueue queue, uint32_t* pCheckpointDataCount, VkCheckpointDataNV* pCheckpointData)
    /// ```
    pub unsafe fn get_queue_checkpoint_data(
        &self,
        queue: vk::Queue,
        checkpoint_data: Option<&mut ::alloc::vec::Vec<CheckpointDataNV>>,
    ) -> uint32_t {
        unsafe {
            let mut _c: uint32_t = Default::default();
            self.0.GetQueueCheckpointDataNV(
                queue.abi(), 
                &mut _c, 
                Default::default(), 
            );
            if let Some(_b) = checkpoint_data {
                _b.reserve(_c as usize);
                self.0.GetQueueCheckpointDataNV(
                    queue.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                );
                _b.set_len(_b.len() + _c as usize);
            }
            _c
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::nv::device_diagnostic_checkpoints {
    type Commands = Device;
}

/// Device object
pub trait NvDeviceDiagnosticCheckpointsDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::nv::device_diagnostic_checkpoints {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::nv::device_diagnostic_checkpoints> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::nv::device_diagnostic_checkpoints {
        type Output = crate::hnd::Device<vk::extensions::nv::device_diagnostic_checkpoints>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::nv::device_diagnostic_checkpoints>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::nv::device_diagnostic_checkpoints> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::nv::device_diagnostic_checkpoints> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::nv::device_diagnostic_checkpoints> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::nv::device_diagnostic_checkpoints> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvDeviceDiagnosticCheckpointsDevice for crate::hnd::Device<vk::extensions::nv::device_diagnostic_checkpoints> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::device_diagnostic_checkpoints, vk::Device> for crate::hnd::Device<vk::extensions::nv::device_diagnostic_checkpoints> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait NvDeviceDiagnosticCheckpointsCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdSetCheckpointNV(VkCommandBuffer commandBuffer, void const* pCheckpointMarker)
    /// ```
    unsafe fn set_checkpoint(
        &self,
        checkpoint_marker: *const void,
    ) -> () {
        unsafe {
            self.commands().cmd_set_checkpoint(
                self.raw(),
                checkpoint_marker,
            )
        }
    }
}

/// Queue object
pub trait NvDeviceDiagnosticCheckpointsQueue {
    fn raw(&self) -> vk::Queue;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkGetQueueCheckpointData2NV(VkQueue queue, uint32_t* pCheckpointDataCount, VkCheckpointData2NV* pCheckpointData)
    /// ```
    unsafe fn get_queue_checkpoint_data2(
        &self,
        checkpoint_data: Option<&mut ::alloc::vec::Vec<CheckpointData2NV>>,
    ) -> uint32_t {
        unsafe {
            self.commands().get_queue_checkpoint_data2(
                self.raw(),
                checkpoint_data,
            )
        }
    }
    /// ```c
    /// void vkGetQueueCheckpointDataNV(VkQueue queue, uint32_t* pCheckpointDataCount, VkCheckpointDataNV* pCheckpointData)
    /// ```
    unsafe fn get_queue_checkpoint_data(
        &self,
        checkpoint_data: Option<&mut ::alloc::vec::Vec<CheckpointDataNV>>,
    ) -> uint32_t {
        unsafe {
            self.commands().get_queue_checkpoint_data(
                self.raw(),
                checkpoint_data,
            )
        }
    }
}
