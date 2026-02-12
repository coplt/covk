// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_maintenance1` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::maintenance1::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::maintenance1::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkTrimCommandPool(VkDevice device, VkCommandPool commandPool, VkCommandPoolTrimFlags flags)
    /// ```
    pub unsafe fn trim_command_pool(
        &self,
        device: vk::Device,
        command_pool: vk::CommandPool,
        flags: CommandPoolTrimFlags,
    ) -> () {
        unsafe {
            self.0.TrimCommandPoolKHR(
                device.abi(), 
                command_pool.abi(), 
                flags.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::maintenance1 {
    type Commands = Device;
}

/// Device object
pub trait KhrMaintenance1Device {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkTrimCommandPool(VkDevice device, VkCommandPool commandPool, VkCommandPoolTrimFlags flags)
    /// ```
    unsafe fn trim_command_pool(
        &self,
        command_pool: vk::CommandPool,
        flags: CommandPoolTrimFlags,
    ) -> () {
        unsafe {
            self.commands().trim_command_pool(
                self.raw(),
                command_pool,
                flags,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::maintenance1 {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::maintenance1> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::maintenance1 {
        type Output = crate::hnd::Device<vk::extensions::khr::maintenance1>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::maintenance1>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::maintenance1> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::maintenance1> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::maintenance1> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::maintenance1> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrMaintenance1Device for crate::hnd::Device<vk::extensions::khr::maintenance1> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::maintenance1, vk::Device> for crate::hnd::Device<vk::extensions::khr::maintenance1> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
