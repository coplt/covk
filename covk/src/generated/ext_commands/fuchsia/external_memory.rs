// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_FUCHSIA_external_memory` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::fuchsia::external_memory::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::fuchsia::external_memory::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkGetMemoryZirconHandleFUCHSIA(VkDevice device, VkMemoryGetZirconHandleInfoFUCHSIA const* pGetZirconHandleInfo, zx_handle_t* pZirconHandle)
    /// ```
    pub unsafe fn get_memory_zircon_handle(
        &self,
        device: vk::Device,
        get_zircon_handle_info: &MemoryGetZirconHandleInfoFUCHSIA,
    ) -> crate::Result<zx_handle_t> {
        unsafe {
            let mut _v: zx_handle_t = Default::default();
            let _r = self.0.GetMemoryZirconHandleFUCHSIA(
                device.abi(), 
                get_zircon_handle_info.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
    /// ```c
    /// VkResult vkGetMemoryZirconHandlePropertiesFUCHSIA(VkDevice device, VkExternalMemoryHandleTypeFlagBits handleType, zx_handle_t zirconHandle, VkMemoryZirconHandlePropertiesFUCHSIA* pMemoryZirconHandleProperties)
    /// ```
    pub unsafe fn get_memory_zircon_handle_properties(
        &self,
        device: vk::Device,
        handle_type: ExternalMemoryHandleTypeFlags,
        zircon_handle: zx_handle_t,
        memory_zircon_handle_properties: &mut MemoryZirconHandlePropertiesFUCHSIA,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetMemoryZirconHandlePropertiesFUCHSIA(
                device.abi(), 
                handle_type.abi(), 
                zircon_handle.abi(), 
                memory_zircon_handle_properties.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::fuchsia::external_memory {
    type Commands = Device;
}

/// Device object
pub trait FuchsiaExternalMemoryDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetMemoryZirconHandleFUCHSIA(VkDevice device, VkMemoryGetZirconHandleInfoFUCHSIA const* pGetZirconHandleInfo, zx_handle_t* pZirconHandle)
    /// ```
    unsafe fn get_memory_zircon_handle(
        &self,
        get_zircon_handle_info: &MemoryGetZirconHandleInfoFUCHSIA,
    ) -> crate::Result<zx_handle_t> {
        unsafe {
            self.commands().get_memory_zircon_handle(
                self.raw(),
                get_zircon_handle_info,
            )
        }
    }
    /// ```c
    /// VkResult vkGetMemoryZirconHandlePropertiesFUCHSIA(VkDevice device, VkExternalMemoryHandleTypeFlagBits handleType, zx_handle_t zirconHandle, VkMemoryZirconHandlePropertiesFUCHSIA* pMemoryZirconHandleProperties)
    /// ```
    unsafe fn get_memory_zircon_handle_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        zircon_handle: zx_handle_t,
        memory_zircon_handle_properties: &mut MemoryZirconHandlePropertiesFUCHSIA,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_memory_zircon_handle_properties(
                self.raw(),
                handle_type,
                zircon_handle,
                memory_zircon_handle_properties,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::fuchsia::external_memory {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::fuchsia::external_memory> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::fuchsia::external_memory {
        type Output = crate::hnd::Device<vk::extensions::fuchsia::external_memory>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::fuchsia::external_memory>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::fuchsia::external_memory> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::fuchsia::external_memory> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::fuchsia::external_memory> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::fuchsia::external_memory> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::FuchsiaExternalMemoryDevice for crate::hnd::Device<vk::extensions::fuchsia::external_memory> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::fuchsia::external_memory, vk::Device> for crate::hnd::Device<vk::extensions::fuchsia::external_memory> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
