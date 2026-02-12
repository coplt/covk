// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_HUAWEI_subpass_shading` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::huawei::subpass_shading::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::huawei::subpass_shading::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdSubpassShadingHUAWEI(VkCommandBuffer commandBuffer)
    /// ```
    pub unsafe fn cmd_subpass_shading(
        &self,
        command_buffer: vk::CommandBuffer,
    ) -> () {
        unsafe {
            self.0.CmdSubpassShadingHUAWEI(
                command_buffer.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI(VkDevice device, VkRenderPass renderpass, VkExtent2D* pMaxWorkgroupSize)
    /// ```
    pub unsafe fn get_device_subpass_shading_max_workgroup_size(
        &self,
        device: vk::Device,
        renderpass: vk::RenderPass,
        max_workgroup_size: &mut Extent2D,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI(
                device.abi(), 
                renderpass.abi(), 
                max_workgroup_size.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::huawei::subpass_shading {
    type Commands = Device;
}

/// Device object
pub trait HuaweiSubpassShadingDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI(VkDevice device, VkRenderPass renderpass, VkExtent2D* pMaxWorkgroupSize)
    /// ```
    unsafe fn get_device_subpass_shading_max_workgroup_size(
        &self,
        renderpass: vk::RenderPass,
        max_workgroup_size: &mut Extent2D,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_device_subpass_shading_max_workgroup_size(
                self.raw(),
                renderpass,
                max_workgroup_size,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::huawei::subpass_shading {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::huawei::subpass_shading> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::huawei::subpass_shading {
        type Output = crate::hnd::Device<vk::extensions::huawei::subpass_shading>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::huawei::subpass_shading>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::huawei::subpass_shading> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::huawei::subpass_shading> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::huawei::subpass_shading> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::huawei::subpass_shading> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::HuaweiSubpassShadingDevice for crate::hnd::Device<vk::extensions::huawei::subpass_shading> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::huawei::subpass_shading, vk::Device> for crate::hnd::Device<vk::extensions::huawei::subpass_shading> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait HuaweiSubpassShadingCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdSubpassShadingHUAWEI(VkCommandBuffer commandBuffer)
    /// ```
    unsafe fn subpass_shading(
        &self,
    ) -> () {
        unsafe {
            self.commands().cmd_subpass_shading(
                self.raw(),
            )
        }
    }
}
