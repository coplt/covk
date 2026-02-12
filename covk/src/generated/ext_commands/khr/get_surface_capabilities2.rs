// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_get_surface_capabilities2` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::khr::get_surface_capabilities2::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::get_surface_capabilities2::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkGetPhysicalDeviceSurfaceCapabilities2KHR(VkPhysicalDevice physicalDevice, VkPhysicalDeviceSurfaceInfo2KHR const* pSurfaceInfo, VkSurfaceCapabilities2KHR* pSurfaceCapabilities)
    /// ```
    pub unsafe fn get_physical_device_surface_capabilities2(
        &self,
        physical_device: vk::PhysicalDevice,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
        surface_capabilities: &mut SurfaceCapabilities2KHR,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetPhysicalDeviceSurfaceCapabilities2KHR(
                physical_device.abi(), 
                surface_info.abi(), 
                surface_capabilities.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkGetPhysicalDeviceSurfaceFormats2KHR(VkPhysicalDevice physicalDevice, VkPhysicalDeviceSurfaceInfo2KHR const* pSurfaceInfo, uint32_t* pSurfaceFormatCount, VkSurfaceFormat2KHR* pSurfaceFormats)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_physical_device_surface_formats2(
        &self,
        physical_device: vk::PhysicalDevice,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
        surface_formats: Option<&mut ::alloc::vec::Vec<SurfaceFormat2KHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetPhysicalDeviceSurfaceFormats2KHR(
                physical_device.abi(), 
                surface_info.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = surface_formats {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetPhysicalDeviceSurfaceFormats2KHR(
                    physical_device.abi(), 
                    surface_info.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                ).vk();
                _r.result(|| Some(()))?;
                _b.set_len(_b.len() + _c as usize);
            }
            _r.result_multi(|| Some(_c))
        }
    }
}

impl crate::CommandScope<vk::Instance> for vk::extensions::khr::get_surface_capabilities2 {
    type Commands = Instance;
}

/// Instance object
pub trait KhrGetSurfaceCapabilities2Instance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::khr::get_surface_capabilities2 {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::khr::get_surface_capabilities2> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::khr::get_surface_capabilities2 {
        type Output = crate::hnd::Instance<vk::extensions::khr::get_surface_capabilities2>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::khr::get_surface_capabilities2>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::khr::get_surface_capabilities2> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::khr::get_surface_capabilities2> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::khr::get_surface_capabilities2> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::khr::get_surface_capabilities2> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrGetSurfaceCapabilities2Instance for crate::hnd::Instance<vk::extensions::khr::get_surface_capabilities2> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::get_surface_capabilities2, vk::Instance> for crate::hnd::Instance<vk::extensions::khr::get_surface_capabilities2> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait KhrGetSurfaceCapabilities2PhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkGetPhysicalDeviceSurfaceCapabilities2KHR(VkPhysicalDevice physicalDevice, VkPhysicalDeviceSurfaceInfo2KHR const* pSurfaceInfo, VkSurfaceCapabilities2KHR* pSurfaceCapabilities)
    /// ```
    unsafe fn get_surface_capabilities2(
        &self,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
        surface_capabilities: &mut SurfaceCapabilities2KHR,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_physical_device_surface_capabilities2(
                self.raw(),
                surface_info,
                surface_capabilities,
            )
        }
    }
    /// ```c
    /// VkResult vkGetPhysicalDeviceSurfaceFormats2KHR(VkPhysicalDevice physicalDevice, VkPhysicalDeviceSurfaceInfo2KHR const* pSurfaceInfo, uint32_t* pSurfaceFormatCount, VkSurfaceFormat2KHR* pSurfaceFormats)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_surface_formats2(
        &self,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
        surface_formats: Option<&mut ::alloc::vec::Vec<SurfaceFormat2KHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_physical_device_surface_formats2(
                self.raw(),
                surface_info,
                surface_formats,
            )
        }
    }
}
