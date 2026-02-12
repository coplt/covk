// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_NV_cooperative_vector` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::nv::cooperative_vector::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::cooperative_vector::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkGetPhysicalDeviceCooperativeVectorPropertiesNV(VkPhysicalDevice physicalDevice, uint32_t* pPropertyCount, VkCooperativeVectorPropertiesNV* pProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_physical_device_cooperative_vector_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        properties: Option<&mut ::alloc::vec::Vec<CooperativeVectorPropertiesNV>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetPhysicalDeviceCooperativeVectorPropertiesNV(
                physical_device.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = properties {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetPhysicalDeviceCooperativeVectorPropertiesNV(
                    physical_device.abi(), 
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

impl crate::CommandScope<vk::Instance> for vk::extensions::nv::cooperative_vector {
    type Commands = Instance;
}

/// Instance object
pub trait NvCooperativeVectorInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::nv::cooperative_vector {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::nv::cooperative_vector> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::nv::cooperative_vector {
        type Output = crate::hnd::Instance<vk::extensions::nv::cooperative_vector>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::nv::cooperative_vector>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::nv::cooperative_vector> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::nv::cooperative_vector> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::nv::cooperative_vector> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::nv::cooperative_vector> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvCooperativeVectorInstance for crate::hnd::Instance<vk::extensions::nv::cooperative_vector> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::cooperative_vector, vk::Instance> for crate::hnd::Instance<vk::extensions::nv::cooperative_vector> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait NvCooperativeVectorPhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkGetPhysicalDeviceCooperativeVectorPropertiesNV(VkPhysicalDevice physicalDevice, uint32_t* pPropertyCount, VkCooperativeVectorPropertiesNV* pProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_cooperative_vector_properties(
        &self,
        properties: Option<&mut ::alloc::vec::Vec<CooperativeVectorPropertiesNV>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_physical_device_cooperative_vector_properties(
                self.raw(),
                properties,
            )
        }
    }
}

/// `VK_NV_cooperative_vector` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::nv::cooperative_vector::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::nv::cooperative_vector::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdConvertCooperativeVectorMatrixNV(VkCommandBuffer commandBuffer, uint32_t infoCount, VkConvertCooperativeVectorMatrixInfoNV const* pInfos)
    /// ```
    pub unsafe fn cmd_convert_cooperative_vector_matrix(
        &self,
        command_buffer: vk::CommandBuffer,
        infos: &[ConvertCooperativeVectorMatrixInfoNV],
    ) -> () {
        unsafe {
            self.0.CmdConvertCooperativeVectorMatrixNV(
                command_buffer.abi(), 
                infos.len() as _, 
                infos.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkConvertCooperativeVectorMatrixNV(VkDevice device, VkConvertCooperativeVectorMatrixInfoNV const* pInfo)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn convert_cooperative_vector_matrix(
        &self,
        device: vk::Device,
        info: &ConvertCooperativeVectorMatrixInfoNV,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.ConvertCooperativeVectorMatrixNV(
                device.abi(), 
                info.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::nv::cooperative_vector {
    type Commands = Device;
}

/// Device object
pub trait NvCooperativeVectorDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkConvertCooperativeVectorMatrixNV(VkDevice device, VkConvertCooperativeVectorMatrixInfoNV const* pInfo)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn convert_cooperative_vector_matrix(
        &self,
        info: &ConvertCooperativeVectorMatrixInfoNV,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().convert_cooperative_vector_matrix(
                self.raw(),
                info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::nv::cooperative_vector {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::nv::cooperative_vector> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::nv::cooperative_vector {
        type Output = crate::hnd::Device<vk::extensions::nv::cooperative_vector>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::nv::cooperative_vector>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::nv::cooperative_vector> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::nv::cooperative_vector> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::nv::cooperative_vector> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::nv::cooperative_vector> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::NvCooperativeVectorDevice for crate::hnd::Device<vk::extensions::nv::cooperative_vector> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::nv::cooperative_vector, vk::Device> for crate::hnd::Device<vk::extensions::nv::cooperative_vector> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait NvCooperativeVectorCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdConvertCooperativeVectorMatrixNV(VkCommandBuffer commandBuffer, uint32_t infoCount, VkConvertCooperativeVectorMatrixInfoNV const* pInfos)
    /// ```
    unsafe fn convert_cooperative_vector_matrix(
        &self,
        infos: &[ConvertCooperativeVectorMatrixInfoNV],
    ) -> () {
        unsafe {
            self.commands().cmd_convert_cooperative_vector_matrix(
                self.raw(),
                infos,
            )
        }
    }
}
