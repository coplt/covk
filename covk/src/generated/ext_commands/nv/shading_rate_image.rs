// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_NV_shading_rate_image` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::nv::shading_rate_image::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::shading_rate_image::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdBindShadingRateImageNV(VkCommandBuffer commandBuffer, VkImageView imageView, VkImageLayout imageLayout)
    /// ```
    pub unsafe fn cmd_bind_shading_rate_image(
        &self,
        command_buffer: vk::CommandBuffer,
        image_view: Option<vk::ImageView>,
        image_layout: ImageLayout,
    ) -> () {
        unsafe {
            self.0.CmdBindShadingRateImageNV(
                command_buffer.abi(), 
                image_view.abi(), 
                image_layout.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetCoarseSampleOrderNV(VkCommandBuffer commandBuffer, VkCoarseSampleOrderTypeNV sampleOrderType, uint32_t customSampleOrderCount, VkCoarseSampleOrderCustomNV const* pCustomSampleOrders)
    /// ```
    pub unsafe fn cmd_set_coarse_sample_order(
        &self,
        command_buffer: vk::CommandBuffer,
        sample_order_type: CoarseSampleOrderTypeNV,
        custom_sample_orders: &[CoarseSampleOrderCustomNV],
    ) -> () {
        unsafe {
            self.0.CmdSetCoarseSampleOrderNV(
                command_buffer.abi(), 
                sample_order_type.abi(), 
                custom_sample_orders.len() as _, 
                custom_sample_orders.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetViewportShadingRatePaletteNV(VkCommandBuffer commandBuffer, uint32_t firstViewport, uint32_t viewportCount, VkShadingRatePaletteNV const* pShadingRatePalettes)
    /// ```
    pub unsafe fn cmd_set_viewport_shading_rate_palette(
        &self,
        command_buffer: vk::CommandBuffer,
        first_viewport: uint32_t,
        shading_rate_palettes: &[ShadingRatePaletteNV],
    ) -> () {
        unsafe {
            self.0.CmdSetViewportShadingRatePaletteNV(
                command_buffer.abi(), 
                first_viewport.abi(), 
                shading_rate_palettes.len() as _, 
                shading_rate_palettes.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::nv::shading_rate_image {
    type Commands = Device;
}

/// Device object
pub trait NvShadingRateImageDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

}

impl crate::HndScope<vk::Device> for vk::extensions::nv::shading_rate_image {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::nv::shading_rate_image> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::nv::shading_rate_image {
        type Output = crate::hnd::Device<vk::extensions::nv::shading_rate_image>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::nv::shading_rate_image>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::nv::shading_rate_image> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::nv::shading_rate_image> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::nv::shading_rate_image> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::nv::shading_rate_image> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvShadingRateImageDevice for crate::hnd::Device<vk::extensions::nv::shading_rate_image> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::shading_rate_image, vk::Device> for crate::hnd::Device<vk::extensions::nv::shading_rate_image> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait NvShadingRateImageCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdBindShadingRateImageNV(VkCommandBuffer commandBuffer, VkImageView imageView, VkImageLayout imageLayout)
    /// ```
    unsafe fn bind_shading_rate_image(
        &self,
        image_view: Option<vk::ImageView>,
        image_layout: ImageLayout,
    ) -> () {
        unsafe {
            self.commands().cmd_bind_shading_rate_image(
                self.raw(),
                image_view,
                image_layout,
            )
        }
    }
    /// ```c
    /// void vkCmdSetCoarseSampleOrderNV(VkCommandBuffer commandBuffer, VkCoarseSampleOrderTypeNV sampleOrderType, uint32_t customSampleOrderCount, VkCoarseSampleOrderCustomNV const* pCustomSampleOrders)
    /// ```
    unsafe fn set_coarse_sample_order(
        &self,
        sample_order_type: CoarseSampleOrderTypeNV,
        custom_sample_orders: &[CoarseSampleOrderCustomNV],
    ) -> () {
        unsafe {
            self.commands().cmd_set_coarse_sample_order(
                self.raw(),
                sample_order_type,
                custom_sample_orders,
            )
        }
    }
    /// ```c
    /// void vkCmdSetViewportShadingRatePaletteNV(VkCommandBuffer commandBuffer, uint32_t firstViewport, uint32_t viewportCount, VkShadingRatePaletteNV const* pShadingRatePalettes)
    /// ```
    unsafe fn set_viewport_shading_rate_palette(
        &self,
        first_viewport: uint32_t,
        shading_rate_palettes: &[ShadingRatePaletteNV],
    ) -> () {
        unsafe {
            self.commands().cmd_set_viewport_shading_rate_palette(
                self.raw(),
                first_viewport,
                shading_rate_palettes,
            )
        }
    }
}
