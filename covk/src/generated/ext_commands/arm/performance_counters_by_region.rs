// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_ARM_performance_counters_by_region` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::arm::performance_counters_by_region::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::arm::performance_counters_by_region::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, uint32_t* pCounterCount, VkPerformanceCounterARM* pCounters, VkPerformanceCounterDescriptionARM* pCounterDescriptions)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn enumerate_physical_device_queue_family_performance_counters_by_region(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: uint32_t,
        counter_count: *mut uint32_t,
        counters: Option<&mut [PerformanceCounterARM]>,
        counter_descriptions: Option<&mut [PerformanceCounterDescriptionARM]>,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.EnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM(
                physical_device.abi(), 
                queue_family_index.abi(), 
                counter_count.abi(), 
                counters.abi(), 
                counter_descriptions.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
}

impl crate::CommandScope<vk::Instance> for vk::extensions::arm::performance_counters_by_region {
    type Commands = Instance;
}

/// Instance object
pub trait ArmPerformanceCountersByRegionInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::arm::performance_counters_by_region {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::arm::performance_counters_by_region> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::arm::performance_counters_by_region {
        type Output = crate::hnd::Instance<vk::extensions::arm::performance_counters_by_region>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::arm::performance_counters_by_region>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::arm::performance_counters_by_region> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::arm::performance_counters_by_region> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::arm::performance_counters_by_region> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::arm::performance_counters_by_region> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ArmPerformanceCountersByRegionInstance for crate::hnd::Instance<vk::extensions::arm::performance_counters_by_region> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::arm::performance_counters_by_region, vk::Instance> for crate::hnd::Instance<vk::extensions::arm::performance_counters_by_region> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait ArmPerformanceCountersByRegionPhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, uint32_t* pCounterCount, VkPerformanceCounterARM* pCounters, VkPerformanceCounterDescriptionARM* pCounterDescriptions)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn enumerate_physical_device_queue_family_performance_counters_by_region(
        &self,
        queue_family_index: uint32_t,
        counter_count: *mut uint32_t,
        counters: Option<&mut [PerformanceCounterARM]>,
        counter_descriptions: Option<&mut [PerformanceCounterDescriptionARM]>,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().enumerate_physical_device_queue_family_performance_counters_by_region(
                self.raw(),
                queue_family_index,
                counter_count,
                counters,
                counter_descriptions,
            )
        }
    }
}
