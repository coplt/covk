// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_copy_commands2` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::copy_commands2::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::copy_commands2::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdBlitImage2(VkCommandBuffer commandBuffer, VkBlitImageInfo2 const* pBlitImageInfo)
    /// ```
    pub unsafe fn cmd_blit_image2(
        &self,
        command_buffer: vk::CommandBuffer,
        blit_image_info: &BlitImageInfo2,
    ) -> () {
        unsafe {
            self.0.CmdBlitImage2KHR(
                command_buffer.abi(), 
                blit_image_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdCopyBuffer2(VkCommandBuffer commandBuffer, VkCopyBufferInfo2 const* pCopyBufferInfo)
    /// ```
    pub unsafe fn cmd_copy_buffer2(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_buffer_info: &CopyBufferInfo2,
    ) -> () {
        unsafe {
            self.0.CmdCopyBuffer2KHR(
                command_buffer.abi(), 
                copy_buffer_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdCopyBufferToImage2(VkCommandBuffer commandBuffer, VkCopyBufferToImageInfo2 const* pCopyBufferToImageInfo)
    /// ```
    pub unsafe fn cmd_copy_buffer_to_image2(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_buffer_to_image_info: &CopyBufferToImageInfo2,
    ) -> () {
        unsafe {
            self.0.CmdCopyBufferToImage2KHR(
                command_buffer.abi(), 
                copy_buffer_to_image_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdCopyImage2(VkCommandBuffer commandBuffer, VkCopyImageInfo2 const* pCopyImageInfo)
    /// ```
    pub unsafe fn cmd_copy_image2(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_image_info: &CopyImageInfo2,
    ) -> () {
        unsafe {
            self.0.CmdCopyImage2KHR(
                command_buffer.abi(), 
                copy_image_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdCopyImageToBuffer2(VkCommandBuffer commandBuffer, VkCopyImageToBufferInfo2 const* pCopyImageToBufferInfo)
    /// ```
    pub unsafe fn cmd_copy_image_to_buffer2(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_image_to_buffer_info: &CopyImageToBufferInfo2,
    ) -> () {
        unsafe {
            self.0.CmdCopyImageToBuffer2KHR(
                command_buffer.abi(), 
                copy_image_to_buffer_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdResolveImage2(VkCommandBuffer commandBuffer, VkResolveImageInfo2 const* pResolveImageInfo)
    /// ```
    pub unsafe fn cmd_resolve_image2(
        &self,
        command_buffer: vk::CommandBuffer,
        resolve_image_info: &ResolveImageInfo2,
    ) -> () {
        unsafe {
            self.0.CmdResolveImage2KHR(
                command_buffer.abi(), 
                resolve_image_info.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::copy_commands2 {
    type Commands = Device;
}

/// Device object
pub trait KhrCopyCommands2Device {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::khr::copy_commands2 {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::copy_commands2> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::copy_commands2 {
        type Output = crate::hnd::Device<vk::extensions::khr::copy_commands2>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::copy_commands2>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::copy_commands2> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::copy_commands2> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::copy_commands2> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::copy_commands2> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrCopyCommands2Device for crate::hnd::Device<vk::extensions::khr::copy_commands2> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::copy_commands2, vk::Device> for crate::hnd::Device<vk::extensions::khr::copy_commands2> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait KhrCopyCommands2CommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdBlitImage2(VkCommandBuffer commandBuffer, VkBlitImageInfo2 const* pBlitImageInfo)
    /// ```
    unsafe fn blit_image2(
        &self,
        blit_image_info: &BlitImageInfo2,
    ) -> () {
        unsafe {
            self.commands().cmd_blit_image2(
                self.raw(),
                blit_image_info,
            )
        }
    }
    /// ```c
    /// void vkCmdCopyBuffer2(VkCommandBuffer commandBuffer, VkCopyBufferInfo2 const* pCopyBufferInfo)
    /// ```
    unsafe fn copy_buffer2(
        &self,
        copy_buffer_info: &CopyBufferInfo2,
    ) -> () {
        unsafe {
            self.commands().cmd_copy_buffer2(
                self.raw(),
                copy_buffer_info,
            )
        }
    }
    /// ```c
    /// void vkCmdCopyBufferToImage2(VkCommandBuffer commandBuffer, VkCopyBufferToImageInfo2 const* pCopyBufferToImageInfo)
    /// ```
    unsafe fn copy_buffer_to_image2(
        &self,
        copy_buffer_to_image_info: &CopyBufferToImageInfo2,
    ) -> () {
        unsafe {
            self.commands().cmd_copy_buffer_to_image2(
                self.raw(),
                copy_buffer_to_image_info,
            )
        }
    }
    /// ```c
    /// void vkCmdCopyImage2(VkCommandBuffer commandBuffer, VkCopyImageInfo2 const* pCopyImageInfo)
    /// ```
    unsafe fn copy_image2(
        &self,
        copy_image_info: &CopyImageInfo2,
    ) -> () {
        unsafe {
            self.commands().cmd_copy_image2(
                self.raw(),
                copy_image_info,
            )
        }
    }
    /// ```c
    /// void vkCmdCopyImageToBuffer2(VkCommandBuffer commandBuffer, VkCopyImageToBufferInfo2 const* pCopyImageToBufferInfo)
    /// ```
    unsafe fn copy_image_to_buffer2(
        &self,
        copy_image_to_buffer_info: &CopyImageToBufferInfo2,
    ) -> () {
        unsafe {
            self.commands().cmd_copy_image_to_buffer2(
                self.raw(),
                copy_image_to_buffer_info,
            )
        }
    }
    /// ```c
    /// void vkCmdResolveImage2(VkCommandBuffer commandBuffer, VkResolveImageInfo2 const* pResolveImageInfo)
    /// ```
    unsafe fn resolve_image2(
        &self,
        resolve_image_info: &ResolveImageInfo2,
    ) -> () {
        unsafe {
            self.commands().cmd_resolve_image2(
                self.raw(),
                resolve_image_info,
            )
        }
    }
}
