// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_map_memory2` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::map_memory2::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::map_memory2::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkMapMemory2(VkDevice device, VkMemoryMapInfo const* pMemoryMapInfo, void** ppData)
    /// ```
    pub unsafe fn map_memory2(
        &self,
        device: vk::Device,
        memory_map_info: &MemoryMapInfo,
        p_data: *mut *mut void,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.MapMemory2KHR(
                device.abi(), 
                memory_map_info.abi(), 
                p_data.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkUnmapMemory2(VkDevice device, VkMemoryUnmapInfo const* pMemoryUnmapInfo)
    /// ```
    pub unsafe fn unmap_memory2(
        &self,
        device: vk::Device,
        memory_unmap_info: &MemoryUnmapInfo,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.UnmapMemory2KHR(
                device.abi(), 
                memory_unmap_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::map_memory2 {
    type Commands = Device;
}

/// Device object
pub trait KhrMapMemory2Device {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkMapMemory2(VkDevice device, VkMemoryMapInfo const* pMemoryMapInfo, void** ppData)
    /// ```
    unsafe fn map_memory2(
        &self,
        memory_map_info: &MemoryMapInfo,
        p_data: *mut *mut void,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().map_memory2(
                self.raw(),
                memory_map_info,
                p_data,
            )
        }
    }
    /// ```c
    /// VkResult vkUnmapMemory2(VkDevice device, VkMemoryUnmapInfo const* pMemoryUnmapInfo)
    /// ```
    unsafe fn unmap_memory2(
        &self,
        memory_unmap_info: &MemoryUnmapInfo,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().unmap_memory2(
                self.raw(),
                memory_unmap_info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::map_memory2 {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::map_memory2> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::map_memory2 {
        type Output = crate::hnd::Device<vk::extensions::khr::map_memory2>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::map_memory2>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::map_memory2> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::map_memory2> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::map_memory2> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::map_memory2> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrMapMemory2Device for crate::hnd::Device<vk::extensions::khr::map_memory2> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::map_memory2, vk::Device> for crate::hnd::Device<vk::extensions::khr::map_memory2> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
