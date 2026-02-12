// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_shared_presentable_image` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::shared_presentable_image::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::shared_presentable_image::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkGetSwapchainStatusKHR(VkDevice device, VkSwapchainKHR swapchain)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::SuboptimalKhr]
    pub unsafe fn get_swapchain_status(
        &self,
        device: vk::Device,
        swapchain: vk::SwapchainKHR,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.GetSwapchainStatusKHR(
                device.abi(), 
                swapchain.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::shared_presentable_image {
    type Commands = Device;
}

/// Device object
pub trait KhrSharedPresentableImageDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetSwapchainStatusKHR(VkDevice device, VkSwapchainKHR swapchain)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::SuboptimalKhr]
    unsafe fn get_swapchain_status(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().get_swapchain_status(
                self.raw(),
                swapchain,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::shared_presentable_image {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::shared_presentable_image> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::shared_presentable_image {
        type Output = crate::hnd::Device<vk::extensions::khr::shared_presentable_image>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::shared_presentable_image>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::shared_presentable_image> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::shared_presentable_image> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::shared_presentable_image> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::shared_presentable_image> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrSharedPresentableImageDevice for crate::hnd::Device<vk::extensions::khr::shared_presentable_image> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::shared_presentable_image, vk::Device> for crate::hnd::Device<vk::extensions::khr::shared_presentable_image> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
