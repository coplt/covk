// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_display` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::khr::display::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::display::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkCreateDisplayModeKHR(VkPhysicalDevice physicalDevice, VkDisplayKHR display, VkDisplayModeCreateInfoKHR const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkDisplayModeKHR* pMode)
    /// ```
    pub unsafe fn create_display_mode(
        &self,
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
        create_info: &DisplayModeCreateInfoKHR,
    ) -> crate::Result<vk::DisplayModeKHR> {
        unsafe {
            let mut _v: Option<vk::DisplayModeKHR> = Default::default();
            let _r = self.0.CreateDisplayModeKHR(
                physical_device.abi(), 
                display.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateDisplayPlaneSurfaceKHR(VkInstance instance, VkDisplaySurfaceCreateInfoKHR const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkSurfaceKHR* pSurface)
    /// ```
    pub unsafe fn create_display_plane_surface(
        &self,
        instance: vk::Instance,
        create_info: &DisplaySurfaceCreateInfoKHR,
    ) -> crate::Result<vk::SurfaceKHR> {
        unsafe {
            let mut _v: Option<vk::SurfaceKHR> = Default::default();
            let _r = self.0.CreateDisplayPlaneSurfaceKHR(
                instance.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkGetDisplayModePropertiesKHR(VkPhysicalDevice physicalDevice, VkDisplayKHR display, uint32_t* pPropertyCount, VkDisplayModePropertiesKHR* pProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_display_mode_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
        properties: Option<&mut ::alloc::vec::Vec<DisplayModePropertiesKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetDisplayModePropertiesKHR(
                physical_device.abi(), 
                display.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = properties {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetDisplayModePropertiesKHR(
                    physical_device.abi(), 
                    display.abi(), 
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
    /// VkResult vkGetDisplayPlaneCapabilitiesKHR(VkPhysicalDevice physicalDevice, VkDisplayModeKHR mode, uint32_t planeIndex, VkDisplayPlaneCapabilitiesKHR* pCapabilities)
    /// ```
    pub unsafe fn get_display_plane_capabilities(
        &self,
        physical_device: vk::PhysicalDevice,
        mode: vk::DisplayModeKHR,
        plane_index: uint32_t,
    ) -> crate::Result<DisplayPlaneCapabilitiesKHR> {
        unsafe {
            let mut _v: DisplayPlaneCapabilitiesKHR = Default::default();
            let _r = self.0.GetDisplayPlaneCapabilitiesKHR(
                physical_device.abi(), 
                mode.abi(), 
                plane_index.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
    /// ```c
    /// VkResult vkGetDisplayPlaneSupportedDisplaysKHR(VkPhysicalDevice physicalDevice, uint32_t planeIndex, uint32_t* pDisplayCount, VkDisplayKHR* pDisplays)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_display_plane_supported_displays(
        &self,
        physical_device: vk::PhysicalDevice,
        plane_index: uint32_t,
        displays: Option<&mut ::alloc::vec::Vec<Option<vk::DisplayKHR>>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetDisplayPlaneSupportedDisplaysKHR(
                physical_device.abi(), 
                plane_index.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = displays {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetDisplayPlaneSupportedDisplaysKHR(
                    physical_device.abi(), 
                    plane_index.abi(), 
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
    /// VkResult vkGetPhysicalDeviceDisplayPlanePropertiesKHR(VkPhysicalDevice physicalDevice, uint32_t* pPropertyCount, VkDisplayPlanePropertiesKHR* pProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_physical_device_display_plane_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        properties: Option<&mut ::alloc::vec::Vec<DisplayPlanePropertiesKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetPhysicalDeviceDisplayPlanePropertiesKHR(
                physical_device.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = properties {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetPhysicalDeviceDisplayPlanePropertiesKHR(
                    physical_device.abi(), 
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
    /// VkResult vkGetPhysicalDeviceDisplayPropertiesKHR(VkPhysicalDevice physicalDevice, uint32_t* pPropertyCount, VkDisplayPropertiesKHR* pProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_physical_device_display_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        properties: Option<&mut ::alloc::vec::Vec<DisplayPropertiesKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetPhysicalDeviceDisplayPropertiesKHR(
                physical_device.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = properties {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetPhysicalDeviceDisplayPropertiesKHR(
                    physical_device.abi(), 
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

impl crate::CommandScope<vk::Instance> for vk::extensions::khr::display {
    type Commands = Instance;
}

/// Instance object
pub trait KhrDisplayInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkCreateDisplayPlaneSurfaceKHR(VkInstance instance, VkDisplaySurfaceCreateInfoKHR const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkSurfaceKHR* pSurface)
    /// ```
    unsafe fn create_display_plane_surface(
        &self,
        create_info: &DisplaySurfaceCreateInfoKHR,
    ) -> crate::Result<vk::SurfaceKHR> {
        unsafe {
            self.commands().create_display_plane_surface(
                self.raw(),
                create_info,
            )
        }
    }
}

impl crate::HndScope<vk::Instance> for vk::extensions::khr::display {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::khr::display> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::khr::display {
        type Output = crate::hnd::Instance<vk::extensions::khr::display>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::khr::display>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::khr::display> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::khr::display> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::khr::display> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::khr::display> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrDisplayInstance for crate::hnd::Instance<vk::extensions::khr::display> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::display, vk::Instance> for crate::hnd::Instance<vk::extensions::khr::display> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait KhrDisplayPhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkCreateDisplayModeKHR(VkPhysicalDevice physicalDevice, VkDisplayKHR display, VkDisplayModeCreateInfoKHR const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkDisplayModeKHR* pMode)
    /// ```
    unsafe fn create_display_mode(
        &self,
        display: vk::DisplayKHR,
        create_info: &DisplayModeCreateInfoKHR,
    ) -> crate::Result<vk::DisplayModeKHR> {
        unsafe {
            self.commands().create_display_mode(
                self.raw(),
                display,
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkGetDisplayModePropertiesKHR(VkPhysicalDevice physicalDevice, VkDisplayKHR display, uint32_t* pPropertyCount, VkDisplayModePropertiesKHR* pProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_display_mode_properties(
        &self,
        display: vk::DisplayKHR,
        properties: Option<&mut ::alloc::vec::Vec<DisplayModePropertiesKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_display_mode_properties(
                self.raw(),
                display,
                properties,
            )
        }
    }
    /// ```c
    /// VkResult vkGetDisplayPlaneCapabilitiesKHR(VkPhysicalDevice physicalDevice, VkDisplayModeKHR mode, uint32_t planeIndex, VkDisplayPlaneCapabilitiesKHR* pCapabilities)
    /// ```
    unsafe fn get_display_plane_capabilities(
        &self,
        mode: vk::DisplayModeKHR,
        plane_index: uint32_t,
    ) -> crate::Result<DisplayPlaneCapabilitiesKHR> {
        unsafe {
            self.commands().get_display_plane_capabilities(
                self.raw(),
                mode,
                plane_index,
            )
        }
    }
    /// ```c
    /// VkResult vkGetDisplayPlaneSupportedDisplaysKHR(VkPhysicalDevice physicalDevice, uint32_t planeIndex, uint32_t* pDisplayCount, VkDisplayKHR* pDisplays)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_display_plane_supported_displays(
        &self,
        plane_index: uint32_t,
        displays: Option<&mut ::alloc::vec::Vec<Option<vk::DisplayKHR>>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_display_plane_supported_displays(
                self.raw(),
                plane_index,
                displays,
            )
        }
    }
    /// ```c
    /// VkResult vkGetPhysicalDeviceDisplayPlanePropertiesKHR(VkPhysicalDevice physicalDevice, uint32_t* pPropertyCount, VkDisplayPlanePropertiesKHR* pProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_display_plane_properties(
        &self,
        properties: Option<&mut ::alloc::vec::Vec<DisplayPlanePropertiesKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_physical_device_display_plane_properties(
                self.raw(),
                properties,
            )
        }
    }
    /// ```c
    /// VkResult vkGetPhysicalDeviceDisplayPropertiesKHR(VkPhysicalDevice physicalDevice, uint32_t* pPropertyCount, VkDisplayPropertiesKHR* pProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_display_properties(
        &self,
        properties: Option<&mut ::alloc::vec::Vec<DisplayPropertiesKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_physical_device_display_properties(
                self.raw(),
                properties,
            )
        }
    }
}
