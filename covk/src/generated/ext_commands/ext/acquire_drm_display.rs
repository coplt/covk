// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_acquire_drm_display` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::ext::acquire_drm_display::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::acquire_drm_display::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkAcquireDrmDisplayEXT(VkPhysicalDevice physicalDevice, int32_t drmFd, VkDisplayKHR display)
    /// ```
    pub unsafe fn acquire_drm_display(
        &self,
        physical_device: vk::PhysicalDevice,
        drm_fd: int32_t,
        display: vk::DisplayKHR,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.AcquireDrmDisplayEXT(
                physical_device.abi(), 
                drm_fd.abi(), 
                display.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkGetDrmDisplayEXT(VkPhysicalDevice physicalDevice, int32_t drmFd, uint32_t connectorId, VkDisplayKHR* display)
    /// ```
    pub unsafe fn get_drm_display(
        &self,
        physical_device: vk::PhysicalDevice,
        drm_fd: int32_t,
        connector_id: uint32_t,
    ) -> crate::Result<vk::DisplayKHR> {
        unsafe {
            let mut _v: Option<vk::DisplayKHR> = Default::default();
            let _r = self.0.GetDrmDisplayEXT(
                physical_device.abi(), 
                drm_fd.abi(), 
                connector_id.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
}

impl crate::CommandScope<vk::Instance> for vk::extensions::ext::acquire_drm_display {
    type Commands = Instance;
}

/// Instance object
pub trait ExtAcquireDrmDisplayInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::ext::acquire_drm_display {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::ext::acquire_drm_display> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::ext::acquire_drm_display {
        type Output = crate::hnd::Instance<vk::extensions::ext::acquire_drm_display>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::ext::acquire_drm_display>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::ext::acquire_drm_display> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::ext::acquire_drm_display> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::ext::acquire_drm_display> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::ext::acquire_drm_display> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtAcquireDrmDisplayInstance for crate::hnd::Instance<vk::extensions::ext::acquire_drm_display> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::acquire_drm_display, vk::Instance> for crate::hnd::Instance<vk::extensions::ext::acquire_drm_display> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait ExtAcquireDrmDisplayPhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkAcquireDrmDisplayEXT(VkPhysicalDevice physicalDevice, int32_t drmFd, VkDisplayKHR display)
    /// ```
    unsafe fn acquire_drm_display(
        &self,
        drm_fd: int32_t,
        display: vk::DisplayKHR,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().acquire_drm_display(
                self.raw(),
                drm_fd,
                display,
            )
        }
    }
    /// ```c
    /// VkResult vkGetDrmDisplayEXT(VkPhysicalDevice physicalDevice, int32_t drmFd, uint32_t connectorId, VkDisplayKHR* display)
    /// ```
    unsafe fn get_drm_display(
        &self,
        drm_fd: int32_t,
        connector_id: uint32_t,
    ) -> crate::Result<vk::DisplayKHR> {
        unsafe {
            self.commands().get_drm_display(
                self.raw(),
                drm_fd,
                connector_id,
            )
        }
    }
}
