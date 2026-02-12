// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_fragment_density_map_offset` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::fragment_density_map_offset::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::fragment_density_map_offset::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdEndRendering2KHR(VkCommandBuffer commandBuffer, VkRenderingEndInfoKHR const* pRenderingEndInfo)
    /// ```
    pub unsafe fn cmd_end_rendering2(
        &self,
        command_buffer: vk::CommandBuffer,
        rendering_end_info: Option<&RenderingEndInfoKHR>,
    ) -> () {
        unsafe {
            self.0.CmdEndRendering2EXT(
                command_buffer.abi(), 
                rendering_end_info.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::fragment_density_map_offset {
    type Commands = Device;
}

/// Device object
pub trait ExtFragmentDensityMapOffsetDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::ext::fragment_density_map_offset {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::fragment_density_map_offset> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::fragment_density_map_offset {
        type Output = crate::hnd::Device<vk::extensions::ext::fragment_density_map_offset>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::fragment_density_map_offset>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::fragment_density_map_offset> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::fragment_density_map_offset> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::fragment_density_map_offset> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::fragment_density_map_offset> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtFragmentDensityMapOffsetDevice for crate::hnd::Device<vk::extensions::ext::fragment_density_map_offset> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::fragment_density_map_offset, vk::Device> for crate::hnd::Device<vk::extensions::ext::fragment_density_map_offset> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait ExtFragmentDensityMapOffsetCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdEndRendering2KHR(VkCommandBuffer commandBuffer, VkRenderingEndInfoKHR const* pRenderingEndInfo)
    /// ```
    unsafe fn end_rendering2(
        &self,
        rendering_end_info: Option<&RenderingEndInfoKHR>,
    ) -> () {
        unsafe {
            self.commands().cmd_end_rendering2(
                self.raw(),
                rendering_end_info,
            )
        }
    }
}
