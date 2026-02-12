// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_GOOGLE_display_timing` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::google::display_timing::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::google::display_timing::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkGetPastPresentationTimingGOOGLE(VkDevice device, VkSwapchainKHR swapchain, uint32_t* pPresentationTimingCount, VkPastPresentationTimingGOOGLE* pPresentationTimings)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_past_presentation_timing(
        &self,
        device: vk::Device,
        swapchain: vk::SwapchainKHR,
        presentation_timings: Option<&mut ::alloc::vec::Vec<PastPresentationTimingGOOGLE>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetPastPresentationTimingGOOGLE(
                device.abi(), 
                swapchain.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = presentation_timings {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetPastPresentationTimingGOOGLE(
                    device.abi(), 
                    swapchain.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                ).vk();
                _r.result(|| Some(()))?;
                _b.set_len(_b.len() + _c as usize);
            }
            _r.result_multi(|| Some(_c))
        }
    }
    /// ```c
    /// VkResult vkGetRefreshCycleDurationGOOGLE(VkDevice device, VkSwapchainKHR swapchain, VkRefreshCycleDurationGOOGLE* pDisplayTimingProperties)
    /// ```
    pub unsafe fn get_refresh_cycle_duration(
        &self,
        device: vk::Device,
        swapchain: vk::SwapchainKHR,
    ) -> crate::Result<RefreshCycleDurationGOOGLE> {
        unsafe {
            let mut _v: RefreshCycleDurationGOOGLE = Default::default();
            let _r = self.0.GetRefreshCycleDurationGOOGLE(
                device.abi(), 
                swapchain.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::google::display_timing {
    type Commands = Device;
}

/// Device object
pub trait GoogleDisplayTimingDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetPastPresentationTimingGOOGLE(VkDevice device, VkSwapchainKHR swapchain, uint32_t* pPresentationTimingCount, VkPastPresentationTimingGOOGLE* pPresentationTimings)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_past_presentation_timing(
        &self,
        swapchain: vk::SwapchainKHR,
        presentation_timings: Option<&mut ::alloc::vec::Vec<PastPresentationTimingGOOGLE>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_past_presentation_timing(
                self.raw(),
                swapchain,
                presentation_timings,
            )
        }
    }
    /// ```c
    /// VkResult vkGetRefreshCycleDurationGOOGLE(VkDevice device, VkSwapchainKHR swapchain, VkRefreshCycleDurationGOOGLE* pDisplayTimingProperties)
    /// ```
    unsafe fn get_refresh_cycle_duration(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> crate::Result<RefreshCycleDurationGOOGLE> {
        unsafe {
            self.commands().get_refresh_cycle_duration(
                self.raw(),
                swapchain,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::google::display_timing {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::google::display_timing> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::google::display_timing {
        type Output = crate::hnd::Device<vk::extensions::google::display_timing>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::google::display_timing>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::google::display_timing> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::google::display_timing> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::google::display_timing> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::google::display_timing> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::GoogleDisplayTimingDevice for crate::hnd::Device<vk::extensions::google::display_timing> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::google::display_timing, vk::Device> for crate::hnd::Device<vk::extensions::google::display_timing> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
