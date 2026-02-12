// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_display_control` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::display_control::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::display_control::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkDisplayPowerControlEXT(VkDevice device, VkDisplayKHR display, VkDisplayPowerInfoEXT const* pDisplayPowerInfo)
    /// ```
    pub unsafe fn display_power_control(
        &self,
        device: vk::Device,
        display: vk::DisplayKHR,
        display_power_info: &DisplayPowerInfoEXT,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.DisplayPowerControlEXT(
                device.abi(), 
                display.abi(), 
                display_power_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkGetSwapchainCounterEXT(VkDevice device, VkSwapchainKHR swapchain, VkSurfaceCounterFlagBitsEXT counter, uint64_t* pCounterValue)
    /// ```
    pub unsafe fn get_swapchain_counter(
        &self,
        device: vk::Device,
        swapchain: vk::SwapchainKHR,
        counter: SurfaceCounterFlagsEXT,
    ) -> crate::Result<uint64_t> {
        unsafe {
            let mut _v: uint64_t = Default::default();
            let _r = self.0.GetSwapchainCounterEXT(
                device.abi(), 
                swapchain.abi(), 
                counter.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
    /// ```c
    /// VkResult vkRegisterDeviceEventEXT(VkDevice device, VkDeviceEventInfoEXT const* pDeviceEventInfo, VkAllocationCallbacks const* pAllocator, VkFence* pFence)
    /// ```
    pub unsafe fn register_device_event(
        &self,
        device: vk::Device,
        device_event_info: &DeviceEventInfoEXT,
    ) -> crate::Result<vk::Fence> {
        unsafe {
            let mut _v: Option<vk::Fence> = Default::default();
            let _r = self.0.RegisterDeviceEventEXT(
                device.abi(), 
                device_event_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkRegisterDisplayEventEXT(VkDevice device, VkDisplayKHR display, VkDisplayEventInfoEXT const* pDisplayEventInfo, VkAllocationCallbacks const* pAllocator, VkFence* pFence)
    /// ```
    pub unsafe fn register_display_event(
        &self,
        device: vk::Device,
        display: vk::DisplayKHR,
        display_event_info: &DisplayEventInfoEXT,
    ) -> crate::Result<vk::Fence> {
        unsafe {
            let mut _v: Option<vk::Fence> = Default::default();
            let _r = self.0.RegisterDisplayEventEXT(
                device.abi(), 
                display.abi(), 
                display_event_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::display_control {
    type Commands = Device;
}

/// Device object
pub trait ExtDisplayControlDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkDisplayPowerControlEXT(VkDevice device, VkDisplayKHR display, VkDisplayPowerInfoEXT const* pDisplayPowerInfo)
    /// ```
    unsafe fn display_power_control(
        &self,
        display: vk::DisplayKHR,
        display_power_info: &DisplayPowerInfoEXT,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().display_power_control(
                self.raw(),
                display,
                display_power_info,
            )
        }
    }
    /// ```c
    /// VkResult vkGetSwapchainCounterEXT(VkDevice device, VkSwapchainKHR swapchain, VkSurfaceCounterFlagBitsEXT counter, uint64_t* pCounterValue)
    /// ```
    unsafe fn get_swapchain_counter(
        &self,
        swapchain: vk::SwapchainKHR,
        counter: SurfaceCounterFlagsEXT,
    ) -> crate::Result<uint64_t> {
        unsafe {
            self.commands().get_swapchain_counter(
                self.raw(),
                swapchain,
                counter,
            )
        }
    }
    /// ```c
    /// VkResult vkRegisterDeviceEventEXT(VkDevice device, VkDeviceEventInfoEXT const* pDeviceEventInfo, VkAllocationCallbacks const* pAllocator, VkFence* pFence)
    /// ```
    unsafe fn register_device_event(
        &self,
        device_event_info: &DeviceEventInfoEXT,
    ) -> crate::Result<vk::Fence> {
        unsafe {
            self.commands().register_device_event(
                self.raw(),
                device_event_info,
            )
        }
    }
    /// ```c
    /// VkResult vkRegisterDisplayEventEXT(VkDevice device, VkDisplayKHR display, VkDisplayEventInfoEXT const* pDisplayEventInfo, VkAllocationCallbacks const* pAllocator, VkFence* pFence)
    /// ```
    unsafe fn register_display_event(
        &self,
        display: vk::DisplayKHR,
        display_event_info: &DisplayEventInfoEXT,
    ) -> crate::Result<vk::Fence> {
        unsafe {
            self.commands().register_display_event(
                self.raw(),
                display,
                display_event_info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::ext::display_control {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::display_control> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::display_control {
        type Output = crate::hnd::Device<vk::extensions::ext::display_control>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::display_control>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::display_control> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::display_control> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::display_control> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::display_control> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtDisplayControlDevice for crate::hnd::Device<vk::extensions::ext::display_control> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::display_control, vk::Device> for crate::hnd::Device<vk::extensions::ext::display_control> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
