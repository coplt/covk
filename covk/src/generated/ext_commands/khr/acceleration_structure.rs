// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_acceleration_structure` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::acceleration_structure::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::acceleration_structure::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkBuildAccelerationStructuresKHR(VkDevice device, VkDeferredOperationKHR deferredOperation, uint32_t infoCount, VkAccelerationStructureBuildGeometryInfoKHR const* pInfos, VkAccelerationStructureBuildRangeInfoKHR const* const* ppBuildRangeInfos)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::OperationDeferredKhr], [Result::OperationNotDeferredKhr]
    pub unsafe fn build_acceleration_structures(
        &self,
        device: vk::Device,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        infos: &[AccelerationStructureBuildGeometryInfoKHR],
        p_build_range_infos: &[*const AccelerationStructureBuildRangeInfoKHR],
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.BuildAccelerationStructuresKHR(
                device.abi(), 
                deferred_operation.abi(), 
                infos.len() as _, 
                infos.abi(), 
                p_build_range_infos.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// void vkCmdBuildAccelerationStructuresIndirectKHR(VkCommandBuffer commandBuffer, uint32_t infoCount, VkAccelerationStructureBuildGeometryInfoKHR const* pInfos, VkDeviceAddress const* pIndirectDeviceAddresses, uint32_t const* pIndirectStrides, uint32_t const* const* ppMaxPrimitiveCounts)
    /// ```
    pub unsafe fn cmd_build_acceleration_structures_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        infos: &[AccelerationStructureBuildGeometryInfoKHR],
        indirect_device_addresses: &[DeviceAddress],
        indirect_strides: &[uint32_t],
        p_max_primitive_counts: &[*const uint32_t],
    ) -> () {
        unsafe {
            self.0.CmdBuildAccelerationStructuresIndirectKHR(
                command_buffer.abi(), 
                infos.len() as _, 
                infos.abi(), 
                indirect_device_addresses.abi(), 
                indirect_strides.abi(), 
                p_max_primitive_counts.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdBuildAccelerationStructuresKHR(VkCommandBuffer commandBuffer, uint32_t infoCount, VkAccelerationStructureBuildGeometryInfoKHR const* pInfos, VkAccelerationStructureBuildRangeInfoKHR const* const* ppBuildRangeInfos)
    /// ```
    pub unsafe fn cmd_build_acceleration_structures(
        &self,
        command_buffer: vk::CommandBuffer,
        infos: &[AccelerationStructureBuildGeometryInfoKHR],
        p_build_range_infos: &[*const AccelerationStructureBuildRangeInfoKHR],
    ) -> () {
        unsafe {
            self.0.CmdBuildAccelerationStructuresKHR(
                command_buffer.abi(), 
                infos.len() as _, 
                infos.abi(), 
                p_build_range_infos.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdCopyAccelerationStructureKHR(VkCommandBuffer commandBuffer, VkCopyAccelerationStructureInfoKHR const* pInfo)
    /// ```
    pub unsafe fn cmd_copy_acceleration_structure(
        &self,
        command_buffer: vk::CommandBuffer,
        info: &CopyAccelerationStructureInfoKHR,
    ) -> () {
        unsafe {
            self.0.CmdCopyAccelerationStructureKHR(
                command_buffer.abi(), 
                info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdCopyAccelerationStructureToMemoryKHR(VkCommandBuffer commandBuffer, VkCopyAccelerationStructureToMemoryInfoKHR const* pInfo)
    /// ```
    pub unsafe fn cmd_copy_acceleration_structure_to_memory(
        &self,
        command_buffer: vk::CommandBuffer,
        info: &CopyAccelerationStructureToMemoryInfoKHR,
    ) -> () {
        unsafe {
            self.0.CmdCopyAccelerationStructureToMemoryKHR(
                command_buffer.abi(), 
                info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdCopyMemoryToAccelerationStructureKHR(VkCommandBuffer commandBuffer, VkCopyMemoryToAccelerationStructureInfoKHR const* pInfo)
    /// ```
    pub unsafe fn cmd_copy_memory_to_acceleration_structure(
        &self,
        command_buffer: vk::CommandBuffer,
        info: &CopyMemoryToAccelerationStructureInfoKHR,
    ) -> () {
        unsafe {
            self.0.CmdCopyMemoryToAccelerationStructureKHR(
                command_buffer.abi(), 
                info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdWriteAccelerationStructuresPropertiesKHR(VkCommandBuffer commandBuffer, uint32_t accelerationStructureCount, VkAccelerationStructureKHR const* pAccelerationStructures, VkQueryType queryType, VkQueryPool queryPool, uint32_t firstQuery)
    /// ```
    pub unsafe fn cmd_write_acceleration_structures_properties(
        &self,
        command_buffer: vk::CommandBuffer,
        acceleration_structures: &[vk::AccelerationStructureKHR],
        query_type: QueryType,
        query_pool: vk::QueryPool,
        first_query: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdWriteAccelerationStructuresPropertiesKHR(
                command_buffer.abi(), 
                acceleration_structures.len() as _, 
                acceleration_structures.abi(), 
                query_type.abi(), 
                query_pool.abi(), 
                first_query.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkCopyAccelerationStructureKHR(VkDevice device, VkDeferredOperationKHR deferredOperation, VkCopyAccelerationStructureInfoKHR const* pInfo)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::OperationDeferredKhr], [Result::OperationNotDeferredKhr]
    pub unsafe fn copy_acceleration_structure(
        &self,
        device: vk::Device,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        info: &CopyAccelerationStructureInfoKHR,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.CopyAccelerationStructureKHR(
                device.abi(), 
                deferred_operation.abi(), 
                info.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// VkResult vkCopyAccelerationStructureToMemoryKHR(VkDevice device, VkDeferredOperationKHR deferredOperation, VkCopyAccelerationStructureToMemoryInfoKHR const* pInfo)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::OperationDeferredKhr], [Result::OperationNotDeferredKhr]
    pub unsafe fn copy_acceleration_structure_to_memory(
        &self,
        device: vk::Device,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        info: &CopyAccelerationStructureToMemoryInfoKHR,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.CopyAccelerationStructureToMemoryKHR(
                device.abi(), 
                deferred_operation.abi(), 
                info.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// VkResult vkCopyMemoryToAccelerationStructureKHR(VkDevice device, VkDeferredOperationKHR deferredOperation, VkCopyMemoryToAccelerationStructureInfoKHR const* pInfo)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::OperationDeferredKhr], [Result::OperationNotDeferredKhr]
    pub unsafe fn copy_memory_to_acceleration_structure(
        &self,
        device: vk::Device,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        info: &CopyMemoryToAccelerationStructureInfoKHR,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.CopyMemoryToAccelerationStructureKHR(
                device.abi(), 
                deferred_operation.abi(), 
                info.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// VkResult vkCreateAccelerationStructureKHR(VkDevice device, VkAccelerationStructureCreateInfoKHR const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkAccelerationStructureKHR* pAccelerationStructure)
    /// ```
    pub unsafe fn create_acceleration_structure(
        &self,
        device: vk::Device,
        create_info: &AccelerationStructureCreateInfoKHR,
    ) -> crate::Result<vk::AccelerationStructureKHR> {
        unsafe {
            let mut _v: Option<vk::AccelerationStructureKHR> = Default::default();
            let _r = self.0.CreateAccelerationStructureKHR(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDestroyAccelerationStructureKHR(VkDevice device, VkAccelerationStructureKHR accelerationStructure, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_acceleration_structure(
        &self,
        device: vk::Device,
        acceleration_structure: Option<vk::AccelerationStructureKHR>,
    ) -> () {
        unsafe {
            self.0.DestroyAccelerationStructureKHR(
                device.abi(), 
                acceleration_structure.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkGetAccelerationStructureBuildSizesKHR(VkDevice device, VkAccelerationStructureBuildTypeKHR buildType, VkAccelerationStructureBuildGeometryInfoKHR const* pBuildInfo, uint32_t const* pMaxPrimitiveCounts, VkAccelerationStructureBuildSizesInfoKHR* pSizeInfo)
    /// ```
    pub unsafe fn get_acceleration_structure_build_sizes(
        &self,
        device: vk::Device,
        build_type: AccelerationStructureBuildTypeKHR,
        build_info: &AccelerationStructureBuildGeometryInfoKHR,
        max_primitive_counts: *const uint32_t,
        size_info: &mut AccelerationStructureBuildSizesInfoKHR,
    ) -> () {
        unsafe {
            self.0.GetAccelerationStructureBuildSizesKHR(
                device.abi(), 
                build_type.abi(), 
                build_info.abi(), 
                max_primitive_counts.abi(), 
                size_info.abi(), 
            );
        }
    }
    /// ```c
    /// VkDeviceAddress vkGetAccelerationStructureDeviceAddressKHR(VkDevice device, VkAccelerationStructureDeviceAddressInfoKHR const* pInfo)
    /// ```
    pub unsafe fn get_acceleration_structure_device_address(
        &self,
        device: vk::Device,
        info: &AccelerationStructureDeviceAddressInfoKHR,
    ) -> DeviceAddress {
        unsafe {
            let _r = self.0.GetAccelerationStructureDeviceAddressKHR(
                device.abi(), 
                info.abi(), 
            );
            _r
        }
    }
    /// ```c
    /// void vkGetDeviceAccelerationStructureCompatibilityKHR(VkDevice device, VkAccelerationStructureVersionInfoKHR const* pVersionInfo, VkAccelerationStructureCompatibilityKHR* pCompatibility)
    /// ```
    pub unsafe fn get_device_acceleration_structure_compatibility(
        &self,
        device: vk::Device,
        version_info: &AccelerationStructureVersionInfoKHR,
    ) -> AccelerationStructureCompatibilityKHR {
        unsafe {
            let mut _v: AccelerationStructureCompatibilityKHR = Default::default();
            self.0.GetDeviceAccelerationStructureCompatibilityKHR(
                device.abi(), 
                version_info.abi(), 
                (&mut _v).abi(), 
            );
            _v
        }
    }
    /// ```c
    /// VkResult vkWriteAccelerationStructuresPropertiesKHR(VkDevice device, uint32_t accelerationStructureCount, VkAccelerationStructureKHR const* pAccelerationStructures, VkQueryType queryType, size_t dataSize, void* pData, size_t stride)
    /// ```
    pub unsafe fn write_acceleration_structures_properties(
        &self,
        device: vk::Device,
        acceleration_structures: &[vk::AccelerationStructureKHR],
        query_type: QueryType,
        data_size: size_t,
        data: *mut void,
        stride: size_t,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.WriteAccelerationStructuresPropertiesKHR(
                device.abi(), 
                acceleration_structures.len() as _, 
                acceleration_structures.abi(), 
                query_type.abi(), 
                data_size.abi(), 
                data.abi(), 
                stride.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::acceleration_structure {
    type Commands = Device;
}

/// Device object
pub trait KhrAccelerationStructureDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkBuildAccelerationStructuresKHR(VkDevice device, VkDeferredOperationKHR deferredOperation, uint32_t infoCount, VkAccelerationStructureBuildGeometryInfoKHR const* pInfos, VkAccelerationStructureBuildRangeInfoKHR const* const* ppBuildRangeInfos)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::OperationDeferredKhr], [Result::OperationNotDeferredKhr]
    unsafe fn build_acceleration_structures(
        &self,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        infos: &[AccelerationStructureBuildGeometryInfoKHR],
        p_build_range_infos: &[*const AccelerationStructureBuildRangeInfoKHR],
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().build_acceleration_structures(
                self.raw(),
                deferred_operation,
                infos,
                p_build_range_infos,
            )
        }
    }
    /// ```c
    /// VkResult vkCopyAccelerationStructureKHR(VkDevice device, VkDeferredOperationKHR deferredOperation, VkCopyAccelerationStructureInfoKHR const* pInfo)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::OperationDeferredKhr], [Result::OperationNotDeferredKhr]
    unsafe fn copy_acceleration_structure(
        &self,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        info: &CopyAccelerationStructureInfoKHR,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().copy_acceleration_structure(
                self.raw(),
                deferred_operation,
                info,
            )
        }
    }
    /// ```c
    /// VkResult vkCopyAccelerationStructureToMemoryKHR(VkDevice device, VkDeferredOperationKHR deferredOperation, VkCopyAccelerationStructureToMemoryInfoKHR const* pInfo)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::OperationDeferredKhr], [Result::OperationNotDeferredKhr]
    unsafe fn copy_acceleration_structure_to_memory(
        &self,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        info: &CopyAccelerationStructureToMemoryInfoKHR,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().copy_acceleration_structure_to_memory(
                self.raw(),
                deferred_operation,
                info,
            )
        }
    }
    /// ```c
    /// VkResult vkCopyMemoryToAccelerationStructureKHR(VkDevice device, VkDeferredOperationKHR deferredOperation, VkCopyMemoryToAccelerationStructureInfoKHR const* pInfo)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::OperationDeferredKhr], [Result::OperationNotDeferredKhr]
    unsafe fn copy_memory_to_acceleration_structure(
        &self,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        info: &CopyMemoryToAccelerationStructureInfoKHR,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().copy_memory_to_acceleration_structure(
                self.raw(),
                deferred_operation,
                info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateAccelerationStructureKHR(VkDevice device, VkAccelerationStructureCreateInfoKHR const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkAccelerationStructureKHR* pAccelerationStructure)
    /// ```
    unsafe fn create_acceleration_structure(
        &self,
        create_info: &AccelerationStructureCreateInfoKHR,
    ) -> crate::Result<vk::AccelerationStructureKHR> {
        unsafe {
            self.commands().create_acceleration_structure(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// void vkDestroyAccelerationStructureKHR(VkDevice device, VkAccelerationStructureKHR accelerationStructure, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_acceleration_structure(
        &self,
        acceleration_structure: Option<vk::AccelerationStructureKHR>,
    ) -> () {
        unsafe {
            self.commands().destroy_acceleration_structure(
                self.raw(),
                acceleration_structure,
            )
        }
    }
    /// ```c
    /// void vkGetAccelerationStructureBuildSizesKHR(VkDevice device, VkAccelerationStructureBuildTypeKHR buildType, VkAccelerationStructureBuildGeometryInfoKHR const* pBuildInfo, uint32_t const* pMaxPrimitiveCounts, VkAccelerationStructureBuildSizesInfoKHR* pSizeInfo)
    /// ```
    unsafe fn get_acceleration_structure_build_sizes(
        &self,
        build_type: AccelerationStructureBuildTypeKHR,
        build_info: &AccelerationStructureBuildGeometryInfoKHR,
        max_primitive_counts: *const uint32_t,
        size_info: &mut AccelerationStructureBuildSizesInfoKHR,
    ) -> () {
        unsafe {
            self.commands().get_acceleration_structure_build_sizes(
                self.raw(),
                build_type,
                build_info,
                max_primitive_counts,
                size_info,
            )
        }
    }
    /// ```c
    /// VkDeviceAddress vkGetAccelerationStructureDeviceAddressKHR(VkDevice device, VkAccelerationStructureDeviceAddressInfoKHR const* pInfo)
    /// ```
    unsafe fn get_acceleration_structure_device_address(
        &self,
        info: &AccelerationStructureDeviceAddressInfoKHR,
    ) -> DeviceAddress {
        unsafe {
            self.commands().get_acceleration_structure_device_address(
                self.raw(),
                info,
            )
        }
    }
    /// ```c
    /// void vkGetDeviceAccelerationStructureCompatibilityKHR(VkDevice device, VkAccelerationStructureVersionInfoKHR const* pVersionInfo, VkAccelerationStructureCompatibilityKHR* pCompatibility)
    /// ```
    unsafe fn get_device_acceleration_structure_compatibility(
        &self,
        version_info: &AccelerationStructureVersionInfoKHR,
    ) -> AccelerationStructureCompatibilityKHR {
        unsafe {
            self.commands().get_device_acceleration_structure_compatibility(
                self.raw(),
                version_info,
            )
        }
    }
    /// ```c
    /// VkResult vkWriteAccelerationStructuresPropertiesKHR(VkDevice device, uint32_t accelerationStructureCount, VkAccelerationStructureKHR const* pAccelerationStructures, VkQueryType queryType, size_t dataSize, void* pData, size_t stride)
    /// ```
    unsafe fn write_acceleration_structures_properties(
        &self,
        acceleration_structures: &[vk::AccelerationStructureKHR],
        query_type: QueryType,
        data_size: size_t,
        data: *mut void,
        stride: size_t,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().write_acceleration_structures_properties(
                self.raw(),
                acceleration_structures,
                query_type,
                data_size,
                data,
                stride,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::acceleration_structure {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::acceleration_structure> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::acceleration_structure {
        type Output = crate::hnd::Device<vk::extensions::khr::acceleration_structure>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::acceleration_structure>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::acceleration_structure> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::acceleration_structure> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::acceleration_structure> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::acceleration_structure> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrAccelerationStructureDevice for crate::hnd::Device<vk::extensions::khr::acceleration_structure> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::acceleration_structure, vk::Device> for crate::hnd::Device<vk::extensions::khr::acceleration_structure> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait KhrAccelerationStructureCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdBuildAccelerationStructuresIndirectKHR(VkCommandBuffer commandBuffer, uint32_t infoCount, VkAccelerationStructureBuildGeometryInfoKHR const* pInfos, VkDeviceAddress const* pIndirectDeviceAddresses, uint32_t const* pIndirectStrides, uint32_t const* const* ppMaxPrimitiveCounts)
    /// ```
    unsafe fn build_acceleration_structures_indirect(
        &self,
        infos: &[AccelerationStructureBuildGeometryInfoKHR],
        indirect_device_addresses: &[DeviceAddress],
        indirect_strides: &[uint32_t],
        p_max_primitive_counts: &[*const uint32_t],
    ) -> () {
        unsafe {
            self.commands().cmd_build_acceleration_structures_indirect(
                self.raw(),
                infos,
                indirect_device_addresses,
                indirect_strides,
                p_max_primitive_counts,
            )
        }
    }
    /// ```c
    /// void vkCmdBuildAccelerationStructuresKHR(VkCommandBuffer commandBuffer, uint32_t infoCount, VkAccelerationStructureBuildGeometryInfoKHR const* pInfos, VkAccelerationStructureBuildRangeInfoKHR const* const* ppBuildRangeInfos)
    /// ```
    unsafe fn build_acceleration_structures(
        &self,
        infos: &[AccelerationStructureBuildGeometryInfoKHR],
        p_build_range_infos: &[*const AccelerationStructureBuildRangeInfoKHR],
    ) -> () {
        unsafe {
            self.commands().cmd_build_acceleration_structures(
                self.raw(),
                infos,
                p_build_range_infos,
            )
        }
    }
    /// ```c
    /// void vkCmdCopyAccelerationStructureKHR(VkCommandBuffer commandBuffer, VkCopyAccelerationStructureInfoKHR const* pInfo)
    /// ```
    unsafe fn copy_acceleration_structure(
        &self,
        info: &CopyAccelerationStructureInfoKHR,
    ) -> () {
        unsafe {
            self.commands().cmd_copy_acceleration_structure(
                self.raw(),
                info,
            )
        }
    }
    /// ```c
    /// void vkCmdCopyAccelerationStructureToMemoryKHR(VkCommandBuffer commandBuffer, VkCopyAccelerationStructureToMemoryInfoKHR const* pInfo)
    /// ```
    unsafe fn copy_acceleration_structure_to_memory(
        &self,
        info: &CopyAccelerationStructureToMemoryInfoKHR,
    ) -> () {
        unsafe {
            self.commands().cmd_copy_acceleration_structure_to_memory(
                self.raw(),
                info,
            )
        }
    }
    /// ```c
    /// void vkCmdCopyMemoryToAccelerationStructureKHR(VkCommandBuffer commandBuffer, VkCopyMemoryToAccelerationStructureInfoKHR const* pInfo)
    /// ```
    unsafe fn copy_memory_to_acceleration_structure(
        &self,
        info: &CopyMemoryToAccelerationStructureInfoKHR,
    ) -> () {
        unsafe {
            self.commands().cmd_copy_memory_to_acceleration_structure(
                self.raw(),
                info,
            )
        }
    }
    /// ```c
    /// void vkCmdWriteAccelerationStructuresPropertiesKHR(VkCommandBuffer commandBuffer, uint32_t accelerationStructureCount, VkAccelerationStructureKHR const* pAccelerationStructures, VkQueryType queryType, VkQueryPool queryPool, uint32_t firstQuery)
    /// ```
    unsafe fn write_acceleration_structures_properties(
        &self,
        acceleration_structures: &[vk::AccelerationStructureKHR],
        query_type: QueryType,
        query_pool: vk::QueryPool,
        first_query: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_write_acceleration_structures_properties(
                self.raw(),
                acceleration_structures,
                query_type,
                query_pool,
                first_query,
            )
        }
    }
}

impl<O: crate::HndCtx<vk::extensions::khr::acceleration_structure, vk::Device>> crate::Owner<vk::AccelerationStructureKHR, vk::extensions::khr::acceleration_structure> for O {
    fn drop(&mut self, raw: vk::AccelerationStructureKHR) {
        unsafe {
            self.commands().destroy_acceleration_structure(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::khr::acceleration_structure, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::AccelerationStructureKHR, O, vk::extensions::khr::acceleration_structure>>
    where O: crate::HndCtx<vk::extensions::khr::acceleration_structure, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::AccelerationStructureKHR> for vk::extensions::khr::acceleration_structure {
    type Impl = _hs_AccelerationStructureKHR::AccelerationStructureKHR;
}


mod _hs_AccelerationStructureKHR {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct AccelerationStructureKHR(pub(crate) crate::handle::Hnd<vk::AccelerationStructureKHR, ::alloc::sync::Arc<super::Device>>);

    impl Clone for AccelerationStructureKHR {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::AccelerationStructureKHR, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::extensions::khr::acceleration_structure, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::AccelerationStructureKHR, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_acceleration_structure(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::AccelerationStructureKHR<vk::extensions::khr::acceleration_structure>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::khr::acceleration_structure, vk::Device>, raw: vk::AccelerationStructureKHR) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::khr::acceleration_structure, vk::Device>, raw: vk::AccelerationStructureKHR, dep: impl FnOnce() -> Dep) -> Self {
            Self(AccelerationStructureKHR(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::AccelerationStructureKHR<vk::extensions::khr::acceleration_structure> {
        pub fn raw(&self) -> vk::AccelerationStructureKHR { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::AccelerationStructureKHR<vk::extensions::khr::acceleration_structure> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("AccelerationStructureKHR({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::AccelerationStructureKHR<vk::extensions::khr::acceleration_structure> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::khr::acceleration_structure> for vk::AccelerationStructureKHR
        where Ctx: crate::HndCtx<vk::extensions::khr::acceleration_structure, vk::Device>,
    {
        type Output = crate::hnd::AccelerationStructureKHR<vk::extensions::khr::acceleration_structure>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::AccelerationStructureKHR::<vk::extensions::khr::acceleration_structure>::new_with(ctx, self, dep) }
        }
    }
}
