// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_tooling_info` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::ext::tooling_info::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::tooling_info::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkGetPhysicalDeviceToolProperties(VkPhysicalDevice physicalDevice, uint32_t* pToolCount, VkPhysicalDeviceToolProperties* pToolProperties)
    /// ```
    pub unsafe fn get_physical_device_tool_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        tool_properties: Option<&mut ::alloc::vec::Vec<PhysicalDeviceToolProperties>>,
    ) -> crate::Result<uint32_t> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetPhysicalDeviceToolPropertiesEXT(
                physical_device.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = tool_properties {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetPhysicalDeviceToolPropertiesEXT(
                    physical_device.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                ).vk();
                _r.result(|| Some(()))?;
                _b.set_len(_b.len() + _c as usize);
            }
            _r.result(|| Some(_c))
        }
    }
}

impl crate::CommandScope<vk::Instance> for vk::extensions::ext::tooling_info {
    type Commands = Instance;
}

/// Instance object
pub trait ExtToolingInfoInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::ext::tooling_info {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::ext::tooling_info> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::ext::tooling_info {
        type Output = crate::hnd::Instance<vk::extensions::ext::tooling_info>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::ext::tooling_info>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::ext::tooling_info> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::ext::tooling_info> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::ext::tooling_info> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::ext::tooling_info> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtToolingInfoInstance for crate::hnd::Instance<vk::extensions::ext::tooling_info> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::tooling_info, vk::Instance> for crate::hnd::Instance<vk::extensions::ext::tooling_info> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait ExtToolingInfoPhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkGetPhysicalDeviceToolProperties(VkPhysicalDevice physicalDevice, uint32_t* pToolCount, VkPhysicalDeviceToolProperties* pToolProperties)
    /// ```
    unsafe fn get_tool_properties(
        &self,
        tool_properties: Option<&mut ::alloc::vec::Vec<PhysicalDeviceToolProperties>>,
    ) -> crate::Result<uint32_t> {
        unsafe {
            self.commands().get_physical_device_tool_properties(
                self.raw(),
                tool_properties,
            )
        }
    }
}
