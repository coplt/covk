// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_calibrated_timestamps` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::ext::calibrated_timestamps::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::calibrated_timestamps::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkGetPhysicalDeviceCalibrateableTimeDomainsKHR(VkPhysicalDevice physicalDevice, uint32_t* pTimeDomainCount, VkTimeDomainKHR* pTimeDomains)
    /// ```
    pub unsafe fn get_physical_device_calibrateable_time_domains(
        &self,
        physical_device: vk::PhysicalDevice,
        time_domains: Option<&mut ::alloc::vec::Vec<TimeDomainKHR>>,
    ) -> crate::Result<uint32_t> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetPhysicalDeviceCalibrateableTimeDomainsEXT(
                physical_device.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = time_domains {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetPhysicalDeviceCalibrateableTimeDomainsEXT(
                    physical_device.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                ).vk();
                _r.result(|| Some(()))?;
                _b.set_len(_b.len() + _c as usize);
            }
            _r.result(|| Some(_c))
        }
    }
}

impl crate::CommandScope<vk::Instance> for vk::extensions::ext::calibrated_timestamps {
    type Commands = Instance;
}

/// Instance object
pub trait ExtCalibratedTimestampsInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::ext::calibrated_timestamps {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::ext::calibrated_timestamps> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::ext::calibrated_timestamps {
        type Output = crate::hnd::Instance<vk::extensions::ext::calibrated_timestamps>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::ext::calibrated_timestamps>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::ext::calibrated_timestamps> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::ext::calibrated_timestamps> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::ext::calibrated_timestamps> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::ext::calibrated_timestamps> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtCalibratedTimestampsInstance for crate::hnd::Instance<vk::extensions::ext::calibrated_timestamps> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::calibrated_timestamps, vk::Instance> for crate::hnd::Instance<vk::extensions::ext::calibrated_timestamps> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait ExtCalibratedTimestampsPhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkGetPhysicalDeviceCalibrateableTimeDomainsKHR(VkPhysicalDevice physicalDevice, uint32_t* pTimeDomainCount, VkTimeDomainKHR* pTimeDomains)
    /// ```
    unsafe fn get_calibrateable_time_domains(
        &self,
        time_domains: Option<&mut ::alloc::vec::Vec<TimeDomainKHR>>,
    ) -> crate::Result<uint32_t> {
        unsafe {
            self.commands().get_physical_device_calibrateable_time_domains(
                self.raw(),
                time_domains,
            )
        }
    }
}

/// `VK_EXT_calibrated_timestamps` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::calibrated_timestamps::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::calibrated_timestamps::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkGetCalibratedTimestampsKHR(VkDevice device, uint32_t timestampCount, VkCalibratedTimestampInfoKHR const* pTimestampInfos, uint64_t* pTimestamps, uint64_t* pMaxDeviation)
    /// ```
    pub unsafe fn get_calibrated_timestamps(
        &self,
        device: vk::Device,
        timestamp_infos: &[CalibratedTimestampInfoKHR],
        timestamps: &mut [uint64_t],
    ) -> crate::Result<uint64_t> {
        unsafe {
            let mut _v: uint64_t = Default::default();
            let _r = self.0.GetCalibratedTimestampsEXT(
                device.abi(), 
                timestamp_infos.len() as _, 
                timestamp_infos.abi(), 
                timestamps.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::calibrated_timestamps {
    type Commands = Device;
}

/// Device object
pub trait ExtCalibratedTimestampsDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetCalibratedTimestampsKHR(VkDevice device, uint32_t timestampCount, VkCalibratedTimestampInfoKHR const* pTimestampInfos, uint64_t* pTimestamps, uint64_t* pMaxDeviation)
    /// ```
    unsafe fn get_calibrated_timestamps(
        &self,
        timestamp_infos: &[CalibratedTimestampInfoKHR],
        timestamps: &mut [uint64_t],
    ) -> crate::Result<uint64_t> {
        unsafe {
            self.commands().get_calibrated_timestamps(
                self.raw(),
                timestamp_infos,
                timestamps,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::ext::calibrated_timestamps {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::calibrated_timestamps> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::calibrated_timestamps {
        type Output = crate::hnd::Device<vk::extensions::ext::calibrated_timestamps>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::calibrated_timestamps>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::calibrated_timestamps> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::calibrated_timestamps> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::calibrated_timestamps> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::calibrated_timestamps> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtCalibratedTimestampsDevice for crate::hnd::Device<vk::extensions::ext::calibrated_timestamps> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::calibrated_timestamps, vk::Device> for crate::hnd::Device<vk::extensions::ext::calibrated_timestamps> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
