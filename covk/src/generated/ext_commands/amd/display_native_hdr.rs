// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_AMD_display_native_hdr` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::amd::display_native_hdr::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::amd::display_native_hdr::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkSetLocalDimmingAMD(VkDevice device, VkSwapchainKHR swapChain, VkBool32 localDimmingEnable)
    /// ```
    pub unsafe fn set_local_dimming(
        &self,
        device: vk::Device,
        swap_chain: vk::SwapchainKHR,
        local_dimming_enable: bool,
    ) -> () {
        unsafe {
            self.0.SetLocalDimmingAMD(
                device.abi(), 
                swap_chain.abi(), 
                local_dimming_enable.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::amd::display_native_hdr {
    type Commands = Device;
}

/// Device object
pub trait AmdDisplayNativeHdrDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkSetLocalDimmingAMD(VkDevice device, VkSwapchainKHR swapChain, VkBool32 localDimmingEnable)
    /// ```
    unsafe fn set_local_dimming(
        &self,
        swap_chain: vk::SwapchainKHR,
        local_dimming_enable: bool,
    ) -> () {
        unsafe {
            self.commands().set_local_dimming(
                self.raw(),
                swap_chain,
                local_dimming_enable,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::amd::display_native_hdr {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::amd::display_native_hdr> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::amd::display_native_hdr {
        type Output = crate::hnd::Device<vk::extensions::amd::display_native_hdr>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::amd::display_native_hdr>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::amd::display_native_hdr> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::amd::display_native_hdr> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::amd::display_native_hdr> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::amd::display_native_hdr> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::AmdDisplayNativeHdrDevice for crate::hnd::Device<vk::extensions::amd::display_native_hdr> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::amd::display_native_hdr, vk::Device> for crate::hnd::Device<vk::extensions::amd::display_native_hdr> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
