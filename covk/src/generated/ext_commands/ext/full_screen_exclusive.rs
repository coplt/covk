// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_full_screen_exclusive` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::ext::full_screen_exclusive::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::full_screen_exclusive::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkGetPhysicalDeviceSurfacePresentModes2EXT(VkPhysicalDevice physicalDevice, VkPhysicalDeviceSurfaceInfo2KHR const* pSurfaceInfo, uint32_t* pPresentModeCount, VkPresentModeKHR* pPresentModes)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_physical_device_surface_present_modes2(
        &self,
        physical_device: vk::PhysicalDevice,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
        present_modes: Option<&mut ::alloc::vec::Vec<PresentModeKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetPhysicalDeviceSurfacePresentModes2EXT(
                physical_device.abi(), 
                surface_info.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = present_modes {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetPhysicalDeviceSurfacePresentModes2EXT(
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

impl crate::CommandScope<vk::Instance> for vk::extensions::ext::full_screen_exclusive {
    type Commands = Instance;
}

/// Instance object
pub trait ExtFullScreenExclusiveInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::ext::full_screen_exclusive {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::ext::full_screen_exclusive> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::ext::full_screen_exclusive {
        type Output = crate::hnd::Instance<vk::extensions::ext::full_screen_exclusive>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::ext::full_screen_exclusive>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::ext::full_screen_exclusive> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::ext::full_screen_exclusive> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::ext::full_screen_exclusive> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::ext::full_screen_exclusive> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtFullScreenExclusiveInstance for crate::hnd::Instance<vk::extensions::ext::full_screen_exclusive> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::full_screen_exclusive, vk::Instance> for crate::hnd::Instance<vk::extensions::ext::full_screen_exclusive> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait ExtFullScreenExclusivePhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkGetPhysicalDeviceSurfacePresentModes2EXT(VkPhysicalDevice physicalDevice, VkPhysicalDeviceSurfaceInfo2KHR const* pSurfaceInfo, uint32_t* pPresentModeCount, VkPresentModeKHR* pPresentModes)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_surface_present_modes2(
        &self,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
        present_modes: Option<&mut ::alloc::vec::Vec<PresentModeKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_physical_device_surface_present_modes2(
                self.raw(),
                surface_info,
                present_modes,
            )
        }
    }
}

/// `VK_EXT_full_screen_exclusive` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::full_screen_exclusive::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::full_screen_exclusive::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkAcquireFullScreenExclusiveModeEXT(VkDevice device, VkSwapchainKHR swapchain)
    /// ```
    pub unsafe fn acquire_full_screen_exclusive_mode(
        &self,
        device: vk::Device,
        swapchain: vk::SwapchainKHR,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.AcquireFullScreenExclusiveModeEXT(
                device.abi(), 
                swapchain.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkGetDeviceGroupSurfacePresentModes2EXT(VkDevice device, VkPhysicalDeviceSurfaceInfo2KHR const* pSurfaceInfo, VkDeviceGroupPresentModeFlagsKHR* pModes)
    /// ```
    pub unsafe fn get_device_group_surface_present_modes2(
        &self,
        device: vk::Device,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
    ) -> crate::Result<DeviceGroupPresentModeFlagsKHR> {
        unsafe {
            let mut _v: DeviceGroupPresentModeFlagsKHR = Default::default();
            let _r = self.0.GetDeviceGroupSurfacePresentModes2EXT(
                device.abi(), 
                surface_info.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
    /// ```c
    /// VkResult vkReleaseFullScreenExclusiveModeEXT(VkDevice device, VkSwapchainKHR swapchain)
    /// ```
    pub unsafe fn release_full_screen_exclusive_mode(
        &self,
        device: vk::Device,
        swapchain: vk::SwapchainKHR,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.ReleaseFullScreenExclusiveModeEXT(
                device.abi(), 
                swapchain.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::full_screen_exclusive {
    type Commands = Device;
}

/// Device object
pub trait ExtFullScreenExclusiveDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkAcquireFullScreenExclusiveModeEXT(VkDevice device, VkSwapchainKHR swapchain)
    /// ```
    unsafe fn acquire_full_screen_exclusive_mode(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().acquire_full_screen_exclusive_mode(
                self.raw(),
                swapchain,
            )
        }
    }
    /// ```c
    /// VkResult vkGetDeviceGroupSurfacePresentModes2EXT(VkDevice device, VkPhysicalDeviceSurfaceInfo2KHR const* pSurfaceInfo, VkDeviceGroupPresentModeFlagsKHR* pModes)
    /// ```
    unsafe fn get_device_group_surface_present_modes2(
        &self,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
    ) -> crate::Result<DeviceGroupPresentModeFlagsKHR> {
        unsafe {
            self.commands().get_device_group_surface_present_modes2(
                self.raw(),
                surface_info,
            )
        }
    }
    /// ```c
    /// VkResult vkReleaseFullScreenExclusiveModeEXT(VkDevice device, VkSwapchainKHR swapchain)
    /// ```
    unsafe fn release_full_screen_exclusive_mode(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().release_full_screen_exclusive_mode(
                self.raw(),
                swapchain,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::ext::full_screen_exclusive {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::full_screen_exclusive> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::full_screen_exclusive {
        type Output = crate::hnd::Device<vk::extensions::ext::full_screen_exclusive>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::full_screen_exclusive>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::full_screen_exclusive> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::full_screen_exclusive> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::full_screen_exclusive> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::full_screen_exclusive> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtFullScreenExclusiveDevice for crate::hnd::Device<vk::extensions::ext::full_screen_exclusive> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::full_screen_exclusive, vk::Device> for crate::hnd::Device<vk::extensions::ext::full_screen_exclusive> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
