// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_maintenance4` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::maintenance4::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::maintenance4::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkGetDeviceBufferMemoryRequirements(VkDevice device, VkDeviceBufferMemoryRequirements const* pInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    pub unsafe fn get_device_buffer_memory_requirements(
        &self,
        device: vk::Device,
        info: &DeviceBufferMemoryRequirements,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self.0.GetDeviceBufferMemoryRequirementsKHR(
                device.abi(), 
                info.abi(), 
                memory_requirements.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetDeviceImageMemoryRequirements(VkDevice device, VkDeviceImageMemoryRequirements const* pInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    pub unsafe fn get_device_image_memory_requirements(
        &self,
        device: vk::Device,
        info: &DeviceImageMemoryRequirements,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self.0.GetDeviceImageMemoryRequirementsKHR(
                device.abi(), 
                info.abi(), 
                memory_requirements.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetDeviceImageSparseMemoryRequirements(VkDevice device, VkDeviceImageMemoryRequirements const* pInfo, uint32_t* pSparseMemoryRequirementCount, VkSparseImageMemoryRequirements2* pSparseMemoryRequirements)
    /// ```
    pub unsafe fn get_device_image_sparse_memory_requirements(
        &self,
        device: vk::Device,
        info: &DeviceImageMemoryRequirements,
        sparse_memory_requirements: Option<&mut ::alloc::vec::Vec<SparseImageMemoryRequirements2>>,
    ) -> uint32_t {
        unsafe {
            let mut _c: uint32_t = Default::default();
            self.0.GetDeviceImageSparseMemoryRequirementsKHR(
                device.abi(), 
                info.abi(), 
                &mut _c, 
                Default::default(), 
            );
            if let Some(_b) = sparse_memory_requirements {
                _b.reserve(_c as usize);
                self.0.GetDeviceImageSparseMemoryRequirementsKHR(
                    device.abi(), 
                    info.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                );
                _b.set_len(_b.len() + _c as usize);
            }
            _c
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::maintenance4 {
    type Commands = Device;
}

/// Device object
pub trait KhrMaintenance4Device {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkGetDeviceBufferMemoryRequirements(VkDevice device, VkDeviceBufferMemoryRequirements const* pInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    unsafe fn get_device_buffer_memory_requirements(
        &self,
        info: &DeviceBufferMemoryRequirements,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self.commands().get_device_buffer_memory_requirements(
                self.raw(),
                info,
                memory_requirements,
            )
        }
    }
    /// ```c
    /// void vkGetDeviceImageMemoryRequirements(VkDevice device, VkDeviceImageMemoryRequirements const* pInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    unsafe fn get_device_image_memory_requirements(
        &self,
        info: &DeviceImageMemoryRequirements,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self.commands().get_device_image_memory_requirements(
                self.raw(),
                info,
                memory_requirements,
            )
        }
    }
    /// ```c
    /// void vkGetDeviceImageSparseMemoryRequirements(VkDevice device, VkDeviceImageMemoryRequirements const* pInfo, uint32_t* pSparseMemoryRequirementCount, VkSparseImageMemoryRequirements2* pSparseMemoryRequirements)
    /// ```
    unsafe fn get_device_image_sparse_memory_requirements(
        &self,
        info: &DeviceImageMemoryRequirements,
        sparse_memory_requirements: Option<&mut ::alloc::vec::Vec<SparseImageMemoryRequirements2>>,
    ) -> uint32_t {
        unsafe {
            self.commands().get_device_image_sparse_memory_requirements(
                self.raw(),
                info,
                sparse_memory_requirements,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::maintenance4 {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::maintenance4> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::maintenance4 {
        type Output = crate::hnd::Device<vk::extensions::khr::maintenance4>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::maintenance4>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::maintenance4> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::maintenance4> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::maintenance4> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::maintenance4> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrMaintenance4Device for crate::hnd::Device<vk::extensions::khr::maintenance4> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::maintenance4, vk::Device> for crate::hnd::Device<vk::extensions::khr::maintenance4> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
