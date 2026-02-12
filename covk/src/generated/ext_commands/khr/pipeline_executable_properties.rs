// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_pipeline_executable_properties` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::pipeline_executable_properties::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::pipeline_executable_properties::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkGetPipelineExecutableInternalRepresentationsKHR(VkDevice device, VkPipelineExecutableInfoKHR const* pExecutableInfo, uint32_t* pInternalRepresentationCount, VkPipelineExecutableInternalRepresentationKHR* pInternalRepresentations)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_pipeline_executable_internal_representations(
        &self,
        device: vk::Device,
        executable_info: &PipelineExecutableInfoKHR,
        internal_representations: Option<&mut ::alloc::vec::Vec<PipelineExecutableInternalRepresentationKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetPipelineExecutableInternalRepresentationsKHR(
                device.abi(), 
                executable_info.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = internal_representations {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetPipelineExecutableInternalRepresentationsKHR(
                    device.abi(), 
                    executable_info.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                ).vk();
                _r.result(|| Some(()))?;
                _b.set_len(_b.len() + _c as usize);
            }
            _r.result_multi(|| Some(_c))
        }
    }
    /// ```c
    /// VkResult vkGetPipelineExecutablePropertiesKHR(VkDevice device, VkPipelineInfoKHR const* pPipelineInfo, uint32_t* pExecutableCount, VkPipelineExecutablePropertiesKHR* pProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_pipeline_executable_properties(
        &self,
        device: vk::Device,
        pipeline_info: &PipelineInfoKHR,
        properties: Option<&mut ::alloc::vec::Vec<PipelineExecutablePropertiesKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetPipelineExecutablePropertiesKHR(
                device.abi(), 
                pipeline_info.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = properties {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetPipelineExecutablePropertiesKHR(
                    device.abi(), 
                    pipeline_info.abi(), 
                    &mut _c, 
                    (&mut **_b).abi(), 
                ).vk();
                _r.result(|| Some(()))?;
                _b.set_len(_b.len() + _c as usize);
            }
            _r.result_multi(|| Some(_c))
        }
    }
    /// ```c
    /// VkResult vkGetPipelineExecutableStatisticsKHR(VkDevice device, VkPipelineExecutableInfoKHR const* pExecutableInfo, uint32_t* pStatisticCount, VkPipelineExecutableStatisticKHR* pStatistics)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_pipeline_executable_statistics(
        &self,
        device: vk::Device,
        executable_info: &PipelineExecutableInfoKHR,
        statistics: Option<&mut ::alloc::vec::Vec<PipelineExecutableStatisticKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetPipelineExecutableStatisticsKHR(
                device.abi(), 
                executable_info.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = statistics {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetPipelineExecutableStatisticsKHR(
                    device.abi(), 
                    executable_info.abi(), 
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

impl crate::CommandScope<vk::Device> for vk::extensions::khr::pipeline_executable_properties {
    type Commands = Device;
}

/// Device object
pub trait KhrPipelineExecutablePropertiesDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkGetPipelineExecutableInternalRepresentationsKHR(VkDevice device, VkPipelineExecutableInfoKHR const* pExecutableInfo, uint32_t* pInternalRepresentationCount, VkPipelineExecutableInternalRepresentationKHR* pInternalRepresentations)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_pipeline_executable_internal_representations(
        &self,
        executable_info: &PipelineExecutableInfoKHR,
        internal_representations: Option<&mut ::alloc::vec::Vec<PipelineExecutableInternalRepresentationKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_pipeline_executable_internal_representations(
                self.raw(),
                executable_info,
                internal_representations,
            )
        }
    }
    /// ```c
    /// VkResult vkGetPipelineExecutablePropertiesKHR(VkDevice device, VkPipelineInfoKHR const* pPipelineInfo, uint32_t* pExecutableCount, VkPipelineExecutablePropertiesKHR* pProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_pipeline_executable_properties(
        &self,
        pipeline_info: &PipelineInfoKHR,
        properties: Option<&mut ::alloc::vec::Vec<PipelineExecutablePropertiesKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_pipeline_executable_properties(
                self.raw(),
                pipeline_info,
                properties,
            )
        }
    }
    /// ```c
    /// VkResult vkGetPipelineExecutableStatisticsKHR(VkDevice device, VkPipelineExecutableInfoKHR const* pExecutableInfo, uint32_t* pStatisticCount, VkPipelineExecutableStatisticKHR* pStatistics)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_pipeline_executable_statistics(
        &self,
        executable_info: &PipelineExecutableInfoKHR,
        statistics: Option<&mut ::alloc::vec::Vec<PipelineExecutableStatisticKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_pipeline_executable_statistics(
                self.raw(),
                executable_info,
                statistics,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::pipeline_executable_properties {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::pipeline_executable_properties> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::pipeline_executable_properties {
        type Output = crate::hnd::Device<vk::extensions::khr::pipeline_executable_properties>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::pipeline_executable_properties>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::pipeline_executable_properties> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::pipeline_executable_properties> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::pipeline_executable_properties> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::pipeline_executable_properties> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrPipelineExecutablePropertiesDevice for crate::hnd::Device<vk::extensions::khr::pipeline_executable_properties> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::pipeline_executable_properties, vk::Device> for crate::hnd::Device<vk::extensions::khr::pipeline_executable_properties> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}
