// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_acquire_xlib_display` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::ext::acquire_xlib_display::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::acquire_xlib_display::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkAcquireXlibDisplayEXT(VkPhysicalDevice physicalDevice, Display* dpy, VkDisplayKHR display)
    /// ```
    pub unsafe fn acquire_xlib_display(
        &self,
        physical_device: vk::PhysicalDevice,
        dpy: *mut Display,
        display: vk::DisplayKHR,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.AcquireXlibDisplayEXT(
                physical_device.abi(), 
                dpy.abi(), 
                display.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkGetRandROutputDisplayEXT(VkPhysicalDevice physicalDevice, Display* dpy, RROutput rrOutput, VkDisplayKHR* pDisplay)
    /// ```
    pub unsafe fn get_rand_r_output_display(
        &self,
        physical_device: vk::PhysicalDevice,
        dpy: *mut Display,
        rr_output: RROutput,
    ) -> crate::Result<vk::DisplayKHR> {
        unsafe {
            let mut _v: Option<vk::DisplayKHR> = Default::default();
            let _r = self.0.GetRandROutputDisplayEXT(
                physical_device.abi(), 
                dpy.abi(), 
                rr_output.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
}

impl crate::CommandScope<vk::Instance> for vk::extensions::ext::acquire_xlib_display {
    type Commands = Instance;
}

/// Instance object
pub trait ExtAcquireXlibDisplayInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::ext::acquire_xlib_display {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::ext::acquire_xlib_display> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::ext::acquire_xlib_display {
        type Output = crate::hnd::Instance<vk::extensions::ext::acquire_xlib_display>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::ext::acquire_xlib_display>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::ext::acquire_xlib_display> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::ext::acquire_xlib_display> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::ext::acquire_xlib_display> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::ext::acquire_xlib_display> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtAcquireXlibDisplayInstance for crate::hnd::Instance<vk::extensions::ext::acquire_xlib_display> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::acquire_xlib_display, vk::Instance> for crate::hnd::Instance<vk::extensions::ext::acquire_xlib_display> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait ExtAcquireXlibDisplayPhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkAcquireXlibDisplayEXT(VkPhysicalDevice physicalDevice, Display* dpy, VkDisplayKHR display)
    /// ```
    unsafe fn acquire_xlib_display(
        &self,
        dpy: *mut Display,
        display: vk::DisplayKHR,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().acquire_xlib_display(
                self.raw(),
                dpy,
                display,
            )
        }
    }
    /// ```c
    /// VkResult vkGetRandROutputDisplayEXT(VkPhysicalDevice physicalDevice, Display* dpy, RROutput rrOutput, VkDisplayKHR* pDisplay)
    /// ```
    unsafe fn get_rand_r_output_display(
        &self,
        dpy: *mut Display,
        rr_output: RROutput,
    ) -> crate::Result<vk::DisplayKHR> {
        unsafe {
            self.commands().get_rand_r_output_display(
                self.raw(),
                dpy,
                rr_output,
            )
        }
    }
}
