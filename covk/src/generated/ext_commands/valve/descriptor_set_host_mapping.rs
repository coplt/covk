// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_VALVE_descriptor_set_host_mapping` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::valve::descriptor_set_host_mapping::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::valve::descriptor_set_host_mapping::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkGetDescriptorSetHostMappingVALVE(VkDevice device, VkDescriptorSet descriptorSet, void** ppData)
    /// ```
    pub unsafe fn get_descriptor_set_host_mapping(
        &self,
        device: vk::Device,
        descriptor_set: vk::DescriptorSet,
    ) -> *mut void {
        unsafe {
            let mut _v: *mut void = Default::default();
            self.0.GetDescriptorSetHostMappingVALVE(
                device.abi(), 
                descriptor_set.abi(), 
                (&mut _v).abi(), 
            );
            _v
        }
    }
    /// ```c
    /// void vkGetDescriptorSetLayoutHostMappingInfoVALVE(VkDevice device, VkDescriptorSetBindingReferenceVALVE const* pBindingReference, VkDescriptorSetLayoutHostMappingInfoVALVE* pHostMapping)
    /// ```
    pub unsafe fn get_descriptor_set_layout_host_mapping_info(
        &self,
        device: vk::Device,
        binding_reference: &DescriptorSetBindingReferenceVALVE,
        host_mapping: &mut DescriptorSetLayoutHostMappingInfoVALVE,
    ) -> () {
        unsafe {
            self.0.GetDescriptorSetLayoutHostMappingInfoVALVE(
                device.abi(), 
                binding_reference.abi(), 
                host_mapping.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::valve::descriptor_set_host_mapping {
    type Commands = Device;
}

/// Device object
pub trait ValveDescriptorSetHostMappingDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkGetDescriptorSetHostMappingVALVE(VkDevice device, VkDescriptorSet descriptorSet, void** ppData)
    /// ```
    unsafe fn get_descriptor_set_host_mapping(
        &self,
        descriptor_set: vk::DescriptorSet,
    ) -> *mut void {
        unsafe {
            self.commands().get_descriptor_set_host_mapping(
                self.raw(),
                descriptor_set,
            )
        }
    }
    /// ```c
    /// void vkGetDescriptorSetLayoutHostMappingInfoVALVE(VkDevice device, VkDescriptorSetBindingReferenceVALVE const* pBindingReference, VkDescriptorSetLayoutHostMappingInfoVALVE* pHostMapping)
    /// ```
    unsafe fn get_descriptor_set_layout_host_mapping_info(
        &self,
        binding_reference: &DescriptorSetBindingReferenceVALVE,
        host_mapping: &mut DescriptorSetLayoutHostMappingInfoVALVE,
    ) -> () {
        unsafe {
            self.commands().get_descriptor_set_layout_host_mapping_info(
                self.raw(),
                binding_reference,
                host_mapping,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::valve::descriptor_set_host_mapping {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::valve::descriptor_set_host_mapping> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::valve::descriptor_set_host_mapping {
        type Output = crate::hnd::Device<vk::extensions::valve::descriptor_set_host_mapping>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::valve::descriptor_set_host_mapping>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::valve::descriptor_set_host_mapping> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::valve::descriptor_set_host_mapping> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::valve::descriptor_set_host_mapping> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::valve::descriptor_set_host_mapping> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ValveDescriptorSetHostMappingDevice for crate::hnd::Device<vk::extensions::valve::descriptor_set_host_mapping> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::valve::descriptor_set_host_mapping, vk::Device> for crate::hnd::Device<vk::extensions::valve::descriptor_set_host_mapping> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
