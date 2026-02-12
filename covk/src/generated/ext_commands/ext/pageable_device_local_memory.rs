// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_pageable_device_local_memory` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::pageable_device_local_memory::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::pageable_device_local_memory::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkSetDeviceMemoryPriorityEXT(VkDevice device, VkDeviceMemory memory, float priority)
    /// ```
    pub unsafe fn set_device_memory_priority(
        &self,
        device: vk::Device,
        memory: vk::DeviceMemory,
        priority: float,
    ) -> () {
        unsafe {
            self.0.SetDeviceMemoryPriorityEXT(
                device.abi(), 
                memory.abi(), 
                priority.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::pageable_device_local_memory {
    type Commands = Device;
}

/// Device object
pub trait ExtPageableDeviceLocalMemoryDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkSetDeviceMemoryPriorityEXT(VkDevice device, VkDeviceMemory memory, float priority)
    /// ```
    unsafe fn set_device_memory_priority(
        &self,
        memory: vk::DeviceMemory,
        priority: float,
    ) -> () {
        unsafe {
            self.commands().set_device_memory_priority(
                self.raw(),
                memory,
                priority,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::ext::pageable_device_local_memory {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::pageable_device_local_memory> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::pageable_device_local_memory {
        type Output = crate::hnd::Device<vk::extensions::ext::pageable_device_local_memory>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::pageable_device_local_memory>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::pageable_device_local_memory> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::pageable_device_local_memory> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::pageable_device_local_memory> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::pageable_device_local_memory> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtPageableDeviceLocalMemoryDevice for crate::hnd::Device<vk::extensions::ext::pageable_device_local_memory> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::pageable_device_local_memory, vk::Device> for crate::hnd::Device<vk::extensions::ext::pageable_device_local_memory> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
