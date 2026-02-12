// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_host_query_reset` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::host_query_reset::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::host_query_reset::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkResetQueryPool(VkDevice device, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount)
    /// ```
    pub unsafe fn reset_query_pool(
        &self,
        device: vk::Device,
        query_pool: vk::QueryPool,
        first_query: uint32_t,
        query_count: uint32_t,
    ) -> () {
        unsafe {
            self.0.ResetQueryPoolEXT(
                device.abi(), 
                query_pool.abi(), 
                first_query.abi(), 
                query_count.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::host_query_reset {
    type Commands = Device;
}

/// Device object
pub trait ExtHostQueryResetDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkResetQueryPool(VkDevice device, VkQueryPool queryPool, uint32_t firstQuery, uint32_t queryCount)
    /// ```
    unsafe fn reset_query_pool(
        &self,
        query_pool: vk::QueryPool,
        first_query: uint32_t,
        query_count: uint32_t,
    ) -> () {
        unsafe {
            self.commands().reset_query_pool(
                self.raw(),
                query_pool,
                first_query,
                query_count,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::ext::host_query_reset {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::host_query_reset> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::host_query_reset {
        type Output = crate::hnd::Device<vk::extensions::ext::host_query_reset>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::host_query_reset>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::host_query_reset> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::host_query_reset> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::host_query_reset> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::host_query_reset> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtHostQueryResetDevice for crate::hnd::Device<vk::extensions::ext::host_query_reset> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::host_query_reset, vk::Device> for crate::hnd::Device<vk::extensions::ext::host_query_reset> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
