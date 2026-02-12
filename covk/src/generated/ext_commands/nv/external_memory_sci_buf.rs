// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_NV_external_memory_sci_buf` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::nv::external_memory_sci_buf::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::external_memory_sci_buf::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV(VkPhysicalDevice physicalDevice, VkExternalMemoryHandleTypeFlagBits handleType, NvSciBufObj handle, VkMemorySciBufPropertiesNV* pMemorySciBufProperties)
    /// ```
    pub unsafe fn get_physical_device_external_memory_sci_buf_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: NvSciBufObj,
        memory_sci_buf_properties: &mut MemorySciBufPropertiesNV,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetPhysicalDeviceExternalMemorySciBufPropertiesNV(
                physical_device.abi(), 
                handle_type.abi(), 
                handle.abi(), 
                memory_sci_buf_properties.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkGetPhysicalDeviceSciBufAttributesNV(VkPhysicalDevice physicalDevice, NvSciBufAttrList pAttributes)
    /// ```
    pub unsafe fn get_physical_device_sci_buf_attributes(
        &self,
        physical_device: vk::PhysicalDevice,
        attributes: NvSciBufAttrList,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetPhysicalDeviceSciBufAttributesNV(
                physical_device.abi(), 
                attributes.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Instance> for vk::extensions::nv::external_memory_sci_buf {
    type Commands = Instance;
}

/// Instance object
pub trait NvExternalMemorySciBufInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::nv::external_memory_sci_buf {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::nv::external_memory_sci_buf> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::nv::external_memory_sci_buf {
        type Output = crate::hnd::Instance<vk::extensions::nv::external_memory_sci_buf>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::nv::external_memory_sci_buf>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::nv::external_memory_sci_buf> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::nv::external_memory_sci_buf> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::nv::external_memory_sci_buf> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::nv::external_memory_sci_buf> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvExternalMemorySciBufInstance for crate::hnd::Instance<vk::extensions::nv::external_memory_sci_buf> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::external_memory_sci_buf, vk::Instance> for crate::hnd::Instance<vk::extensions::nv::external_memory_sci_buf> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait NvExternalMemorySciBufPhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV(VkPhysicalDevice physicalDevice, VkExternalMemoryHandleTypeFlagBits handleType, NvSciBufObj handle, VkMemorySciBufPropertiesNV* pMemorySciBufProperties)
    /// ```
    unsafe fn get_external_memory_sci_buf_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: NvSciBufObj,
        memory_sci_buf_properties: &mut MemorySciBufPropertiesNV,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_physical_device_external_memory_sci_buf_properties(
                self.raw(),
                handle_type,
                handle,
                memory_sci_buf_properties,
            )
        }
    }
    /// ```c
    /// VkResult vkGetPhysicalDeviceSciBufAttributesNV(VkPhysicalDevice physicalDevice, NvSciBufAttrList pAttributes)
    /// ```
    unsafe fn get_sci_buf_attributes(
        &self,
        attributes: NvSciBufAttrList,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_physical_device_sci_buf_attributes(
                self.raw(),
                attributes,
            )
        }
    }
}

/// `VK_NV_external_memory_sci_buf` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::nv::external_memory_sci_buf::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::external_memory_sci_buf::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkGetMemorySciBufNV(VkDevice device, VkMemoryGetSciBufInfoNV const* pGetSciBufInfo, NvSciBufObj* pHandle)
    /// ```
    pub unsafe fn get_memory_sci_buf(
        &self,
        device: vk::Device,
        get_sci_buf_info: &MemoryGetSciBufInfoNV,
    ) -> crate::Result<NvSciBufObj> {
        unsafe {
            let mut _v: NvSciBufObj = Default::default();
            let _r = self.0.GetMemorySciBufNV(
                device.abi(), 
                get_sci_buf_info.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::nv::external_memory_sci_buf {
    type Commands = Device;
}

/// Device object
pub trait NvExternalMemorySciBufDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetMemorySciBufNV(VkDevice device, VkMemoryGetSciBufInfoNV const* pGetSciBufInfo, NvSciBufObj* pHandle)
    /// ```
    unsafe fn get_memory_sci_buf(
        &self,
        get_sci_buf_info: &MemoryGetSciBufInfoNV,
    ) -> crate::Result<NvSciBufObj> {
        unsafe {
            self.commands().get_memory_sci_buf(
                self.raw(),
                get_sci_buf_info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::nv::external_memory_sci_buf {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::nv::external_memory_sci_buf> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::nv::external_memory_sci_buf {
        type Output = crate::hnd::Device<vk::extensions::nv::external_memory_sci_buf>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::nv::external_memory_sci_buf>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::nv::external_memory_sci_buf> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::nv::external_memory_sci_buf> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::nv::external_memory_sci_buf> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::nv::external_memory_sci_buf> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvExternalMemorySciBufDevice for crate::hnd::Device<vk::extensions::nv::external_memory_sci_buf> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::external_memory_sci_buf, vk::Device> for crate::hnd::Device<vk::extensions::nv::external_memory_sci_buf> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
