// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_NV_external_memory_rdma` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::nv::external_memory_rdma::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::external_memory_rdma::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkGetMemoryRemoteAddressNV(VkDevice device, VkMemoryGetRemoteAddressInfoNV const* pMemoryGetRemoteAddressInfo, VkRemoteAddressNV* pAddress)
    /// ```
    pub unsafe fn get_memory_remote_address(
        &self,
        device: vk::Device,
        memory_get_remote_address_info: &MemoryGetRemoteAddressInfoNV,
    ) -> crate::Result<RemoteAddressNV> {
        unsafe {
            let mut _v: RemoteAddressNV = Default::default();
            let _r = self.0.GetMemoryRemoteAddressNV(
                device.abi(), 
                memory_get_remote_address_info.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::nv::external_memory_rdma {
    type Commands = Device;
}

/// Device object
pub trait NvExternalMemoryRdmaDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetMemoryRemoteAddressNV(VkDevice device, VkMemoryGetRemoteAddressInfoNV const* pMemoryGetRemoteAddressInfo, VkRemoteAddressNV* pAddress)
    /// ```
    unsafe fn get_memory_remote_address(
        &self,
        memory_get_remote_address_info: &MemoryGetRemoteAddressInfoNV,
    ) -> crate::Result<RemoteAddressNV> {
        unsafe {
            self.commands().get_memory_remote_address(
                self.raw(),
                memory_get_remote_address_info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::nv::external_memory_rdma {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::nv::external_memory_rdma> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::nv::external_memory_rdma {
        type Output = crate::hnd::Device<vk::extensions::nv::external_memory_rdma>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::nv::external_memory_rdma>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::nv::external_memory_rdma> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::nv::external_memory_rdma> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::nv::external_memory_rdma> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::nv::external_memory_rdma> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvExternalMemoryRdmaDevice for crate::hnd::Device<vk::extensions::nv::external_memory_rdma> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::external_memory_rdma, vk::Device> for crate::hnd::Device<vk::extensions::nv::external_memory_rdma> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
