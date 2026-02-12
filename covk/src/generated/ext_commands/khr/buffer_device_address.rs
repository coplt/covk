// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_buffer_device_address` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::buffer_device_address::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::buffer_device_address::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkDeviceAddress vkGetBufferDeviceAddress(VkDevice device, VkBufferDeviceAddressInfo const* pInfo)
    /// ```
    pub unsafe fn get_buffer_device_address(
        &self,
        device: vk::Device,
        info: &BufferDeviceAddressInfo,
    ) -> DeviceAddress {
        unsafe {
            let _r = self.0.GetBufferDeviceAddressKHR(
                device.abi(), 
                info.abi(), 
            );
            _r
        }
    }
    /// ```c
    /// uint64_t vkGetBufferOpaqueCaptureAddress(VkDevice device, VkBufferDeviceAddressInfo const* pInfo)
    /// ```
    pub unsafe fn get_buffer_opaque_capture_address(
        &self,
        device: vk::Device,
        info: &BufferDeviceAddressInfo,
    ) -> uint64_t {
        unsafe {
            let _r = self.0.GetBufferOpaqueCaptureAddressKHR(
                device.abi(), 
                info.abi(), 
            );
            _r
        }
    }
    /// ```c
    /// uint64_t vkGetDeviceMemoryOpaqueCaptureAddress(VkDevice device, VkDeviceMemoryOpaqueCaptureAddressInfo const* pInfo)
    /// ```
    pub unsafe fn get_device_memory_opaque_capture_address(
        &self,
        device: vk::Device,
        info: &DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> uint64_t {
        unsafe {
            let _r = self.0.GetDeviceMemoryOpaqueCaptureAddressKHR(
                device.abi(), 
                info.abi(), 
            );
            _r
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::buffer_device_address {
    type Commands = Device;
}

/// Device object
pub trait KhrBufferDeviceAddressDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkDeviceAddress vkGetBufferDeviceAddress(VkDevice device, VkBufferDeviceAddressInfo const* pInfo)
    /// ```
    unsafe fn get_buffer_device_address(
        &self,
        info: &BufferDeviceAddressInfo,
    ) -> DeviceAddress {
        unsafe {
            self.commands().get_buffer_device_address(
                self.raw(),
                info,
            )
        }
    }
    /// ```c
    /// uint64_t vkGetBufferOpaqueCaptureAddress(VkDevice device, VkBufferDeviceAddressInfo const* pInfo)
    /// ```
    unsafe fn get_buffer_opaque_capture_address(
        &self,
        info: &BufferDeviceAddressInfo,
    ) -> uint64_t {
        unsafe {
            self.commands().get_buffer_opaque_capture_address(
                self.raw(),
                info,
            )
        }
    }
    /// ```c
    /// uint64_t vkGetDeviceMemoryOpaqueCaptureAddress(VkDevice device, VkDeviceMemoryOpaqueCaptureAddressInfo const* pInfo)
    /// ```
    unsafe fn get_device_memory_opaque_capture_address(
        &self,
        info: &DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> uint64_t {
        unsafe {
            self.commands().get_device_memory_opaque_capture_address(
                self.raw(),
                info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::buffer_device_address {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::buffer_device_address> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::buffer_device_address {
        type Output = crate::hnd::Device<vk::extensions::khr::buffer_device_address>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::buffer_device_address>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::buffer_device_address> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::buffer_device_address> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::buffer_device_address> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::buffer_device_address> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrBufferDeviceAddressDevice for crate::hnd::Device<vk::extensions::khr::buffer_device_address> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::buffer_device_address, vk::Device> for crate::hnd::Device<vk::extensions::khr::buffer_device_address> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
