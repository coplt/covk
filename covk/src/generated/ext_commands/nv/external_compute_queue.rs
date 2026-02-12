// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_NV_external_compute_queue` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::nv::external_compute_queue::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::external_compute_queue::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkCreateExternalComputeQueueNV(VkDevice device, VkExternalComputeQueueCreateInfoNV const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkExternalComputeQueueNV* pExternalQueue)
    /// ```
    pub unsafe fn create_external_compute_queue(
        &self,
        device: vk::Device,
        create_info: &ExternalComputeQueueCreateInfoNV,
    ) -> crate::Result<vk::ExternalComputeQueueNV> {
        unsafe {
            let mut _v: Option<vk::ExternalComputeQueueNV> = Default::default();
            let _r = self.0.CreateExternalComputeQueueNV(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDestroyExternalComputeQueueNV(VkDevice device, VkExternalComputeQueueNV externalQueue, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_external_compute_queue(
        &self,
        device: vk::Device,
        external_queue: vk::ExternalComputeQueueNV,
    ) -> () {
        unsafe {
            self.0.DestroyExternalComputeQueueNV(
                device.abi(), 
                external_queue.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkGetExternalComputeQueueDataNV(VkExternalComputeQueueNV externalQueue, VkExternalComputeQueueDataParamsNV* params, void* pData)
    /// ```
    pub unsafe fn get_external_compute_queue_data(
        &self,
        external_queue: vk::ExternalComputeQueueNV,
        params: &mut ExternalComputeQueueDataParamsNV,
        data: *mut void,
    ) -> () {
        unsafe {
            self.0.GetExternalComputeQueueDataNV(
                external_queue.abi(), 
                params.abi(), 
                data.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::nv::external_compute_queue {
    type Commands = Device;
}

/// Device object
pub trait NvExternalComputeQueueDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkCreateExternalComputeQueueNV(VkDevice device, VkExternalComputeQueueCreateInfoNV const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkExternalComputeQueueNV* pExternalQueue)
    /// ```
    unsafe fn create_external_compute_queue(
        &self,
        create_info: &ExternalComputeQueueCreateInfoNV,
    ) -> crate::Result<vk::ExternalComputeQueueNV> {
        unsafe {
            self.commands().create_external_compute_queue(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// void vkDestroyExternalComputeQueueNV(VkDevice device, VkExternalComputeQueueNV externalQueue, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_external_compute_queue(
        &self,
        external_queue: vk::ExternalComputeQueueNV,
    ) -> () {
        unsafe {
            self.commands().destroy_external_compute_queue(
                self.raw(),
                external_queue,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::nv::external_compute_queue {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::nv::external_compute_queue> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::nv::external_compute_queue {
        type Output = crate::hnd::Device<vk::extensions::nv::external_compute_queue>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::nv::external_compute_queue>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::nv::external_compute_queue> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::nv::external_compute_queue> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::nv::external_compute_queue> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::nv::external_compute_queue> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvExternalComputeQueueDevice for crate::hnd::Device<vk::extensions::nv::external_compute_queue> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::external_compute_queue, vk::Device> for crate::hnd::Device<vk::extensions::nv::external_compute_queue> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// ExternalComputeQueueNV object
pub trait NvExternalComputeQueueExternalComputeQueueNV {
    fn raw(&self) -> vk::ExternalComputeQueueNV;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkGetExternalComputeQueueDataNV(VkExternalComputeQueueNV externalQueue, VkExternalComputeQueueDataParamsNV* params, void* pData)
    /// ```
    unsafe fn get_external_compute_queue_data(
        &self,
        params: &mut ExternalComputeQueueDataParamsNV,
        data: *mut void,
    ) -> () {
        unsafe {
            self.commands().get_external_compute_queue_data(
                self.raw(),
                params,
                data,
            )
        }
    }
}
