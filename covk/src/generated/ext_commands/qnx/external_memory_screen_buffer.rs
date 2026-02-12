// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_QNX_external_memory_screen_buffer` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::qnx::external_memory_screen_buffer::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::qnx::external_memory_screen_buffer::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkGetScreenBufferPropertiesQNX(VkDevice device, _screen_buffer const* buffer, VkScreenBufferPropertiesQNX* pProperties)
    /// ```
    pub unsafe fn get_screen_buffer_properties(
        &self,
        device: vk::Device,
        buffer: *const _screen_buffer,
        properties: &mut ScreenBufferPropertiesQNX,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetScreenBufferPropertiesQNX(
                device.abi(), 
                buffer.abi(), 
                properties.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::qnx::external_memory_screen_buffer {
    type Commands = Device;
}

/// Device object
pub trait QnxExternalMemoryScreenBufferDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetScreenBufferPropertiesQNX(VkDevice device, _screen_buffer const* buffer, VkScreenBufferPropertiesQNX* pProperties)
    /// ```
    unsafe fn get_screen_buffer_properties(
        &self,
        buffer: *const _screen_buffer,
        properties: &mut ScreenBufferPropertiesQNX,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_screen_buffer_properties(
                self.raw(),
                buffer,
                properties,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::qnx::external_memory_screen_buffer {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::qnx::external_memory_screen_buffer> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::qnx::external_memory_screen_buffer {
        type Output = crate::hnd::Device<vk::extensions::qnx::external_memory_screen_buffer>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::qnx::external_memory_screen_buffer>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::qnx::external_memory_screen_buffer> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::qnx::external_memory_screen_buffer> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::qnx::external_memory_screen_buffer> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::qnx::external_memory_screen_buffer> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::QnxExternalMemoryScreenBufferDevice for crate::hnd::Device<vk::extensions::qnx::external_memory_screen_buffer> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::qnx::external_memory_screen_buffer, vk::Device> for crate::hnd::Device<vk::extensions::qnx::external_memory_screen_buffer> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
