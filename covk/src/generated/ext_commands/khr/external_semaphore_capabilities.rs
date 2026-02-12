// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_external_semaphore_capabilities` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::khr::external_semaphore_capabilities::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::external_semaphore_capabilities::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// void vkGetPhysicalDeviceExternalSemaphoreProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceExternalSemaphoreInfo const* pExternalSemaphoreInfo, VkExternalSemaphoreProperties* pExternalSemaphoreProperties)
    /// ```
    pub unsafe fn get_physical_device_external_semaphore_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        external_semaphore_info: &PhysicalDeviceExternalSemaphoreInfo,
        external_semaphore_properties: &mut ExternalSemaphoreProperties,
    ) -> () {
        unsafe {
            self.0.GetPhysicalDeviceExternalSemaphorePropertiesKHR(
                physical_device.abi(), 
                external_semaphore_info.abi(), 
                external_semaphore_properties.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Instance> for vk::extensions::khr::external_semaphore_capabilities {
    type Commands = Instance;
}

/// Instance object
pub trait KhrExternalSemaphoreCapabilitiesInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::khr::external_semaphore_capabilities {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::khr::external_semaphore_capabilities> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::khr::external_semaphore_capabilities {
        type Output = crate::hnd::Instance<vk::extensions::khr::external_semaphore_capabilities>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::khr::external_semaphore_capabilities>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::khr::external_semaphore_capabilities> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::khr::external_semaphore_capabilities> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::khr::external_semaphore_capabilities> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::khr::external_semaphore_capabilities> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrExternalSemaphoreCapabilitiesInstance for crate::hnd::Instance<vk::extensions::khr::external_semaphore_capabilities> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::external_semaphore_capabilities, vk::Instance> for crate::hnd::Instance<vk::extensions::khr::external_semaphore_capabilities> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait KhrExternalSemaphoreCapabilitiesPhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// void vkGetPhysicalDeviceExternalSemaphoreProperties(VkPhysicalDevice physicalDevice, VkPhysicalDeviceExternalSemaphoreInfo const* pExternalSemaphoreInfo, VkExternalSemaphoreProperties* pExternalSemaphoreProperties)
    /// ```
    unsafe fn get_external_semaphore_properties(
        &self,
        external_semaphore_info: &PhysicalDeviceExternalSemaphoreInfo,
        external_semaphore_properties: &mut ExternalSemaphoreProperties,
    ) -> () {
        unsafe {
            self.commands().get_physical_device_external_semaphore_properties(
                self.raw(),
                external_semaphore_info,
                external_semaphore_properties,
            )
        }
    }
}
