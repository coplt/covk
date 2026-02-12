// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_FUCHSIA_external_semaphore` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::fuchsia::external_semaphore::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::fuchsia::external_semaphore::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkGetSemaphoreZirconHandleFUCHSIA(VkDevice device, VkSemaphoreGetZirconHandleInfoFUCHSIA const* pGetZirconHandleInfo, zx_handle_t* pZirconHandle)
    /// ```
    pub unsafe fn get_semaphore_zircon_handle(
        &self,
        device: vk::Device,
        get_zircon_handle_info: &SemaphoreGetZirconHandleInfoFUCHSIA,
    ) -> crate::Result<zx_handle_t> {
        unsafe {
            let mut _v: zx_handle_t = Default::default();
            let _r = self.0.GetSemaphoreZirconHandleFUCHSIA(
                device.abi(), 
                get_zircon_handle_info.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
    /// ```c
    /// VkResult vkImportSemaphoreZirconHandleFUCHSIA(VkDevice device, VkImportSemaphoreZirconHandleInfoFUCHSIA const* pImportSemaphoreZirconHandleInfo)
    /// ```
    pub unsafe fn import_semaphore_zircon_handle(
        &self,
        device: vk::Device,
        import_semaphore_zircon_handle_info: &ImportSemaphoreZirconHandleInfoFUCHSIA,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.ImportSemaphoreZirconHandleFUCHSIA(
                device.abi(), 
                import_semaphore_zircon_handle_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::fuchsia::external_semaphore {
    type Commands = Device;
}

/// Device object
pub trait FuchsiaExternalSemaphoreDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetSemaphoreZirconHandleFUCHSIA(VkDevice device, VkSemaphoreGetZirconHandleInfoFUCHSIA const* pGetZirconHandleInfo, zx_handle_t* pZirconHandle)
    /// ```
    unsafe fn get_semaphore_zircon_handle(
        &self,
        get_zircon_handle_info: &SemaphoreGetZirconHandleInfoFUCHSIA,
    ) -> crate::Result<zx_handle_t> {
        unsafe {
            self.commands().get_semaphore_zircon_handle(
                self.raw(),
                get_zircon_handle_info,
            )
        }
    }
    /// ```c
    /// VkResult vkImportSemaphoreZirconHandleFUCHSIA(VkDevice device, VkImportSemaphoreZirconHandleInfoFUCHSIA const* pImportSemaphoreZirconHandleInfo)
    /// ```
    unsafe fn import_semaphore_zircon_handle(
        &self,
        import_semaphore_zircon_handle_info: &ImportSemaphoreZirconHandleInfoFUCHSIA,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().import_semaphore_zircon_handle(
                self.raw(),
                import_semaphore_zircon_handle_info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::fuchsia::external_semaphore {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::fuchsia::external_semaphore> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::fuchsia::external_semaphore {
        type Output = crate::hnd::Device<vk::extensions::fuchsia::external_semaphore>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::fuchsia::external_semaphore>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::fuchsia::external_semaphore> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::fuchsia::external_semaphore> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::fuchsia::external_semaphore> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::fuchsia::external_semaphore> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::FuchsiaExternalSemaphoreDevice for crate::hnd::Device<vk::extensions::fuchsia::external_semaphore> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::fuchsia::external_semaphore, vk::Device> for crate::hnd::Device<vk::extensions::fuchsia::external_semaphore> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
