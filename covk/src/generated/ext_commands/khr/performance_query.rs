// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_performance_query` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::khr::performance_query::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::performance_query::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, uint32_t* pCounterCount, VkPerformanceCounterKHR* pCounters, VkPerformanceCounterDescriptionKHR* pCounterDescriptions)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn enumerate_physical_device_queue_family_performance_query_counters(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: uint32_t,
        counter_count: *mut uint32_t,
        counters: Option<&mut [PerformanceCounterKHR]>,
        counter_descriptions: Option<&mut [PerformanceCounterDescriptionKHR]>,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.EnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR(
                physical_device.abi(), 
                queue_family_index.abi(), 
                counter_count.abi(), 
                counters.abi(), 
                counter_descriptions.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR(VkPhysicalDevice physicalDevice, VkQueryPoolPerformanceCreateInfoKHR const* pPerformanceQueryCreateInfo, uint32_t* pNumPasses)
    /// ```
    pub unsafe fn get_physical_device_queue_family_performance_query_passes(
        &self,
        physical_device: vk::PhysicalDevice,
        performance_query_create_info: &QueryPoolPerformanceCreateInfoKHR,
    ) -> uint32_t {
        unsafe {
            let mut _v: uint32_t = Default::default();
            self.0.GetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR(
                physical_device.abi(), 
                performance_query_create_info.abi(), 
                (&mut _v).abi(), 
            );
            _v
        }
    }
}

impl crate::CommandScope<vk::Instance> for vk::extensions::khr::performance_query {
    type Commands = Instance;
}

/// Instance object
pub trait KhrPerformanceQueryInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::khr::performance_query {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::khr::performance_query> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::khr::performance_query {
        type Output = crate::hnd::Instance<vk::extensions::khr::performance_query>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::khr::performance_query>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::khr::performance_query> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::khr::performance_query> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::khr::performance_query> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::khr::performance_query> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrPerformanceQueryInstance for crate::hnd::Instance<vk::extensions::khr::performance_query> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::performance_query, vk::Instance> for crate::hnd::Instance<vk::extensions::khr::performance_query> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait KhrPerformanceQueryPhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, uint32_t* pCounterCount, VkPerformanceCounterKHR* pCounters, VkPerformanceCounterDescriptionKHR* pCounterDescriptions)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn enumerate_physical_device_queue_family_performance_query_counters(
        &self,
        queue_family_index: uint32_t,
        counter_count: *mut uint32_t,
        counters: Option<&mut [PerformanceCounterKHR]>,
        counter_descriptions: Option<&mut [PerformanceCounterDescriptionKHR]>,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().enumerate_physical_device_queue_family_performance_query_counters(
                self.raw(),
                queue_family_index,
                counter_count,
                counters,
                counter_descriptions,
            )
        }
    }
    /// ```c
    /// void vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR(VkPhysicalDevice physicalDevice, VkQueryPoolPerformanceCreateInfoKHR const* pPerformanceQueryCreateInfo, uint32_t* pNumPasses)
    /// ```
    unsafe fn get_queue_family_performance_query_passes(
        &self,
        performance_query_create_info: &QueryPoolPerformanceCreateInfoKHR,
    ) -> uint32_t {
        unsafe {
            self.commands().get_physical_device_queue_family_performance_query_passes(
                self.raw(),
                performance_query_create_info,
            )
        }
    }
}

/// `VK_KHR_performance_query` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::performance_query::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::performance_query::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkAcquireProfilingLockKHR(VkDevice device, VkAcquireProfilingLockInfoKHR const* pInfo)
    /// ```
    pub unsafe fn acquire_profiling_lock(
        &self,
        device: vk::Device,
        info: &AcquireProfilingLockInfoKHR,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.AcquireProfilingLockKHR(
                device.abi(), 
                info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// void vkReleaseProfilingLockKHR(VkDevice device)
    /// ```
    pub unsafe fn release_profiling_lock(
        &self,
        device: vk::Device,
    ) -> () {
        unsafe {
            self.0.ReleaseProfilingLockKHR(
                device.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::performance_query {
    type Commands = Device;
}

/// Device object
pub trait KhrPerformanceQueryDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkAcquireProfilingLockKHR(VkDevice device, VkAcquireProfilingLockInfoKHR const* pInfo)
    /// ```
    unsafe fn acquire_profiling_lock(
        &self,
        info: &AcquireProfilingLockInfoKHR,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().acquire_profiling_lock(
                self.raw(),
                info,
            )
        }
    }
    /// ```c
    /// void vkReleaseProfilingLockKHR(VkDevice device)
    /// ```
    unsafe fn release_profiling_lock(
        &self,
    ) -> () {
        unsafe {
            self.commands().release_profiling_lock(
                self.raw(),
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::performance_query {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::performance_query> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::performance_query {
        type Output = crate::hnd::Device<vk::extensions::khr::performance_query>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::performance_query>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::performance_query> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::performance_query> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::performance_query> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::performance_query> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrPerformanceQueryDevice for crate::hnd::Device<vk::extensions::khr::performance_query> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::performance_query, vk::Device> for crate::hnd::Device<vk::extensions::khr::performance_query> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
