// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_external_memory_metal` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::external_memory_metal::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::external_memory_metal::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkGetMemoryMetalHandleEXT(VkDevice device, VkMemoryGetMetalHandleInfoEXT const* pGetMetalHandleInfo, void** pHandle)
    /// ```
    pub unsafe fn get_memory_metal_handle(
        &self,
        device: vk::Device,
        get_metal_handle_info: &MemoryGetMetalHandleInfoEXT,
    ) -> crate::Result<*mut void> {
        unsafe {
            let mut _v: *mut void = Default::default();
            let _r = self.0.GetMemoryMetalHandleEXT(
                device.abi(), 
                get_metal_handle_info.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
    /// ```c
    /// VkResult vkGetMemoryMetalHandlePropertiesEXT(VkDevice device, VkExternalMemoryHandleTypeFlagBits handleType, void const* pHandle, VkMemoryMetalHandlePropertiesEXT* pMemoryMetalHandleProperties)
    /// ```
    pub unsafe fn get_memory_metal_handle_properties(
        &self,
        device: vk::Device,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: *const void,
        memory_metal_handle_properties: &mut MemoryMetalHandlePropertiesEXT,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetMemoryMetalHandlePropertiesEXT(
                device.abi(), 
                handle_type.abi(), 
                handle.abi(), 
                memory_metal_handle_properties.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::external_memory_metal {
    type Commands = Device;
}

/// Device object
pub trait ExtExternalMemoryMetalDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetMemoryMetalHandleEXT(VkDevice device, VkMemoryGetMetalHandleInfoEXT const* pGetMetalHandleInfo, void** pHandle)
    /// ```
    unsafe fn get_memory_metal_handle(
        &self,
        get_metal_handle_info: &MemoryGetMetalHandleInfoEXT,
    ) -> crate::Result<*mut void> {
        unsafe {
            self.commands().get_memory_metal_handle(
                self.raw(),
                get_metal_handle_info,
            )
        }
    }
    /// ```c
    /// VkResult vkGetMemoryMetalHandlePropertiesEXT(VkDevice device, VkExternalMemoryHandleTypeFlagBits handleType, void const* pHandle, VkMemoryMetalHandlePropertiesEXT* pMemoryMetalHandleProperties)
    /// ```
    unsafe fn get_memory_metal_handle_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: *const void,
        memory_metal_handle_properties: &mut MemoryMetalHandlePropertiesEXT,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_memory_metal_handle_properties(
                self.raw(),
                handle_type,
                handle,
                memory_metal_handle_properties,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::ext::external_memory_metal {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::external_memory_metal> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::external_memory_metal {
        type Output = crate::hnd::Device<vk::extensions::ext::external_memory_metal>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::external_memory_metal>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::external_memory_metal> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::external_memory_metal> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::external_memory_metal> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::external_memory_metal> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtExternalMemoryMetalDevice for crate::hnd::Device<vk::extensions::ext::external_memory_metal> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::external_memory_metal, vk::Device> for crate::hnd::Device<vk::extensions::ext::external_memory_metal> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
