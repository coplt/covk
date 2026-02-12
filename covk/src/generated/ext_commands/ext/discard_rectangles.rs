// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_discard_rectangles` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::discard_rectangles::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::discard_rectangles::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdSetDiscardRectangleEXT(VkCommandBuffer commandBuffer, uint32_t firstDiscardRectangle, uint32_t discardRectangleCount, VkRect2D const* pDiscardRectangles)
    /// ```
    pub unsafe fn cmd_set_discard_rectangle(
        &self,
        command_buffer: vk::CommandBuffer,
        first_discard_rectangle: uint32_t,
        discard_rectangles: &[Rect2D],
    ) -> () {
        unsafe {
            self.0.CmdSetDiscardRectangleEXT(
                command_buffer.abi(), 
                first_discard_rectangle.abi(), 
                discard_rectangles.len() as _, 
                discard_rectangles.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetDiscardRectangleEnableEXT(VkCommandBuffer commandBuffer, VkBool32 discardRectangleEnable)
    /// ```
    pub unsafe fn cmd_set_discard_rectangle_enable(
        &self,
        command_buffer: vk::CommandBuffer,
        discard_rectangle_enable: bool,
    ) -> () {
        unsafe {
            self.0.CmdSetDiscardRectangleEnableEXT(
                command_buffer.abi(), 
                discard_rectangle_enable.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetDiscardRectangleModeEXT(VkCommandBuffer commandBuffer, VkDiscardRectangleModeEXT discardRectangleMode)
    /// ```
    pub unsafe fn cmd_set_discard_rectangle_mode(
        &self,
        command_buffer: vk::CommandBuffer,
        discard_rectangle_mode: DiscardRectangleModeEXT,
    ) -> () {
        unsafe {
            self.0.CmdSetDiscardRectangleModeEXT(
                command_buffer.abi(), 
                discard_rectangle_mode.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::discard_rectangles {
    type Commands = Device;
}

/// Device object
pub trait ExtDiscardRectanglesDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::ext::discard_rectangles {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::discard_rectangles> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::discard_rectangles {
        type Output = crate::hnd::Device<vk::extensions::ext::discard_rectangles>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::discard_rectangles>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::discard_rectangles> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::discard_rectangles> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::discard_rectangles> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::discard_rectangles> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtDiscardRectanglesDevice for crate::hnd::Device<vk::extensions::ext::discard_rectangles> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::discard_rectangles, vk::Device> for crate::hnd::Device<vk::extensions::ext::discard_rectangles> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait ExtDiscardRectanglesCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdSetDiscardRectangleEXT(VkCommandBuffer commandBuffer, uint32_t firstDiscardRectangle, uint32_t discardRectangleCount, VkRect2D const* pDiscardRectangles)
    /// ```
    unsafe fn set_discard_rectangle(
        &self,
        first_discard_rectangle: uint32_t,
        discard_rectangles: &[Rect2D],
    ) -> () {
        unsafe {
            self.commands().cmd_set_discard_rectangle(
                self.raw(),
                first_discard_rectangle,
                discard_rectangles,
            )
        }
    }
    /// ```c
    /// void vkCmdSetDiscardRectangleEnableEXT(VkCommandBuffer commandBuffer, VkBool32 discardRectangleEnable)
    /// ```
    unsafe fn set_discard_rectangle_enable(
        &self,
        discard_rectangle_enable: bool,
    ) -> () {
        unsafe {
            self.commands().cmd_set_discard_rectangle_enable(
                self.raw(),
                discard_rectangle_enable,
            )
        }
    }
    /// ```c
    /// void vkCmdSetDiscardRectangleModeEXT(VkCommandBuffer commandBuffer, VkDiscardRectangleModeEXT discardRectangleMode)
    /// ```
    unsafe fn set_discard_rectangle_mode(
        &self,
        discard_rectangle_mode: DiscardRectangleModeEXT,
    ) -> () {
        unsafe {
            self.commands().cmd_set_discard_rectangle_mode(
                self.raw(),
                discard_rectangle_mode,
            )
        }
    }
}
