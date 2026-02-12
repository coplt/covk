// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_NV_acquire_winrt_display` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::nv::acquire_winrt_display::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::acquire_winrt_display::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkAcquireWinrtDisplayNV(VkPhysicalDevice physicalDevice, VkDisplayKHR display)
    /// ```
    pub unsafe fn acquire_winrt_display(
        &self,
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.AcquireWinrtDisplayNV(
                physical_device.abi(), 
                display.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkGetWinrtDisplayNV(VkPhysicalDevice physicalDevice, uint32_t deviceRelativeId, VkDisplayKHR* pDisplay)
    /// ```
    pub unsafe fn get_winrt_display(
        &self,
        physical_device: vk::PhysicalDevice,
        device_relative_id: uint32_t,
    ) -> crate::Result<vk::DisplayKHR> {
        unsafe {
            let mut _v: Option<vk::DisplayKHR> = Default::default();
            let _r = self.0.GetWinrtDisplayNV(
                physical_device.abi(), 
                device_relative_id.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
}

impl crate::CommandScope<vk::Instance> for vk::extensions::nv::acquire_winrt_display {
    type Commands = Instance;
}

/// Instance object
pub trait NvAcquireWinrtDisplayInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::nv::acquire_winrt_display {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::nv::acquire_winrt_display> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::nv::acquire_winrt_display {
        type Output = crate::hnd::Instance<vk::extensions::nv::acquire_winrt_display>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::nv::acquire_winrt_display>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::nv::acquire_winrt_display> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::nv::acquire_winrt_display> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::nv::acquire_winrt_display> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::nv::acquire_winrt_display> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvAcquireWinrtDisplayInstance for crate::hnd::Instance<vk::extensions::nv::acquire_winrt_display> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::acquire_winrt_display, vk::Instance> for crate::hnd::Instance<vk::extensions::nv::acquire_winrt_display> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait NvAcquireWinrtDisplayPhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkAcquireWinrtDisplayNV(VkPhysicalDevice physicalDevice, VkDisplayKHR display)
    /// ```
    unsafe fn acquire_winrt_display(
        &self,
        display: vk::DisplayKHR,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().acquire_winrt_display(
                self.raw(),
                display,
            )
        }
    }
    /// ```c
    /// VkResult vkGetWinrtDisplayNV(VkPhysicalDevice physicalDevice, uint32_t deviceRelativeId, VkDisplayKHR* pDisplay)
    /// ```
    unsafe fn get_winrt_display(
        &self,
        device_relative_id: uint32_t,
    ) -> crate::Result<vk::DisplayKHR> {
        unsafe {
            self.commands().get_winrt_display(
                self.raw(),
                device_relative_id,
            )
        }
    }
}
