// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_shader_module_identifier` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::shader_module_identifier::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::shader_module_identifier::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkGetShaderModuleCreateInfoIdentifierEXT(VkDevice device, VkShaderModuleCreateInfo const* pCreateInfo, VkShaderModuleIdentifierEXT* pIdentifier)
    /// ```
    pub unsafe fn get_shader_module_create_info_identifier(
        &self,
        device: vk::Device,
        create_info: &ShaderModuleCreateInfo,
        identifier: &mut ShaderModuleIdentifierEXT,
    ) -> () {
        unsafe {
            self.0.GetShaderModuleCreateInfoIdentifierEXT(
                device.abi(), 
                create_info.abi(), 
                identifier.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetShaderModuleIdentifierEXT(VkDevice device, VkShaderModule shaderModule, VkShaderModuleIdentifierEXT* pIdentifier)
    /// ```
    pub unsafe fn get_shader_module_identifier(
        &self,
        device: vk::Device,
        shader_module: vk::ShaderModule,
        identifier: &mut ShaderModuleIdentifierEXT,
    ) -> () {
        unsafe {
            self.0.GetShaderModuleIdentifierEXT(
                device.abi(), 
                shader_module.abi(), 
                identifier.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::shader_module_identifier {
    type Commands = Device;
}

/// Device object
pub trait ExtShaderModuleIdentifierDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkGetShaderModuleCreateInfoIdentifierEXT(VkDevice device, VkShaderModuleCreateInfo const* pCreateInfo, VkShaderModuleIdentifierEXT* pIdentifier)
    /// ```
    unsafe fn get_shader_module_create_info_identifier(
        &self,
        create_info: &ShaderModuleCreateInfo,
        identifier: &mut ShaderModuleIdentifierEXT,
    ) -> () {
        unsafe {
            self.commands().get_shader_module_create_info_identifier(
                self.raw(),
                create_info,
                identifier,
            )
        }
    }
    /// ```c
    /// void vkGetShaderModuleIdentifierEXT(VkDevice device, VkShaderModule shaderModule, VkShaderModuleIdentifierEXT* pIdentifier)
    /// ```
    unsafe fn get_shader_module_identifier(
        &self,
        shader_module: vk::ShaderModule,
        identifier: &mut ShaderModuleIdentifierEXT,
    ) -> () {
        unsafe {
            self.commands().get_shader_module_identifier(
                self.raw(),
                shader_module,
                identifier,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::ext::shader_module_identifier {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::shader_module_identifier> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::shader_module_identifier {
        type Output = crate::hnd::Device<vk::extensions::ext::shader_module_identifier>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::shader_module_identifier>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::shader_module_identifier> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::shader_module_identifier> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::shader_module_identifier> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::shader_module_identifier> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtShaderModuleIdentifierDevice for crate::hnd::Device<vk::extensions::ext::shader_module_identifier> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::shader_module_identifier, vk::Device> for crate::hnd::Device<vk::extensions::ext::shader_module_identifier> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
