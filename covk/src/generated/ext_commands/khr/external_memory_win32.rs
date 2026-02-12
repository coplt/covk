// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_external_memory_win32` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::external_memory_win32::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::external_memory_win32::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkGetMemoryWin32HandleKHR(VkDevice device, VkMemoryGetWin32HandleInfoKHR const* pGetWin32HandleInfo, HANDLE* pHandle)
    /// ```
    pub unsafe fn get_memory_win32handle(
        &self,
        device: vk::Device,
        get_win32handle_info: &MemoryGetWin32HandleInfoKHR,
    ) -> crate::Result<HANDLE> {
        unsafe {
            let mut _v: HANDLE = Default::default();
            let _r = self.0.GetMemoryWin32HandleKHR(
                device.abi(), 
                get_win32handle_info.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
    /// ```c
    /// VkResult vkGetMemoryWin32HandlePropertiesKHR(VkDevice device, VkExternalMemoryHandleTypeFlagBits handleType, HANDLE handle, VkMemoryWin32HandlePropertiesKHR* pMemoryWin32HandleProperties)
    /// ```
    pub unsafe fn get_memory_win32handle_properties(
        &self,
        device: vk::Device,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: HANDLE,
        memory_win32handle_properties: &mut MemoryWin32HandlePropertiesKHR,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetMemoryWin32HandlePropertiesKHR(
                device.abi(), 
                handle_type.abi(), 
                handle.abi(), 
                memory_win32handle_properties.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::external_memory_win32 {
    type Commands = Device;
}

/// Device object
pub trait KhrExternalMemoryWin32Device {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetMemoryWin32HandleKHR(VkDevice device, VkMemoryGetWin32HandleInfoKHR const* pGetWin32HandleInfo, HANDLE* pHandle)
    /// ```
    unsafe fn get_memory_win32handle(
        &self,
        get_win32handle_info: &MemoryGetWin32HandleInfoKHR,
    ) -> crate::Result<HANDLE> {
        unsafe {
            self.commands().get_memory_win32handle(
                self.raw(),
                get_win32handle_info,
            )
        }
    }
    /// ```c
    /// VkResult vkGetMemoryWin32HandlePropertiesKHR(VkDevice device, VkExternalMemoryHandleTypeFlagBits handleType, HANDLE handle, VkMemoryWin32HandlePropertiesKHR* pMemoryWin32HandleProperties)
    /// ```
    unsafe fn get_memory_win32handle_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: HANDLE,
        memory_win32handle_properties: &mut MemoryWin32HandlePropertiesKHR,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_memory_win32handle_properties(
                self.raw(),
                handle_type,
                handle,
                memory_win32handle_properties,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::external_memory_win32 {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::external_memory_win32> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::external_memory_win32 {
        type Output = crate::hnd::Device<vk::extensions::khr::external_memory_win32>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::external_memory_win32>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::external_memory_win32> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::external_memory_win32> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::external_memory_win32> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::external_memory_win32> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrExternalMemoryWin32Device for crate::hnd::Device<vk::extensions::khr::external_memory_win32> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::external_memory_win32, vk::Device> for crate::hnd::Device<vk::extensions::khr::external_memory_win32> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
