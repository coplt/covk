// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_fragment_shading_rate` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::khr::fragment_shading_rate::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::fragment_shading_rate::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkGetPhysicalDeviceFragmentShadingRatesKHR(VkPhysicalDevice physicalDevice, uint32_t* pFragmentShadingRateCount, VkPhysicalDeviceFragmentShadingRateKHR* pFragmentShadingRates)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_physical_device_fragment_shading_rates(
        &self,
        physical_device: vk::PhysicalDevice,
        fragment_shading_rates: Option<&mut ::alloc::vec::Vec<PhysicalDeviceFragmentShadingRateKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetPhysicalDeviceFragmentShadingRatesKHR(
                physical_device.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = fragment_shading_rates {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetPhysicalDeviceFragmentShadingRatesKHR(
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

impl crate::CommandScope<vk::Instance> for vk::extensions::khr::fragment_shading_rate {
    type Commands = Instance;
}

/// Instance object
pub trait KhrFragmentShadingRateInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::khr::fragment_shading_rate {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::khr::fragment_shading_rate> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::khr::fragment_shading_rate {
        type Output = crate::hnd::Instance<vk::extensions::khr::fragment_shading_rate>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::khr::fragment_shading_rate>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::khr::fragment_shading_rate> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::khr::fragment_shading_rate> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::khr::fragment_shading_rate> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::khr::fragment_shading_rate> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrFragmentShadingRateInstance for crate::hnd::Instance<vk::extensions::khr::fragment_shading_rate> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::fragment_shading_rate, vk::Instance> for crate::hnd::Instance<vk::extensions::khr::fragment_shading_rate> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait KhrFragmentShadingRatePhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkGetPhysicalDeviceFragmentShadingRatesKHR(VkPhysicalDevice physicalDevice, uint32_t* pFragmentShadingRateCount, VkPhysicalDeviceFragmentShadingRateKHR* pFragmentShadingRates)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_fragment_shading_rates(
        &self,
        fragment_shading_rates: Option<&mut ::alloc::vec::Vec<PhysicalDeviceFragmentShadingRateKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_physical_device_fragment_shading_rates(
                self.raw(),
                fragment_shading_rates,
            )
        }
    }
}

/// `VK_KHR_fragment_shading_rate` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::fragment_shading_rate::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::fragment_shading_rate::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdSetFragmentShadingRateKHR(VkCommandBuffer commandBuffer, VkExtent2D const* pFragmentSize, const VkFragmentShadingRateCombinerOpKHR combinerOps[2])
    /// ```
    pub unsafe fn cmd_set_fragment_shading_rate(
        &self,
        command_buffer: vk::CommandBuffer,
        fragment_size: &Extent2D,
        combiner_ops: [FragmentShadingRateCombinerOpKHR; 2],
    ) -> () {
        unsafe {
            self.0.CmdSetFragmentShadingRateKHR(
                command_buffer.abi(), 
                fragment_size.abi(), 
                combiner_ops.as_ref().abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::fragment_shading_rate {
    type Commands = Device;
}

/// Device object
pub trait KhrFragmentShadingRateDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::khr::fragment_shading_rate {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::fragment_shading_rate> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::fragment_shading_rate {
        type Output = crate::hnd::Device<vk::extensions::khr::fragment_shading_rate>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::fragment_shading_rate>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::fragment_shading_rate> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::fragment_shading_rate> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::fragment_shading_rate> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::fragment_shading_rate> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrFragmentShadingRateDevice for crate::hnd::Device<vk::extensions::khr::fragment_shading_rate> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::fragment_shading_rate, vk::Device> for crate::hnd::Device<vk::extensions::khr::fragment_shading_rate> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait KhrFragmentShadingRateCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdSetFragmentShadingRateKHR(VkCommandBuffer commandBuffer, VkExtent2D const* pFragmentSize, const VkFragmentShadingRateCombinerOpKHR combinerOps[2])
    /// ```
    unsafe fn set_fragment_shading_rate(
        &self,
        fragment_size: &Extent2D,
        combiner_ops: [FragmentShadingRateCombinerOpKHR; 2],
    ) -> () {
        unsafe {
            self.commands().cmd_set_fragment_shading_rate(
                self.raw(),
                fragment_size,
                combiner_ops,
            )
        }
    }
}
