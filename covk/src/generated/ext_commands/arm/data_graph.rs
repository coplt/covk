// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_ARM_data_graph` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::arm::data_graph::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::arm::data_graph::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// void vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM(VkPhysicalDevice physicalDevice, VkPhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM const* pQueueFamilyDataGraphProcessingEngineInfo, VkQueueFamilyDataGraphProcessingEnginePropertiesARM* pQueueFamilyDataGraphProcessingEngineProperties)
    /// ```
    pub unsafe fn get_physical_device_queue_family_data_graph_processing_engine_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_data_graph_processing_engine_info: &PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM,
        queue_family_data_graph_processing_engine_properties: &mut QueueFamilyDataGraphProcessingEnginePropertiesARM,
    ) -> () {
        unsafe {
            self.0.GetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM(
                physical_device.abi(), 
                queue_family_data_graph_processing_engine_info.abi(), 
                queue_family_data_graph_processing_engine_properties.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, uint32_t* pQueueFamilyDataGraphPropertyCount, VkQueueFamilyDataGraphPropertiesARM* pQueueFamilyDataGraphProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_physical_device_queue_family_data_graph_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: uint32_t,
        queue_family_data_graph_properties: Option<&mut ::alloc::vec::Vec<QueueFamilyDataGraphPropertiesARM>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetPhysicalDeviceQueueFamilyDataGraphPropertiesARM(
                physical_device.abi(), 
                queue_family_index.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = queue_family_data_graph_properties {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetPhysicalDeviceQueueFamilyDataGraphPropertiesARM(
                    physical_device.abi(), 
                    queue_family_index.abi(), 
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

impl crate::CommandScope<vk::Instance> for vk::extensions::arm::data_graph {
    type Commands = Instance;
}

/// Instance object
pub trait ArmDataGraphInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::arm::data_graph {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::arm::data_graph> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::arm::data_graph {
        type Output = crate::hnd::Instance<vk::extensions::arm::data_graph>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::arm::data_graph>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::arm::data_graph> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::arm::data_graph> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::arm::data_graph> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::arm::data_graph> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ArmDataGraphInstance for crate::hnd::Instance<vk::extensions::arm::data_graph> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::arm::data_graph, vk::Instance> for crate::hnd::Instance<vk::extensions::arm::data_graph> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait ArmDataGraphPhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// void vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM(VkPhysicalDevice physicalDevice, VkPhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM const* pQueueFamilyDataGraphProcessingEngineInfo, VkQueueFamilyDataGraphProcessingEnginePropertiesARM* pQueueFamilyDataGraphProcessingEngineProperties)
    /// ```
    unsafe fn get_queue_family_data_graph_processing_engine_properties(
        &self,
        queue_family_data_graph_processing_engine_info: &PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM,
        queue_family_data_graph_processing_engine_properties: &mut QueueFamilyDataGraphProcessingEnginePropertiesARM,
    ) -> () {
        unsafe {
            self.commands().get_physical_device_queue_family_data_graph_processing_engine_properties(
                self.raw(),
                queue_family_data_graph_processing_engine_info,
                queue_family_data_graph_processing_engine_properties,
            )
        }
    }
    /// ```c
    /// VkResult vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM(VkPhysicalDevice physicalDevice, uint32_t queueFamilyIndex, uint32_t* pQueueFamilyDataGraphPropertyCount, VkQueueFamilyDataGraphPropertiesARM* pQueueFamilyDataGraphProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_queue_family_data_graph_properties(
        &self,
        queue_family_index: uint32_t,
        queue_family_data_graph_properties: Option<&mut ::alloc::vec::Vec<QueueFamilyDataGraphPropertiesARM>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_physical_device_queue_family_data_graph_properties(
                self.raw(),
                queue_family_index,
                queue_family_data_graph_properties,
            )
        }
    }
}

/// `VK_ARM_data_graph` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::arm::data_graph::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::arm::data_graph::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkBindDataGraphPipelineSessionMemoryARM(VkDevice device, uint32_t bindInfoCount, VkBindDataGraphPipelineSessionMemoryInfoARM const* pBindInfos)
    /// ```
    pub unsafe fn bind_data_graph_pipeline_session_memory(
        &self,
        device: vk::Device,
        bind_infos: &[BindDataGraphPipelineSessionMemoryInfoARM],
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.BindDataGraphPipelineSessionMemoryARM(
                device.abi(), 
                bind_infos.len() as _, 
                bind_infos.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// void vkCmdDispatchDataGraphARM(VkCommandBuffer commandBuffer, VkDataGraphPipelineSessionARM session, VkDataGraphPipelineDispatchInfoARM const* pInfo)
    /// ```
    pub unsafe fn cmd_dispatch_data_graph(
        &self,
        command_buffer: vk::CommandBuffer,
        session: vk::DataGraphPipelineSessionARM,
        info: Option<&DataGraphPipelineDispatchInfoARM>,
    ) -> () {
        unsafe {
            self.0.CmdDispatchDataGraphARM(
                command_buffer.abi(), 
                session.abi(), 
                info.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkCreateDataGraphPipelineSessionARM(VkDevice device, VkDataGraphPipelineSessionCreateInfoARM const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkDataGraphPipelineSessionARM* pSession)
    /// ```
    pub unsafe fn create_data_graph_pipeline_session(
        &self,
        device: vk::Device,
        create_info: &DataGraphPipelineSessionCreateInfoARM,
    ) -> crate::Result<vk::DataGraphPipelineSessionARM> {
        unsafe {
            let mut _v: Option<vk::DataGraphPipelineSessionARM> = Default::default();
            let _r = self.0.CreateDataGraphPipelineSessionARM(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateDataGraphPipelinesARM(VkDevice device, VkDeferredOperationKHR deferredOperation, VkPipelineCache pipelineCache, uint32_t createInfoCount, VkDataGraphPipelineCreateInfoARM const* pCreateInfos, VkAllocationCallbacks const* pAllocator, VkPipeline* pPipelines)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::PipelineCompileRequiredExt]
    pub unsafe fn create_data_graph_pipelines(
        &self,
        device: vk::Device,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        pipeline_cache: Option<vk::PipelineCache>,
        create_infos: &[DataGraphPipelineCreateInfoARM],
        pipelines: &mut [Option<vk::Pipeline>],
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.CreateDataGraphPipelinesARM(
                device.abi(), 
                deferred_operation.abi(), 
                pipeline_cache.abi(), 
                create_infos.len() as _, 
                create_infos.abi(), 
                Default::default(), 
                pipelines.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// void vkDestroyDataGraphPipelineSessionARM(VkDevice device, VkDataGraphPipelineSessionARM session, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_data_graph_pipeline_session(
        &self,
        device: vk::Device,
        session: vk::DataGraphPipelineSessionARM,
    ) -> () {
        unsafe {
            self.0.DestroyDataGraphPipelineSessionARM(
                device.abi(), 
                session.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// VkResult vkGetDataGraphPipelineAvailablePropertiesARM(VkDevice device, VkDataGraphPipelineInfoARM const* pPipelineInfo, uint32_t* pPropertiesCount, VkDataGraphPipelinePropertyARM* pProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_data_graph_pipeline_available_properties(
        &self,
        device: vk::Device,
        pipeline_info: &DataGraphPipelineInfoARM,
        properties: Option<&mut ::alloc::vec::Vec<DataGraphPipelinePropertyARM>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetDataGraphPipelineAvailablePropertiesARM(
                device.abi(), 
                pipeline_info.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = properties {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetDataGraphPipelineAvailablePropertiesARM(
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
    /// VkResult vkGetDataGraphPipelinePropertiesARM(VkDevice device, VkDataGraphPipelineInfoARM const* pPipelineInfo, uint32_t propertiesCount, VkDataGraphPipelinePropertyQueryResultARM* pProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_data_graph_pipeline_properties(
        &self,
        device: vk::Device,
        pipeline_info: &DataGraphPipelineInfoARM,
        properties: &mut [DataGraphPipelinePropertyQueryResultARM],
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.GetDataGraphPipelinePropertiesARM(
                device.abi(), 
                pipeline_info.abi(), 
                properties.len() as _, 
                properties.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// VkResult vkGetDataGraphPipelineSessionBindPointRequirementsARM(VkDevice device, VkDataGraphPipelineSessionBindPointRequirementsInfoARM const* pInfo, uint32_t* pBindPointRequirementCount, VkDataGraphPipelineSessionBindPointRequirementARM* pBindPointRequirements)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_data_graph_pipeline_session_bind_point_requirements(
        &self,
        device: vk::Device,
        info: &DataGraphPipelineSessionBindPointRequirementsInfoARM,
        bind_point_requirements: Option<&mut ::alloc::vec::Vec<DataGraphPipelineSessionBindPointRequirementARM>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetDataGraphPipelineSessionBindPointRequirementsARM(
                device.abi(), 
                info.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = bind_point_requirements {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetDataGraphPipelineSessionBindPointRequirementsARM(
                    device.abi(), 
                    info.abi(), 
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
    /// void vkGetDataGraphPipelineSessionMemoryRequirementsARM(VkDevice device, VkDataGraphPipelineSessionMemoryRequirementsInfoARM const* pInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    pub unsafe fn get_data_graph_pipeline_session_memory_requirements(
        &self,
        device: vk::Device,
        info: &DataGraphPipelineSessionMemoryRequirementsInfoARM,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self.0.GetDataGraphPipelineSessionMemoryRequirementsARM(
                device.abi(), 
                info.abi(), 
                memory_requirements.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::arm::data_graph {
    type Commands = Device;
}

/// Device object
pub trait ArmDataGraphDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkBindDataGraphPipelineSessionMemoryARM(VkDevice device, uint32_t bindInfoCount, VkBindDataGraphPipelineSessionMemoryInfoARM const* pBindInfos)
    /// ```
    unsafe fn bind_data_graph_pipeline_session_memory(
        &self,
        bind_infos: &[BindDataGraphPipelineSessionMemoryInfoARM],
    ) -> crate::Result<()> {
        unsafe {
            self.commands().bind_data_graph_pipeline_session_memory(
                self.raw(),
                bind_infos,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateDataGraphPipelineSessionARM(VkDevice device, VkDataGraphPipelineSessionCreateInfoARM const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkDataGraphPipelineSessionARM* pSession)
    /// ```
    unsafe fn create_data_graph_pipeline_session(
        &self,
        create_info: &DataGraphPipelineSessionCreateInfoARM,
    ) -> crate::Result<vk::DataGraphPipelineSessionARM> {
        unsafe {
            self.commands().create_data_graph_pipeline_session(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateDataGraphPipelinesARM(VkDevice device, VkDeferredOperationKHR deferredOperation, VkPipelineCache pipelineCache, uint32_t createInfoCount, VkDataGraphPipelineCreateInfoARM const* pCreateInfos, VkAllocationCallbacks const* pAllocator, VkPipeline* pPipelines)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::PipelineCompileRequiredExt]
    unsafe fn create_data_graph_pipelines(
        &self,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        pipeline_cache: Option<vk::PipelineCache>,
        create_infos: &[DataGraphPipelineCreateInfoARM],
        pipelines: &mut [Option<vk::Pipeline>],
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().create_data_graph_pipelines(
                self.raw(),
                deferred_operation,
                pipeline_cache,
                create_infos,
                pipelines,
            )
        }
    }
    /// ```c
    /// void vkDestroyDataGraphPipelineSessionARM(VkDevice device, VkDataGraphPipelineSessionARM session, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_data_graph_pipeline_session(
        &self,
        session: vk::DataGraphPipelineSessionARM,
    ) -> () {
        unsafe {
            self.commands().destroy_data_graph_pipeline_session(
                self.raw(),
                session,
            )
        }
    }
    /// ```c
    /// VkResult vkGetDataGraphPipelineAvailablePropertiesARM(VkDevice device, VkDataGraphPipelineInfoARM const* pPipelineInfo, uint32_t* pPropertiesCount, VkDataGraphPipelinePropertyARM* pProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_data_graph_pipeline_available_properties(
        &self,
        pipeline_info: &DataGraphPipelineInfoARM,
        properties: Option<&mut ::alloc::vec::Vec<DataGraphPipelinePropertyARM>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_data_graph_pipeline_available_properties(
                self.raw(),
                pipeline_info,
                properties,
            )
        }
    }
    /// ```c
    /// VkResult vkGetDataGraphPipelinePropertiesARM(VkDevice device, VkDataGraphPipelineInfoARM const* pPipelineInfo, uint32_t propertiesCount, VkDataGraphPipelinePropertyQueryResultARM* pProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_data_graph_pipeline_properties(
        &self,
        pipeline_info: &DataGraphPipelineInfoARM,
        properties: &mut [DataGraphPipelinePropertyQueryResultARM],
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().get_data_graph_pipeline_properties(
                self.raw(),
                pipeline_info,
                properties,
            )
        }
    }
    /// ```c
    /// VkResult vkGetDataGraphPipelineSessionBindPointRequirementsARM(VkDevice device, VkDataGraphPipelineSessionBindPointRequirementsInfoARM const* pInfo, uint32_t* pBindPointRequirementCount, VkDataGraphPipelineSessionBindPointRequirementARM* pBindPointRequirements)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_data_graph_pipeline_session_bind_point_requirements(
        &self,
        info: &DataGraphPipelineSessionBindPointRequirementsInfoARM,
        bind_point_requirements: Option<&mut ::alloc::vec::Vec<DataGraphPipelineSessionBindPointRequirementARM>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_data_graph_pipeline_session_bind_point_requirements(
                self.raw(),
                info,
                bind_point_requirements,
            )
        }
    }
    /// ```c
    /// void vkGetDataGraphPipelineSessionMemoryRequirementsARM(VkDevice device, VkDataGraphPipelineSessionMemoryRequirementsInfoARM const* pInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    unsafe fn get_data_graph_pipeline_session_memory_requirements(
        &self,
        info: &DataGraphPipelineSessionMemoryRequirementsInfoARM,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self.commands().get_data_graph_pipeline_session_memory_requirements(
                self.raw(),
                info,
                memory_requirements,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::arm::data_graph {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::arm::data_graph> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::arm::data_graph {
        type Output = crate::hnd::Device<vk::extensions::arm::data_graph>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::arm::data_graph>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::arm::data_graph> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::arm::data_graph> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::arm::data_graph> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::arm::data_graph> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ArmDataGraphDevice for crate::hnd::Device<vk::extensions::arm::data_graph> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::arm::data_graph, vk::Device> for crate::hnd::Device<vk::extensions::arm::data_graph> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait ArmDataGraphCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdDispatchDataGraphARM(VkCommandBuffer commandBuffer, VkDataGraphPipelineSessionARM session, VkDataGraphPipelineDispatchInfoARM const* pInfo)
    /// ```
    unsafe fn dispatch_data_graph(
        &self,
        session: vk::DataGraphPipelineSessionARM,
        info: Option<&DataGraphPipelineDispatchInfoARM>,
    ) -> () {
        unsafe {
            self.commands().cmd_dispatch_data_graph(
                self.raw(),
                session,
                info,
            )
        }
    }
}
