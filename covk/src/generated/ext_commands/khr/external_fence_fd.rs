// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_external_fence_fd` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::external_fence_fd::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::external_fence_fd::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkGetFenceFdKHR(VkDevice device, VkFenceGetFdInfoKHR const* pGetFdInfo, int* pFd)
    /// ```
    pub unsafe fn get_fence_fd(
        &self,
        device: vk::Device,
        get_fd_info: &FenceGetFdInfoKHR,
    ) -> crate::Result<int> {
        unsafe {
            let mut _v: int = Default::default();
            let _r = self.0.GetFenceFdKHR(
                device.abi(), 
                get_fd_info.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
    /// ```c
    /// VkResult vkImportFenceFdKHR(VkDevice device, VkImportFenceFdInfoKHR const* pImportFenceFdInfo)
    /// ```
    pub unsafe fn import_fence_fd(
        &self,
        device: vk::Device,
        import_fence_fd_info: &ImportFenceFdInfoKHR,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.ImportFenceFdKHR(
                device.abi(), 
                import_fence_fd_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::external_fence_fd {
    type Commands = Device;
}

/// Device object
pub trait KhrExternalFenceFdDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetFenceFdKHR(VkDevice device, VkFenceGetFdInfoKHR const* pGetFdInfo, int* pFd)
    /// ```
    unsafe fn get_fence_fd(
        &self,
        get_fd_info: &FenceGetFdInfoKHR,
    ) -> crate::Result<int> {
        unsafe {
            self.commands().get_fence_fd(
                self.raw(),
                get_fd_info,
            )
        }
    }
    /// ```c
    /// VkResult vkImportFenceFdKHR(VkDevice device, VkImportFenceFdInfoKHR const* pImportFenceFdInfo)
    /// ```
    unsafe fn import_fence_fd(
        &self,
        import_fence_fd_info: &ImportFenceFdInfoKHR,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().import_fence_fd(
                self.raw(),
                import_fence_fd_info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::external_fence_fd {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::external_fence_fd> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::external_fence_fd {
        type Output = crate::hnd::Device<vk::extensions::khr::external_fence_fd>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::external_fence_fd>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::external_fence_fd> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::external_fence_fd> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::external_fence_fd> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::external_fence_fd> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrExternalFenceFdDevice for crate::hnd::Device<vk::extensions::khr::external_fence_fd> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::external_fence_fd, vk::Device> for crate::hnd::Device<vk::extensions::khr::external_fence_fd> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
