// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_display_swapchain` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::display_swapchain::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::display_swapchain::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkCreateSharedSwapchainsKHR(VkDevice device, uint32_t swapchainCount, VkSwapchainCreateInfoKHR const* pCreateInfos, VkAllocationCallbacks const* pAllocator, VkSwapchainKHR* pSwapchains)
    /// ```
    pub unsafe fn create_shared_swapchains(
        &self,
        device: vk::Device,
        create_infos: &[SwapchainCreateInfoKHR],
        swapchains: &mut [Option<vk::SwapchainKHR>],
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.CreateSharedSwapchainsKHR(
                device.abi(), 
                create_infos.len() as _, 
                create_infos.abi(), 
                Default::default(), 
                swapchains.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::display_swapchain {
    type Commands = Device;
}

/// Device object
pub trait KhrDisplaySwapchainDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkCreateSharedSwapchainsKHR(VkDevice device, uint32_t swapchainCount, VkSwapchainCreateInfoKHR const* pCreateInfos, VkAllocationCallbacks const* pAllocator, VkSwapchainKHR* pSwapchains)
    /// ```
    unsafe fn create_shared_swapchains(
        &self,
        create_infos: &[SwapchainCreateInfoKHR],
        swapchains: &mut [Option<vk::SwapchainKHR>],
    ) -> crate::Result<()> {
        unsafe {
            self.commands().create_shared_swapchains(
                self.raw(),
                create_infos,
                swapchains,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::display_swapchain {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::display_swapchain> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::display_swapchain {
        type Output = crate::hnd::Device<vk::extensions::khr::display_swapchain>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::display_swapchain>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::display_swapchain> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::display_swapchain> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::display_swapchain> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::display_swapchain> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrDisplaySwapchainDevice for crate::hnd::Device<vk::extensions::khr::display_swapchain> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::display_swapchain, vk::Device> for crate::hnd::Device<vk::extensions::khr::display_swapchain> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
