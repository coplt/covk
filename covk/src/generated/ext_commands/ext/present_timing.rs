// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_present_timing` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::present_timing::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::present_timing::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkGetPastPresentationTimingEXT(VkDevice device, VkPastPresentationTimingInfoEXT const* pPastPresentationTimingInfo, VkPastPresentationTimingPropertiesEXT* pPastPresentationTimingProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_past_presentation_timing(
        &self,
        device: vk::Device,
        past_presentation_timing_info: &PastPresentationTimingInfoEXT,
        past_presentation_timing_properties: &mut PastPresentationTimingPropertiesEXT,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.GetPastPresentationTimingEXT(
                device.abi(), 
                past_presentation_timing_info.abi(), 
                past_presentation_timing_properties.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// VkResult vkGetSwapchainTimeDomainPropertiesEXT(VkDevice device, VkSwapchainKHR swapchain, VkSwapchainTimeDomainPropertiesEXT* pSwapchainTimeDomainProperties, uint64_t* pTimeDomainsCounter)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_swapchain_time_domain_properties(
        &self,
        device: vk::Device,
        swapchain: vk::SwapchainKHR,
        swapchain_time_domain_properties: &mut SwapchainTimeDomainPropertiesEXT,
    ) -> crate::Result<(uint64_t, Result)> {
        unsafe {
            let mut _v: uint64_t = Default::default();
            let _r = self.0.GetSwapchainTimeDomainPropertiesEXT(
                device.abi(), 
                swapchain.abi(), 
                swapchain_time_domain_properties.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result_multi(|| Some(_v))
        }
    }
    /// ```c
    /// VkResult vkGetSwapchainTimingPropertiesEXT(VkDevice device, VkSwapchainKHR swapchain, VkSwapchainTimingPropertiesEXT* pSwapchainTimingProperties, uint64_t* pSwapchainTimingPropertiesCounter)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::NotReady]
    pub unsafe fn get_swapchain_timing_properties(
        &self,
        device: vk::Device,
        swapchain: vk::SwapchainKHR,
        swapchain_timing_properties: &mut SwapchainTimingPropertiesEXT,
    ) -> crate::Result<(uint64_t, Result)> {
        unsafe {
            let mut _v: uint64_t = Default::default();
            let _r = self.0.GetSwapchainTimingPropertiesEXT(
                device.abi(), 
                swapchain.abi(), 
                swapchain_timing_properties.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result_multi(|| Some(_v))
        }
    }
    /// ```c
    /// VkResult vkSetSwapchainPresentTimingQueueSizeEXT(VkDevice device, VkSwapchainKHR swapchain, uint32_t size)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::NotReady]
    pub unsafe fn set_swapchain_present_timing_queue_size(
        &self,
        device: vk::Device,
        swapchain: vk::SwapchainKHR,
        size: uint32_t,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.SetSwapchainPresentTimingQueueSizeEXT(
                device.abi(), 
                swapchain.abi(), 
                size.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::present_timing {
    type Commands = Device;
}

/// Device object
pub trait ExtPresentTimingDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetPastPresentationTimingEXT(VkDevice device, VkPastPresentationTimingInfoEXT const* pPastPresentationTimingInfo, VkPastPresentationTimingPropertiesEXT* pPastPresentationTimingProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_past_presentation_timing(
        &self,
        past_presentation_timing_info: &PastPresentationTimingInfoEXT,
        past_presentation_timing_properties: &mut PastPresentationTimingPropertiesEXT,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().get_past_presentation_timing(
                self.raw(),
                past_presentation_timing_info,
                past_presentation_timing_properties,
            )
        }
    }
    /// ```c
    /// VkResult vkGetSwapchainTimeDomainPropertiesEXT(VkDevice device, VkSwapchainKHR swapchain, VkSwapchainTimeDomainPropertiesEXT* pSwapchainTimeDomainProperties, uint64_t* pTimeDomainsCounter)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_swapchain_time_domain_properties(
        &self,
        swapchain: vk::SwapchainKHR,
        swapchain_time_domain_properties: &mut SwapchainTimeDomainPropertiesEXT,
    ) -> crate::Result<(uint64_t, Result)> {
        unsafe {
            self.commands().get_swapchain_time_domain_properties(
                self.raw(),
                swapchain,
                swapchain_time_domain_properties,
            )
        }
    }
    /// ```c
    /// VkResult vkGetSwapchainTimingPropertiesEXT(VkDevice device, VkSwapchainKHR swapchain, VkSwapchainTimingPropertiesEXT* pSwapchainTimingProperties, uint64_t* pSwapchainTimingPropertiesCounter)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::NotReady]
    unsafe fn get_swapchain_timing_properties(
        &self,
        swapchain: vk::SwapchainKHR,
        swapchain_timing_properties: &mut SwapchainTimingPropertiesEXT,
    ) -> crate::Result<(uint64_t, Result)> {
        unsafe {
            self.commands().get_swapchain_timing_properties(
                self.raw(),
                swapchain,
                swapchain_timing_properties,
            )
        }
    }
    /// ```c
    /// VkResult vkSetSwapchainPresentTimingQueueSizeEXT(VkDevice device, VkSwapchainKHR swapchain, uint32_t size)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::NotReady]
    unsafe fn set_swapchain_present_timing_queue_size(
        &self,
        swapchain: vk::SwapchainKHR,
        size: uint32_t,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().set_swapchain_present_timing_queue_size(
                self.raw(),
                swapchain,
                size,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::ext::present_timing {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::present_timing> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::present_timing {
        type Output = crate::hnd::Device<vk::extensions::ext::present_timing>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::present_timing>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::present_timing> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::present_timing> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::present_timing> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::present_timing> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtPresentTimingDevice for crate::hnd::Device<vk::extensions::ext::present_timing> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::present_timing, vk::Device> for crate::hnd::Device<vk::extensions::ext::present_timing> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
