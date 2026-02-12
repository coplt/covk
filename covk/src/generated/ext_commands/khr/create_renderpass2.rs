// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_create_renderpass2` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::create_renderpass2::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::create_renderpass2::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdBeginRenderPass2(VkCommandBuffer commandBuffer, VkRenderPassBeginInfo const* pRenderPassBegin, VkSubpassBeginInfo const* pSubpassBeginInfo)
    /// ```
    pub unsafe fn cmd_begin_render_pass2(
        &self,
        command_buffer: vk::CommandBuffer,
        render_pass_begin: &RenderPassBeginInfo,
        subpass_begin_info: &SubpassBeginInfo,
    ) -> () {
        unsafe {
            self.0.CmdBeginRenderPass2KHR(
                command_buffer.abi(), 
                render_pass_begin.abi(), 
                subpass_begin_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdEndRenderPass2(VkCommandBuffer commandBuffer, VkSubpassEndInfo const* pSubpassEndInfo)
    /// ```
    pub unsafe fn cmd_end_render_pass2(
        &self,
        command_buffer: vk::CommandBuffer,
        subpass_end_info: &SubpassEndInfo,
    ) -> () {
        unsafe {
            self.0.CmdEndRenderPass2KHR(
                command_buffer.abi(), 
                subpass_end_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdNextSubpass2(VkCommandBuffer commandBuffer, VkSubpassBeginInfo const* pSubpassBeginInfo, VkSubpassEndInfo const* pSubpassEndInfo)
    /// ```
    pub unsafe fn cmd_next_subpass2(
        &self,
        command_buffer: vk::CommandBuffer,
        subpass_begin_info: &SubpassBeginInfo,
        subpass_end_info: &SubpassEndInfo,
    ) -> () {
        unsafe {
            self.0.CmdNextSubpass2KHR(
                command_buffer.abi(), 
                subpass_begin_info.abi(), 
                subpass_end_info.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkCreateRenderPass2(VkDevice device, VkRenderPassCreateInfo2 const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkRenderPass* pRenderPass)
    /// ```
    pub unsafe fn create_render_pass2(
        &self,
        device: vk::Device,
        create_info: &RenderPassCreateInfo2,
    ) -> crate::Result<vk::RenderPass> {
        unsafe {
            let mut _v: Option<vk::RenderPass> = Default::default();
            let _r = self.0.CreateRenderPass2KHR(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::create_renderpass2 {
    type Commands = Device;
}

/// Device object
pub trait KhrCreateRenderpass2Device {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkCreateRenderPass2(VkDevice device, VkRenderPassCreateInfo2 const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkRenderPass* pRenderPass)
    /// ```
    unsafe fn create_render_pass2(
        &self,
        create_info: &RenderPassCreateInfo2,
    ) -> crate::Result<vk::RenderPass> {
        unsafe {
            self.commands().create_render_pass2(
                self.raw(),
                create_info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::create_renderpass2 {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::create_renderpass2> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::create_renderpass2 {
        type Output = crate::hnd::Device<vk::extensions::khr::create_renderpass2>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::create_renderpass2>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::create_renderpass2> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::create_renderpass2> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::create_renderpass2> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::create_renderpass2> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrCreateRenderpass2Device for crate::hnd::Device<vk::extensions::khr::create_renderpass2> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::create_renderpass2, vk::Device> for crate::hnd::Device<vk::extensions::khr::create_renderpass2> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait KhrCreateRenderpass2CommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdBeginRenderPass2(VkCommandBuffer commandBuffer, VkRenderPassBeginInfo const* pRenderPassBegin, VkSubpassBeginInfo const* pSubpassBeginInfo)
    /// ```
    unsafe fn begin_render_pass2(
        &self,
        render_pass_begin: &RenderPassBeginInfo,
        subpass_begin_info: &SubpassBeginInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_begin_render_pass2(
                self.raw(),
                render_pass_begin,
                subpass_begin_info,
            )
        }
    }
    /// ```c
    /// void vkCmdEndRenderPass2(VkCommandBuffer commandBuffer, VkSubpassEndInfo const* pSubpassEndInfo)
    /// ```
    unsafe fn end_render_pass2(
        &self,
        subpass_end_info: &SubpassEndInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_end_render_pass2(
                self.raw(),
                subpass_end_info,
            )
        }
    }
    /// ```c
    /// void vkCmdNextSubpass2(VkCommandBuffer commandBuffer, VkSubpassBeginInfo const* pSubpassBeginInfo, VkSubpassEndInfo const* pSubpassEndInfo)
    /// ```
    unsafe fn next_subpass2(
        &self,
        subpass_begin_info: &SubpassBeginInfo,
        subpass_end_info: &SubpassEndInfo,
    ) -> () {
        unsafe {
            self.commands().cmd_next_subpass2(
                self.raw(),
                subpass_begin_info,
                subpass_end_info,
            )
        }
    }
}
