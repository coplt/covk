// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_xlib_surface` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::khr::xlib_surface::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::xlib_surface::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkCreateXlibSurfaceKHR(VkInstance instance, VkXlibSurfaceCreateInfoKHR const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkSurfaceKHR* pSurface)
    /// ```
    pub unsafe fn create_xlib_surface(
        &self,
        instance: vk::Instance,
        create_info: &XlibSurfaceCreateInfoKHR,
    ) -> crate::Result<vk::SurfaceKHR> {
        unsafe {
            let mut _v: Option<vk::SurfaceKHR> = Default::default();
            let _r = self.0.CreateXlibSurfaceKHR(
                instance.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkBool32 vkGetPhysicalDeviceXlibPresentationSupportKHR(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, Display* dpy, VisualID visualID)
    /// ```
    pub unsafe fn get_physical_device_xlib_presentation_support(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: uint32_t,
        dpy: *mut Display,
        visual_id: VisualID,
    ) -> Bool {
        unsafe {
            let _r = self.0.GetPhysicalDeviceXlibPresentationSupportKHR(
                physical_device.abi(), 
                queue_family_index.abi(), 
                dpy.abi(), 
                visual_id.abi(), 
            );
            _r.vk()
        }
    }
}

impl crate::CommandScope<vk::Instance> for vk::extensions::khr::xlib_surface {
    type Commands = Instance;
}

/// Instance object
pub trait KhrXlibSurfaceInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkCreateXlibSurfaceKHR(VkInstance instance, VkXlibSurfaceCreateInfoKHR const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkSurfaceKHR* pSurface)
    /// ```
    unsafe fn create_xlib_surface(
        &self,
        create_info: &XlibSurfaceCreateInfoKHR,
    ) -> crate::Result<vk::SurfaceKHR> {
        unsafe {
            self.commands().create_xlib_surface(
                self.raw(),
                create_info,
            )
        }
    }
}

impl crate::HndScope<vk::Instance> for vk::extensions::khr::xlib_surface {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::khr::xlib_surface> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::khr::xlib_surface {
        type Output = crate::hnd::Instance<vk::extensions::khr::xlib_surface>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::khr::xlib_surface>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::khr::xlib_surface> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::khr::xlib_surface> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::khr::xlib_surface> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::khr::xlib_surface> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrXlibSurfaceInstance for crate::hnd::Instance<vk::extensions::khr::xlib_surface> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::xlib_surface, vk::Instance> for crate::hnd::Instance<vk::extensions::khr::xlib_surface> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait KhrXlibSurfacePhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkBool32 vkGetPhysicalDeviceXlibPresentationSupportKHR(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, Display* dpy, VisualID visualID)
    /// ```
    unsafe fn get_xlib_presentation_support(
        &self,
        queue_family_index: uint32_t,
        dpy: *mut Display,
        visual_id: VisualID,
    ) -> Bool {
        unsafe {
            self.commands().get_physical_device_xlib_presentation_support(
                self.raw(),
                queue_family_index,
                dpy,
                visual_id,
            )
        }
    }
}
