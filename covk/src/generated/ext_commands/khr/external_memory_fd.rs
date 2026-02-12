// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_external_memory_fd` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::external_memory_fd::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::external_memory_fd::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkGetMemoryFdKHR(VkDevice device, VkMemoryGetFdInfoKHR const* pGetFdInfo, int* pFd)
    /// ```
    pub unsafe fn get_memory_fd(
        &self,
        device: vk::Device,
        get_fd_info: &MemoryGetFdInfoKHR,
    ) -> crate::Result<int> {
        unsafe {
            let mut _v: int = Default::default();
            let _r = self.0.GetMemoryFdKHR(
                device.abi(), 
                get_fd_info.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
    /// ```c
    /// VkResult vkGetMemoryFdPropertiesKHR(VkDevice device, VkExternalMemoryHandleTypeFlagBits handleType, int fd, VkMemoryFdPropertiesKHR* pMemoryFdProperties)
    /// ```
    pub unsafe fn get_memory_fd_properties(
        &self,
        device: vk::Device,
        handle_type: ExternalMemoryHandleTypeFlags,
        fd: int,
        memory_fd_properties: &mut MemoryFdPropertiesKHR,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetMemoryFdPropertiesKHR(
                device.abi(), 
                handle_type.abi(), 
                fd.abi(), 
                memory_fd_properties.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::external_memory_fd {
    type Commands = Device;
}

/// Device object
pub trait KhrExternalMemoryFdDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetMemoryFdKHR(VkDevice device, VkMemoryGetFdInfoKHR const* pGetFdInfo, int* pFd)
    /// ```
    unsafe fn get_memory_fd(
        &self,
        get_fd_info: &MemoryGetFdInfoKHR,
    ) -> crate::Result<int> {
        unsafe {
            self.commands().get_memory_fd(
                self.raw(),
                get_fd_info,
            )
        }
    }
    /// ```c
    /// VkResult vkGetMemoryFdPropertiesKHR(VkDevice device, VkExternalMemoryHandleTypeFlagBits handleType, int fd, VkMemoryFdPropertiesKHR* pMemoryFdProperties)
    /// ```
    unsafe fn get_memory_fd_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        fd: int,
        memory_fd_properties: &mut MemoryFdPropertiesKHR,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_memory_fd_properties(
                self.raw(),
                handle_type,
                fd,
                memory_fd_properties,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::external_memory_fd {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::external_memory_fd> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::external_memory_fd {
        type Output = crate::hnd::Device<vk::extensions::khr::external_memory_fd>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::external_memory_fd>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::external_memory_fd> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::external_memory_fd> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::external_memory_fd> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::external_memory_fd> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrExternalMemoryFdDevice for crate::hnd::Device<vk::extensions::khr::external_memory_fd> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::external_memory_fd, vk::Device> for crate::hnd::Device<vk::extensions::khr::external_memory_fd> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
