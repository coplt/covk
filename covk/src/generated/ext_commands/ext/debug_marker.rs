// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_debug_marker` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::debug_marker::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::debug_marker::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdDebugMarkerBeginEXT(VkCommandBuffer commandBuffer, VkDebugMarkerMarkerInfoEXT const* pMarkerInfo)
    /// ```
    pub unsafe fn cmd_debug_marker_begin(
        &self,
        command_buffer: vk::CommandBuffer,
        marker_info: &DebugMarkerMarkerInfoEXT,
    ) -> () {
        unsafe {
            self.0.CmdDebugMarkerBeginEXT(
                command_buffer.abi(), 
                marker_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdDebugMarkerEndEXT(VkCommandBuffer commandBuffer)
    /// ```
    pub unsafe fn cmd_debug_marker_end(
        &self,
        command_buffer: vk::CommandBuffer,
    ) -> () {
        unsafe {
            self.0.CmdDebugMarkerEndEXT(
                command_buffer.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdDebugMarkerInsertEXT(VkCommandBuffer commandBuffer, VkDebugMarkerMarkerInfoEXT const* pMarkerInfo)
    /// ```
    pub unsafe fn cmd_debug_marker_insert(
        &self,
        command_buffer: vk::CommandBuffer,
        marker_info: &DebugMarkerMarkerInfoEXT,
    ) -> () {
        unsafe {
            self.0.CmdDebugMarkerInsertEXT(
                command_buffer.abi(), 
                marker_info.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkDebugMarkerSetObjectNameEXT(VkDevice device, VkDebugMarkerObjectNameInfoEXT const* pNameInfo)
    /// ```
    pub unsafe fn debug_marker_set_object_name(
        &self,
        device: vk::Device,
        name_info: &DebugMarkerObjectNameInfoEXT,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.DebugMarkerSetObjectNameEXT(
                device.abi(), 
                name_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkDebugMarkerSetObjectTagEXT(VkDevice device, VkDebugMarkerObjectTagInfoEXT const* pTagInfo)
    /// ```
    pub unsafe fn debug_marker_set_object_tag(
        &self,
        device: vk::Device,
        tag_info: &DebugMarkerObjectTagInfoEXT,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.DebugMarkerSetObjectTagEXT(
                device.abi(), 
                tag_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::debug_marker {
    type Commands = Device;
}

/// Device object
pub trait ExtDebugMarkerDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkDebugMarkerSetObjectNameEXT(VkDevice device, VkDebugMarkerObjectNameInfoEXT const* pNameInfo)
    /// ```
    unsafe fn debug_marker_set_object_name(
        &self,
        name_info: &DebugMarkerObjectNameInfoEXT,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().debug_marker_set_object_name(
                self.raw(),
                name_info,
            )
        }
    }
    /// ```c
    /// VkResult vkDebugMarkerSetObjectTagEXT(VkDevice device, VkDebugMarkerObjectTagInfoEXT const* pTagInfo)
    /// ```
    unsafe fn debug_marker_set_object_tag(
        &self,
        tag_info: &DebugMarkerObjectTagInfoEXT,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().debug_marker_set_object_tag(
                self.raw(),
                tag_info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::ext::debug_marker {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::debug_marker> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::debug_marker {
        type Output = crate::hnd::Device<vk::extensions::ext::debug_marker>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::debug_marker>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::debug_marker> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::debug_marker> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::debug_marker> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::debug_marker> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtDebugMarkerDevice for crate::hnd::Device<vk::extensions::ext::debug_marker> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::debug_marker, vk::Device> for crate::hnd::Device<vk::extensions::ext::debug_marker> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait ExtDebugMarkerCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdDebugMarkerBeginEXT(VkCommandBuffer commandBuffer, VkDebugMarkerMarkerInfoEXT const* pMarkerInfo)
    /// ```
    unsafe fn debug_marker_begin(
        &self,
        marker_info: &DebugMarkerMarkerInfoEXT,
    ) -> () {
        unsafe {
            self.commands().cmd_debug_marker_begin(
                self.raw(),
                marker_info,
            )
        }
    }
    /// ```c
    /// void vkCmdDebugMarkerEndEXT(VkCommandBuffer commandBuffer)
    /// ```
    unsafe fn debug_marker_end(
        &self,
    ) -> () {
        unsafe {
            self.commands().cmd_debug_marker_end(
                self.raw(),
            )
        }
    }
    /// ```c
    /// void vkCmdDebugMarkerInsertEXT(VkCommandBuffer commandBuffer, VkDebugMarkerMarkerInfoEXT const* pMarkerInfo)
    /// ```
    unsafe fn debug_marker_insert(
        &self,
        marker_info: &DebugMarkerMarkerInfoEXT,
    ) -> () {
        unsafe {
            self.commands().cmd_debug_marker_insert(
                self.raw(),
                marker_info,
            )
        }
    }
}
