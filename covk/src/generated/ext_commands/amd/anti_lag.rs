// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_AMD_anti_lag` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::amd::anti_lag::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::amd::anti_lag::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkAntiLagUpdateAMD(VkDevice device, VkAntiLagDataAMD const* pData)
    /// ```
    pub unsafe fn anti_lag_update(
        &self,
        device: vk::Device,
        data: &AntiLagDataAMD,
    ) -> () {
        unsafe {
            self.0.AntiLagUpdateAMD(
                device.abi(), 
                data.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::amd::anti_lag {
    type Commands = Device;
}

/// Device object
pub trait AmdAntiLagDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkAntiLagUpdateAMD(VkDevice device, VkAntiLagDataAMD const* pData)
    /// ```
    unsafe fn anti_lag_update(
        &self,
        data: &AntiLagDataAMD,
    ) -> () {
        unsafe {
            self.commands().anti_lag_update(
                self.raw(),
                data,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::amd::anti_lag {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::amd::anti_lag> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::amd::anti_lag {
        type Output = crate::hnd::Device<vk::extensions::amd::anti_lag>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::amd::anti_lag>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::amd::anti_lag> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::amd::anti_lag> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::amd::anti_lag> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::amd::anti_lag> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::AmdAntiLagDevice for crate::hnd::Device<vk::extensions::amd::anti_lag> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::amd::anti_lag, vk::Device> for crate::hnd::Device<vk::extensions::amd::anti_lag> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
