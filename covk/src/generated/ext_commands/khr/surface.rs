// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_surface` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::khr::surface::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::surface::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// void vkDestroySurfaceKHR(VkInstance instance, VkSurfaceKHR surface, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_surface(
        &self,
        instance: vk::Instance,
        surface: Option<vk::SurfaceKHR>,
    ) -> () {
        unsafe {
            self.0.DestroySurfaceKHR(
                instance.abi(), 
                surface.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// VkResult vkGetPhysicalDeviceSurfaceCapabilitiesKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, VkSurfaceCapabilitiesKHR* pSurfaceCapabilities)
    /// ```
    pub unsafe fn get_physical_device_surface_capabilities(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> crate::Result<SurfaceCapabilitiesKHR> {
        unsafe {
            let mut _v: SurfaceCapabilitiesKHR = Default::default();
            let _r = self.0.GetPhysicalDeviceSurfaceCapabilitiesKHR(
                physical_device.abi(), 
                surface.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
    /// ```c
    /// VkResult vkGetPhysicalDeviceSurfaceFormatsKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pSurfaceFormatCount, VkSurfaceFormatKHR* pSurfaceFormats)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_physical_device_surface_formats(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: Option<vk::SurfaceKHR>,
        surface_formats: Option<&mut ::alloc::vec::Vec<SurfaceFormatKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetPhysicalDeviceSurfaceFormatsKHR(
                physical_device.abi(), 
                surface.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = surface_formats {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetPhysicalDeviceSurfaceFormatsKHR(
                    physical_device.abi(), 
                    surface.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                ).vk();
                _r.result(|| Some(()))?;
                _b.set_len(_b.len() + _c as usize);
            }
            _r.result_multi(|| Some(_c))
        }
    }
    /// ```c
    /// VkResult vkGetPhysicalDeviceSurfacePresentModesKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pPresentModeCount, VkPresentModeKHR* pPresentModes)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_physical_device_surface_present_modes(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: Option<vk::SurfaceKHR>,
        present_modes: Option<&mut ::alloc::vec::Vec<PresentModeKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetPhysicalDeviceSurfacePresentModesKHR(
                physical_device.abi(), 
                surface.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = present_modes {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetPhysicalDeviceSurfacePresentModesKHR(
                    physical_device.abi(), 
                    surface.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                ).vk();
                _r.result(|| Some(()))?;
                _b.set_len(_b.len() + _c as usize);
            }
            _r.result_multi(|| Some(_c))
        }
    }
    /// ```c
    /// VkResult vkGetPhysicalDeviceSurfaceSupportKHR(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, VkSurfaceKHR surface, VkBool32* pSupported)
    /// ```
    pub unsafe fn get_physical_device_surface_support(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: uint32_t,
        surface: vk::SurfaceKHR,
    ) -> crate::Result<Bool> {
        unsafe {
            let mut _v: Bool = Default::default();
            let _r = self.0.GetPhysicalDeviceSurfaceSupportKHR(
                physical_device.abi(), 
                queue_family_index.abi(), 
                surface.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
}

impl crate::CommandScope<vk::Instance> for vk::extensions::khr::surface {
    type Commands = Instance;
}

/// Instance object
pub trait KhrSurfaceInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

    /// ```c
    /// void vkDestroySurfaceKHR(VkInstance instance, VkSurfaceKHR surface, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_surface(
        &self,
        surface: Option<vk::SurfaceKHR>,
    ) -> () {
        unsafe {
            self.commands().destroy_surface(
                self.raw(),
                surface,
            )
        }
    }
}

impl crate::HndScope<vk::Instance> for vk::extensions::khr::surface {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::khr::surface> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::khr::surface {
        type Output = crate::hnd::Instance<vk::extensions::khr::surface>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::khr::surface>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::khr::surface> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::khr::surface> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::khr::surface> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::khr::surface> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrSurfaceInstance for crate::hnd::Instance<vk::extensions::khr::surface> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::surface, vk::Instance> for crate::hnd::Instance<vk::extensions::khr::surface> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait KhrSurfacePhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkGetPhysicalDeviceSurfaceCapabilitiesKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, VkSurfaceCapabilitiesKHR* pSurfaceCapabilities)
    /// ```
    unsafe fn get_surface_capabilities(
        &self,
        surface: vk::SurfaceKHR,
    ) -> crate::Result<SurfaceCapabilitiesKHR> {
        unsafe {
            self.commands().get_physical_device_surface_capabilities(
                self.raw(),
                surface,
            )
        }
    }
    /// ```c
    /// VkResult vkGetPhysicalDeviceSurfaceFormatsKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pSurfaceFormatCount, VkSurfaceFormatKHR* pSurfaceFormats)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_surface_formats(
        &self,
        surface: Option<vk::SurfaceKHR>,
        surface_formats: Option<&mut ::alloc::vec::Vec<SurfaceFormatKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_physical_device_surface_formats(
                self.raw(),
                surface,
                surface_formats,
            )
        }
    }
    /// ```c
    /// VkResult vkGetPhysicalDeviceSurfacePresentModesKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pPresentModeCount, VkPresentModeKHR* pPresentModes)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_surface_present_modes(
        &self,
        surface: Option<vk::SurfaceKHR>,
        present_modes: Option<&mut ::alloc::vec::Vec<PresentModeKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_physical_device_surface_present_modes(
                self.raw(),
                surface,
                present_modes,
            )
        }
    }
    /// ```c
    /// VkResult vkGetPhysicalDeviceSurfaceSupportKHR(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, VkSurfaceKHR surface, VkBool32* pSupported)
    /// ```
    unsafe fn get_surface_support(
        &self,
        queue_family_index: uint32_t,
        surface: vk::SurfaceKHR,
    ) -> crate::Result<Bool> {
        unsafe {
            self.commands().get_physical_device_surface_support(
                self.raw(),
                queue_family_index,
                surface,
            )
        }
    }
}

impl<O: crate::HndCtx<vk::extensions::khr::surface, vk::Instance>> crate::Owner<vk::SurfaceKHR, vk::extensions::khr::surface> for O {
    fn drop(&mut self, raw: vk::SurfaceKHR) {
        unsafe {
            self.commands().destroy_surface(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::khr::surface, vk::Instance> for ::alloc::sync::Arc<crate::Unique<vk::SurfaceKHR, O, vk::extensions::khr::surface>>
    where O: crate::HndCtx<vk::extensions::khr::surface, vk::Instance> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Instance { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Instance> { self.owner.commands() }
}

impl crate::HndScope<vk::SurfaceKHR> for vk::extensions::khr::surface {
    type Impl = _hs_SurfaceKHR::SurfaceKHR;
}


mod _hs_SurfaceKHR {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct SurfaceKHR(pub(crate) crate::handle::Hnd<vk::SurfaceKHR, ::alloc::sync::Arc<super::Instance>>);

    impl Clone for SurfaceKHR {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::SurfaceKHR, ::alloc::sync::Arc<super::Instance>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::extensions::khr::surface, vk::Instance>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::SurfaceKHR, ::alloc::sync::Arc<super::Instance>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_surface(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::SurfaceKHR<vk::extensions::khr::surface>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::khr::surface, vk::Instance>, raw: vk::SurfaceKHR) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::khr::surface, vk::Instance>, raw: vk::SurfaceKHR, dep: impl FnOnce() -> Dep) -> Self {
            Self(SurfaceKHR(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::SurfaceKHR<vk::extensions::khr::surface> {
        pub fn raw(&self) -> vk::SurfaceKHR { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::SurfaceKHR<vk::extensions::khr::surface> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("SurfaceKHR({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::SurfaceKHR<vk::extensions::khr::surface> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::khr::surface> for vk::SurfaceKHR
        where Ctx: crate::HndCtx<vk::extensions::khr::surface, vk::Instance>,
    {
        type Output = crate::hnd::SurfaceKHR<vk::extensions::khr::surface>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::SurfaceKHR::<vk::extensions::khr::surface>::new_with(ctx, self, dep) }
        }
    }
}
