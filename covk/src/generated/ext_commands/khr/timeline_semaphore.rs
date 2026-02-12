// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_timeline_semaphore` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::timeline_semaphore::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::timeline_semaphore::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkGetSemaphoreCounterValue(VkDevice device, VkSemaphore semaphore, uint64_t* pValue)
    /// ```
    pub unsafe fn get_semaphore_counter_value(
        &self,
        device: vk::Device,
        semaphore: vk::Semaphore,
    ) -> crate::Result<uint64_t> {
        unsafe {
            let mut _v: uint64_t = Default::default();
            let _r = self.0.GetSemaphoreCounterValueKHR(
                device.abi(), 
                semaphore.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
    /// ```c
    /// VkResult vkSignalSemaphore(VkDevice device, VkSemaphoreSignalInfo const* pSignalInfo)
    /// ```
    pub unsafe fn signal_semaphore(
        &self,
        device: vk::Device,
        signal_info: &SemaphoreSignalInfo,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.SignalSemaphoreKHR(
                device.abi(), 
                signal_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkWaitSemaphores(VkDevice device, VkSemaphoreWaitInfo const* pWaitInfo, uint64_t timeout)
    /// ```
    pub unsafe fn wait_semaphores(
        &self,
        device: vk::Device,
        wait_info: &SemaphoreWaitInfo,
        timeout: uint64_t,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.WaitSemaphoresKHR(
                device.abi(), 
                wait_info.abi(), 
                timeout.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::timeline_semaphore {
    type Commands = Device;
}

/// Device object
pub trait KhrTimelineSemaphoreDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetSemaphoreCounterValue(VkDevice device, VkSemaphore semaphore, uint64_t* pValue)
    /// ```
    unsafe fn get_semaphore_counter_value(
        &self,
        semaphore: vk::Semaphore,
    ) -> crate::Result<uint64_t> {
        unsafe {
            self.commands().get_semaphore_counter_value(
                self.raw(),
                semaphore,
            )
        }
    }
    /// ```c
    /// VkResult vkSignalSemaphore(VkDevice device, VkSemaphoreSignalInfo const* pSignalInfo)
    /// ```
    unsafe fn signal_semaphore(
        &self,
        signal_info: &SemaphoreSignalInfo,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().signal_semaphore(
                self.raw(),
                signal_info,
            )
        }
    }
    /// ```c
    /// VkResult vkWaitSemaphores(VkDevice device, VkSemaphoreWaitInfo const* pWaitInfo, uint64_t timeout)
    /// ```
    unsafe fn wait_semaphores(
        &self,
        wait_info: &SemaphoreWaitInfo,
        timeout: uint64_t,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().wait_semaphores(
                self.raw(),
                wait_info,
                timeout,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::timeline_semaphore {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::timeline_semaphore> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::timeline_semaphore {
        type Output = crate::hnd::Device<vk::extensions::khr::timeline_semaphore>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::timeline_semaphore>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::timeline_semaphore> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::timeline_semaphore> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::timeline_semaphore> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::timeline_semaphore> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrTimelineSemaphoreDevice for crate::hnd::Device<vk::extensions::khr::timeline_semaphore> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::timeline_semaphore, vk::Device> for crate::hnd::Device<vk::extensions::khr::timeline_semaphore> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
