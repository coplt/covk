// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_QCOM_tile_properties` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::qcom::tile_properties::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::qcom::tile_properties::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkGetDynamicRenderingTilePropertiesQCOM(VkDevice device, VkRenderingInfo const* pRenderingInfo, VkTilePropertiesQCOM* pProperties)
    /// ```
    pub unsafe fn get_dynamic_rendering_tile_properties(
        &self,
        device: vk::Device,
        rendering_info: &RenderingInfo,
        properties: &mut TilePropertiesQCOM,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetDynamicRenderingTilePropertiesQCOM(
                device.abi(), 
                rendering_info.abi(), 
                properties.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkGetFramebufferTilePropertiesQCOM(VkDevice device, VkFramebuffer framebuffer, uint32_t* pPropertiesCount, VkTilePropertiesQCOM* pProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_framebuffer_tile_properties(
        &self,
        device: vk::Device,
        framebuffer: vk::Framebuffer,
        properties: Option<&mut ::alloc::vec::Vec<TilePropertiesQCOM>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetFramebufferTilePropertiesQCOM(
                device.abi(), 
                framebuffer.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = properties {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetFramebufferTilePropertiesQCOM(
                    device.abi(), 
                    framebuffer.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                ).vk();
                _r.result(|| Some(()))?;
                _b.set_len(_b.len() + _c as usize);
            }
            _r.result_multi(|| Some(_c))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::qcom::tile_properties {
    type Commands = Device;
}

/// Device object
pub trait QcomTilePropertiesDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetDynamicRenderingTilePropertiesQCOM(VkDevice device, VkRenderingInfo const* pRenderingInfo, VkTilePropertiesQCOM* pProperties)
    /// ```
    unsafe fn get_dynamic_rendering_tile_properties(
        &self,
        rendering_info: &RenderingInfo,
        properties: &mut TilePropertiesQCOM,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_dynamic_rendering_tile_properties(
                self.raw(),
                rendering_info,
                properties,
            )
        }
    }
    /// ```c
    /// VkResult vkGetFramebufferTilePropertiesQCOM(VkDevice device, VkFramebuffer framebuffer, uint32_t* pPropertiesCount, VkTilePropertiesQCOM* pProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_framebuffer_tile_properties(
        &self,
        framebuffer: vk::Framebuffer,
        properties: Option<&mut ::alloc::vec::Vec<TilePropertiesQCOM>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_framebuffer_tile_properties(
                self.raw(),
                framebuffer,
                properties,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::qcom::tile_properties {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::qcom::tile_properties> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::qcom::tile_properties {
        type Output = crate::hnd::Device<vk::extensions::qcom::tile_properties>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::qcom::tile_properties>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::qcom::tile_properties> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::qcom::tile_properties> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::qcom::tile_properties> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::qcom::tile_properties> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::QcomTilePropertiesDevice for crate::hnd::Device<vk::extensions::qcom::tile_properties> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::qcom::tile_properties, vk::Device> for crate::hnd::Device<vk::extensions::qcom::tile_properties> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
