// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_present_wait2` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::present_wait2::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::present_wait2::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkWaitForPresent2KHR(VkDevice device, VkSwapchainKHR swapchain, VkPresentWait2InfoKHR const* pPresentWait2Info)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Timeout], [Result::SuboptimalKhr]
    pub unsafe fn wait_for_present2(
        &self,
        device: vk::Device,
        swapchain: vk::SwapchainKHR,
        present_wait2info: &PresentWait2InfoKHR,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.WaitForPresent2KHR(
                device.abi(), 
                swapchain.abi(), 
                present_wait2info.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::present_wait2 {
    type Commands = Device;
}

/// Device object
pub trait KhrPresentWait2Device {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkWaitForPresent2KHR(VkDevice device, VkSwapchainKHR swapchain, VkPresentWait2InfoKHR const* pPresentWait2Info)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Timeout], [Result::SuboptimalKhr]
    unsafe fn wait_for_present2(
        &self,
        swapchain: vk::SwapchainKHR,
        present_wait2info: &PresentWait2InfoKHR,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().wait_for_present2(
                self.raw(),
                swapchain,
                present_wait2info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::present_wait2 {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::present_wait2> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::present_wait2 {
        type Output = crate::hnd::Device<vk::extensions::khr::present_wait2>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::present_wait2>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::present_wait2> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::present_wait2> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::present_wait2> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::present_wait2> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrPresentWait2Device for crate::hnd::Device<vk::extensions::khr::present_wait2> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::present_wait2, vk::Device> for crate::hnd::Device<vk::extensions::khr::present_wait2> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
