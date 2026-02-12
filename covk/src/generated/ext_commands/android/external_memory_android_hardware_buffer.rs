// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_ANDROID_external_memory_android_hardware_buffer` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::android::external_memory_android_hardware_buffer::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::android::external_memory_android_hardware_buffer::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkGetAndroidHardwareBufferPropertiesANDROID(VkDevice device, AHardwareBuffer const* buffer, VkAndroidHardwareBufferPropertiesANDROID* pProperties)
    /// ```
    pub unsafe fn get_android_hardware_buffer_properties(
        &self,
        device: vk::Device,
        buffer: *const AHardwareBuffer,
        properties: &mut AndroidHardwareBufferPropertiesANDROID,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetAndroidHardwareBufferPropertiesANDROID(
                device.abi(), 
                buffer.abi(), 
                properties.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkGetMemoryAndroidHardwareBufferANDROID(VkDevice device, VkMemoryGetAndroidHardwareBufferInfoANDROID const* pInfo, AHardwareBuffer** pBuffer)
    /// ```
    pub unsafe fn get_memory_android_hardware_buffer(
        &self,
        device: vk::Device,
        info: &MemoryGetAndroidHardwareBufferInfoANDROID,
    ) -> crate::Result<*mut AHardwareBuffer> {
        unsafe {
            let mut _v: *mut AHardwareBuffer = Default::default();
            let _r = self.0.GetMemoryAndroidHardwareBufferANDROID(
                device.abi(), 
                info.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::android::external_memory_android_hardware_buffer {
    type Commands = Device;
}

/// Device object
pub trait AndroidExternalMemoryAndroidHardwareBufferDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetAndroidHardwareBufferPropertiesANDROID(VkDevice device, AHardwareBuffer const* buffer, VkAndroidHardwareBufferPropertiesANDROID* pProperties)
    /// ```
    unsafe fn get_android_hardware_buffer_properties(
        &self,
        buffer: *const AHardwareBuffer,
        properties: &mut AndroidHardwareBufferPropertiesANDROID,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_android_hardware_buffer_properties(
                self.raw(),
                buffer,
                properties,
            )
        }
    }
    /// ```c
    /// VkResult vkGetMemoryAndroidHardwareBufferANDROID(VkDevice device, VkMemoryGetAndroidHardwareBufferInfoANDROID const* pInfo, AHardwareBuffer** pBuffer)
    /// ```
    unsafe fn get_memory_android_hardware_buffer(
        &self,
        info: &MemoryGetAndroidHardwareBufferInfoANDROID,
    ) -> crate::Result<*mut AHardwareBuffer> {
        unsafe {
            self.commands().get_memory_android_hardware_buffer(
                self.raw(),
                info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::android::external_memory_android_hardware_buffer {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::android::external_memory_android_hardware_buffer> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::android::external_memory_android_hardware_buffer {
        type Output = crate::hnd::Device<vk::extensions::android::external_memory_android_hardware_buffer>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::android::external_memory_android_hardware_buffer>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::android::external_memory_android_hardware_buffer> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::android::external_memory_android_hardware_buffer> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::android::external_memory_android_hardware_buffer> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::android::external_memory_android_hardware_buffer> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::AndroidExternalMemoryAndroidHardwareBufferDevice for crate::hnd::Device<vk::extensions::android::external_memory_android_hardware_buffer> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::android::external_memory_android_hardware_buffer, vk::Device> for crate::hnd::Device<vk::extensions::android::external_memory_android_hardware_buffer> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
