// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_INTEL_performance_query` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::intel::performance_query::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::intel::performance_query::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkAcquirePerformanceConfigurationINTEL(VkDevice device, VkPerformanceConfigurationAcquireInfoINTEL const* pAcquireInfo, VkPerformanceConfigurationINTEL* pConfiguration)
    /// ```
    pub unsafe fn acquire_performance_configuration(
        &self,
        device: vk::Device,
        acquire_info: &PerformanceConfigurationAcquireInfoINTEL,
    ) -> crate::Result<vk::PerformanceConfigurationINTEL> {
        unsafe {
            let mut _v: Option<vk::PerformanceConfigurationINTEL> = Default::default();
            let _r = self.0.AcquirePerformanceConfigurationINTEL(
                device.abi(), 
                acquire_info.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCmdSetPerformanceMarkerINTEL(VkCommandBuffer commandBuffer, VkPerformanceMarkerInfoINTEL const* pMarkerInfo)
    /// ```
    pub unsafe fn cmd_set_performance_marker(
        &self,
        command_buffer: vk::CommandBuffer,
        marker_info: &PerformanceMarkerInfoINTEL,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.CmdSetPerformanceMarkerINTEL(
                command_buffer.abi(), 
                marker_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkCmdSetPerformanceOverrideINTEL(VkCommandBuffer commandBuffer, VkPerformanceOverrideInfoINTEL const* pOverrideInfo)
    /// ```
    pub unsafe fn cmd_set_performance_override(
        &self,
        command_buffer: vk::CommandBuffer,
        override_info: &PerformanceOverrideInfoINTEL,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.CmdSetPerformanceOverrideINTEL(
                command_buffer.abi(), 
                override_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkCmdSetPerformanceStreamMarkerINTEL(VkCommandBuffer commandBuffer, VkPerformanceStreamMarkerInfoINTEL const* pMarkerInfo)
    /// ```
    pub unsafe fn cmd_set_performance_stream_marker(
        &self,
        command_buffer: vk::CommandBuffer,
        marker_info: &PerformanceStreamMarkerInfoINTEL,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.CmdSetPerformanceStreamMarkerINTEL(
                command_buffer.abi(), 
                marker_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkGetPerformanceParameterINTEL(VkDevice device, VkPerformanceParameterTypeINTEL parameter, VkPerformanceValueINTEL* pValue)
    /// ```
    pub unsafe fn get_performance_parameter(
        &self,
        device: vk::Device,
        parameter: PerformanceParameterTypeINTEL,
    ) -> crate::Result<PerformanceValueINTEL> {
        unsafe {
            let mut _v: PerformanceValueINTEL = Default::default();
            let _r = self.0.GetPerformanceParameterINTEL(
                device.abi(), 
                parameter.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
    /// ```c
    /// VkResult vkInitializePerformanceApiINTEL(VkDevice device, VkInitializePerformanceApiInfoINTEL const* pInitializeInfo)
    /// ```
    pub unsafe fn initialize_performance_api(
        &self,
        device: vk::Device,
        initialize_info: &InitializePerformanceApiInfoINTEL,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.InitializePerformanceApiINTEL(
                device.abi(), 
                initialize_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkQueueSetPerformanceConfigurationINTEL(VkQueue queue, VkPerformanceConfigurationINTEL configuration)
    /// ```
    pub unsafe fn queue_set_performance_configuration(
        &self,
        queue: vk::Queue,
        configuration: vk::PerformanceConfigurationINTEL,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.QueueSetPerformanceConfigurationINTEL(
                queue.abi(), 
                configuration.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkReleasePerformanceConfigurationINTEL(VkDevice device, VkPerformanceConfigurationINTEL configuration)
    /// ```
    pub unsafe fn release_performance_configuration(
        &self,
        device: vk::Device,
        configuration: Option<vk::PerformanceConfigurationINTEL>,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.ReleasePerformanceConfigurationINTEL(
                device.abi(), 
                configuration.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// void vkUninitializePerformanceApiINTEL(VkDevice device)
    /// ```
    pub unsafe fn uninitialize_performance_api(
        &self,
        device: vk::Device,
    ) -> () {
        unsafe {
            self.0.UninitializePerformanceApiINTEL(
                device.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::intel::performance_query {
    type Commands = Device;
}

/// Device object
pub trait IntelPerformanceQueryDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkAcquirePerformanceConfigurationINTEL(VkDevice device, VkPerformanceConfigurationAcquireInfoINTEL const* pAcquireInfo, VkPerformanceConfigurationINTEL* pConfiguration)
    /// ```
    unsafe fn acquire_performance_configuration(
        &self,
        acquire_info: &PerformanceConfigurationAcquireInfoINTEL,
    ) -> crate::Result<vk::PerformanceConfigurationINTEL> {
        unsafe {
            self.commands().acquire_performance_configuration(
                self.raw(),
                acquire_info,
            )
        }
    }
    /// ```c
    /// VkResult vkGetPerformanceParameterINTEL(VkDevice device, VkPerformanceParameterTypeINTEL parameter, VkPerformanceValueINTEL* pValue)
    /// ```
    unsafe fn get_performance_parameter(
        &self,
        parameter: PerformanceParameterTypeINTEL,
    ) -> crate::Result<PerformanceValueINTEL> {
        unsafe {
            self.commands().get_performance_parameter(
                self.raw(),
                parameter,
            )
        }
    }
    /// ```c
    /// VkResult vkInitializePerformanceApiINTEL(VkDevice device, VkInitializePerformanceApiInfoINTEL const* pInitializeInfo)
    /// ```
    unsafe fn initialize_performance_api(
        &self,
        initialize_info: &InitializePerformanceApiInfoINTEL,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().initialize_performance_api(
                self.raw(),
                initialize_info,
            )
        }
    }
    /// ```c
    /// VkResult vkReleasePerformanceConfigurationINTEL(VkDevice device, VkPerformanceConfigurationINTEL configuration)
    /// ```
    unsafe fn release_performance_configuration(
        &self,
        configuration: Option<vk::PerformanceConfigurationINTEL>,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().release_performance_configuration(
                self.raw(),
                configuration,
            )
        }
    }
    /// ```c
    /// void vkUninitializePerformanceApiINTEL(VkDevice device)
    /// ```
    unsafe fn uninitialize_performance_api(
        &self,
    ) -> () {
        unsafe {
            self.commands().uninitialize_performance_api(
                self.raw(),
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::intel::performance_query {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::intel::performance_query> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::intel::performance_query {
        type Output = crate::hnd::Device<vk::extensions::intel::performance_query>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::intel::performance_query>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::intel::performance_query> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::intel::performance_query> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::intel::performance_query> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::intel::performance_query> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::IntelPerformanceQueryDevice for crate::hnd::Device<vk::extensions::intel::performance_query> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::intel::performance_query, vk::Device> for crate::hnd::Device<vk::extensions::intel::performance_query> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait IntelPerformanceQueryCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkCmdSetPerformanceMarkerINTEL(VkCommandBuffer commandBuffer, VkPerformanceMarkerInfoINTEL const* pMarkerInfo)
    /// ```
    unsafe fn set_performance_marker(
        &self,
        marker_info: &PerformanceMarkerInfoINTEL,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().cmd_set_performance_marker(
                self.raw(),
                marker_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCmdSetPerformanceOverrideINTEL(VkCommandBuffer commandBuffer, VkPerformanceOverrideInfoINTEL const* pOverrideInfo)
    /// ```
    unsafe fn set_performance_override(
        &self,
        override_info: &PerformanceOverrideInfoINTEL,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().cmd_set_performance_override(
                self.raw(),
                override_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCmdSetPerformanceStreamMarkerINTEL(VkCommandBuffer commandBuffer, VkPerformanceStreamMarkerInfoINTEL const* pMarkerInfo)
    /// ```
    unsafe fn set_performance_stream_marker(
        &self,
        marker_info: &PerformanceStreamMarkerInfoINTEL,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().cmd_set_performance_stream_marker(
                self.raw(),
                marker_info,
            )
        }
    }
}

/// Queue object
pub trait IntelPerformanceQueryQueue {
    fn raw(&self) -> vk::Queue;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkQueueSetPerformanceConfigurationINTEL(VkQueue queue, VkPerformanceConfigurationINTEL configuration)
    /// ```
    unsafe fn set_performance_configuration(
        &self,
        configuration: vk::PerformanceConfigurationINTEL,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().queue_set_performance_configuration(
                self.raw(),
                configuration,
            )
        }
    }
}
