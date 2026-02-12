// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_android_surface` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::khr::android_surface::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::android_surface::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkCreateAndroidSurfaceKHR(VkInstance instance, VkAndroidSurfaceCreateInfoKHR const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkSurfaceKHR* pSurface)
    /// ```
    pub unsafe fn create_android_surface(
        &self,
        instance: vk::Instance,
        create_info: &AndroidSurfaceCreateInfoKHR,
    ) -> crate::Result<vk::SurfaceKHR> {
        unsafe {
            let mut _v: Option<vk::SurfaceKHR> = Default::default();
            let _r = self.0.CreateAndroidSurfaceKHR(
                instance.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
}

impl crate::CommandScope<vk::Instance> for vk::extensions::khr::android_surface {
    type Commands = Instance;
}

/// Instance object
pub trait KhrAndroidSurfaceInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkCreateAndroidSurfaceKHR(VkInstance instance, VkAndroidSurfaceCreateInfoKHR const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkSurfaceKHR* pSurface)
    /// ```
    unsafe fn create_android_surface(
        &self,
        create_info: &AndroidSurfaceCreateInfoKHR,
    ) -> crate::Result<vk::SurfaceKHR> {
        unsafe {
            self.commands().create_android_surface(
                self.raw(),
                create_info,
            )
        }
    }
}

impl crate::HndScope<vk::Instance> for vk::extensions::khr::android_surface {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::khr::android_surface> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::khr::android_surface {
        type Output = crate::hnd::Instance<vk::extensions::khr::android_surface>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::khr::android_surface>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::khr::android_surface> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::khr::android_surface> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::khr::android_surface> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::khr::android_surface> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrAndroidSurfaceInstance for crate::hnd::Instance<vk::extensions::khr::android_surface> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::android_surface, vk::Instance> for crate::hnd::Instance<vk::extensions::khr::android_surface> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}
