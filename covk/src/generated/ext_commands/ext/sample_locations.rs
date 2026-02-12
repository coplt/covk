// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_sample_locations` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::ext::sample_locations::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::sample_locations::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// void vkGetPhysicalDeviceMultisamplePropertiesEXT(VkPhysicalDevice physicalDevice, VkSampleCountFlagBits samples, VkMultisamplePropertiesEXT* pMultisampleProperties)
    /// ```
    pub unsafe fn get_physical_device_multisample_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        samples: SampleCountFlags,
        multisample_properties: &mut MultisamplePropertiesEXT,
    ) -> () {
        unsafe {
            self.0.GetPhysicalDeviceMultisamplePropertiesEXT(
                physical_device.abi(), 
                samples.abi(), 
                multisample_properties.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Instance> for vk::extensions::ext::sample_locations {
    type Commands = Instance;
}

/// Instance object
pub trait ExtSampleLocationsInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::ext::sample_locations {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::ext::sample_locations> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::ext::sample_locations {
        type Output = crate::hnd::Instance<vk::extensions::ext::sample_locations>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::ext::sample_locations>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::ext::sample_locations> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::ext::sample_locations> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::ext::sample_locations> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::ext::sample_locations> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtSampleLocationsInstance for crate::hnd::Instance<vk::extensions::ext::sample_locations> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::sample_locations, vk::Instance> for crate::hnd::Instance<vk::extensions::ext::sample_locations> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait ExtSampleLocationsPhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// void vkGetPhysicalDeviceMultisamplePropertiesEXT(VkPhysicalDevice physicalDevice, VkSampleCountFlagBits samples, VkMultisamplePropertiesEXT* pMultisampleProperties)
    /// ```
    unsafe fn get_multisample_properties(
        &self,
        samples: SampleCountFlags,
        multisample_properties: &mut MultisamplePropertiesEXT,
    ) -> () {
        unsafe {
            self.commands().get_physical_device_multisample_properties(
                self.raw(),
                samples,
                multisample_properties,
            )
        }
    }
}

/// `VK_EXT_sample_locations` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::sample_locations::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::sample_locations::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdSetSampleLocationsEXT(VkCommandBuffer commandBuffer, VkSampleLocationsInfoEXT const* pSampleLocationsInfo)
    /// ```
    pub unsafe fn cmd_set_sample_locations(
        &self,
        command_buffer: vk::CommandBuffer,
        sample_locations_info: &SampleLocationsInfoEXT,
    ) -> () {
        unsafe {
            self.0.CmdSetSampleLocationsEXT(
                command_buffer.abi(), 
                sample_locations_info.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::sample_locations {
    type Commands = Device;
}

/// Device object
pub trait ExtSampleLocationsDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::ext::sample_locations {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::sample_locations> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::sample_locations {
        type Output = crate::hnd::Device<vk::extensions::ext::sample_locations>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::sample_locations>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::sample_locations> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::sample_locations> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::sample_locations> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::sample_locations> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtSampleLocationsDevice for crate::hnd::Device<vk::extensions::ext::sample_locations> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::sample_locations, vk::Device> for crate::hnd::Device<vk::extensions::ext::sample_locations> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait ExtSampleLocationsCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdSetSampleLocationsEXT(VkCommandBuffer commandBuffer, VkSampleLocationsInfoEXT const* pSampleLocationsInfo)
    /// ```
    unsafe fn set_sample_locations(
        &self,
        sample_locations_info: &SampleLocationsInfoEXT,
    ) -> () {
        unsafe {
            self.commands().cmd_set_sample_locations(
                self.raw(),
                sample_locations_info,
            )
        }
    }
}
