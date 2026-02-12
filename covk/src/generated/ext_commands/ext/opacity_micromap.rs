// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_opacity_micromap` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::opacity_micromap::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::opacity_micromap::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkBuildMicromapsEXT(VkDevice device, VkDeferredOperationKHR deferredOperation, uint32_t infoCount, VkMicromapBuildInfoEXT const* pInfos)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::OperationDeferredKhr], [Result::OperationNotDeferredKhr]
    pub unsafe fn build_micromaps(
        &self,
        device: vk::Device,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        infos: &[MicromapBuildInfoEXT],
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.BuildMicromapsEXT(
                device.abi(), 
                deferred_operation.abi(), 
                infos.len() as _, 
                infos.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// void vkCmdBuildMicromapsEXT(VkCommandBuffer commandBuffer, uint32_t infoCount, VkMicromapBuildInfoEXT const* pInfos)
    /// ```
    pub unsafe fn cmd_build_micromaps(
        &self,
        command_buffer: vk::CommandBuffer,
        infos: &[MicromapBuildInfoEXT],
    ) -> () {
        unsafe {
            self.0.CmdBuildMicromapsEXT(
                command_buffer.abi(), 
                infos.len() as _, 
                infos.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdCopyMemoryToMicromapEXT(VkCommandBuffer commandBuffer, VkCopyMemoryToMicromapInfoEXT const* pInfo)
    /// ```
    pub unsafe fn cmd_copy_memory_to_micromap(
        &self,
        command_buffer: vk::CommandBuffer,
        info: &CopyMemoryToMicromapInfoEXT,
    ) -> () {
        unsafe {
            self.0.CmdCopyMemoryToMicromapEXT(
                command_buffer.abi(), 
                info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdCopyMicromapEXT(VkCommandBuffer commandBuffer, VkCopyMicromapInfoEXT const* pInfo)
    /// ```
    pub unsafe fn cmd_copy_micromap(
        &self,
        command_buffer: vk::CommandBuffer,
        info: &CopyMicromapInfoEXT,
    ) -> () {
        unsafe {
            self.0.CmdCopyMicromapEXT(
                command_buffer.abi(), 
                info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdCopyMicromapToMemoryEXT(VkCommandBuffer commandBuffer, VkCopyMicromapToMemoryInfoEXT const* pInfo)
    /// ```
    pub unsafe fn cmd_copy_micromap_to_memory(
        &self,
        command_buffer: vk::CommandBuffer,
        info: &CopyMicromapToMemoryInfoEXT,
    ) -> () {
        unsafe {
            self.0.CmdCopyMicromapToMemoryEXT(
                command_buffer.abi(), 
                info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdWriteMicromapsPropertiesEXT(VkCommandBuffer commandBuffer, uint32_t micromapCount, VkMicromapEXT const* pMicromaps, VkQueryType queryType, VkQueryPool queryPool, uint32_t firstQuery)
    /// ```
    pub unsafe fn cmd_write_micromaps_properties(
        &self,
        command_buffer: vk::CommandBuffer,
        micromaps: &[vk::MicromapEXT],
        query_type: QueryType,
        query_pool: vk::QueryPool,
        first_query: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdWriteMicromapsPropertiesEXT(
                command_buffer.abi(), 
                micromaps.len() as _, 
                micromaps.abi(), 
                query_type.abi(), 
                query_pool.abi(), 
                first_query.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkCopyMemoryToMicromapEXT(VkDevice device, VkDeferredOperationKHR deferredOperation, VkCopyMemoryToMicromapInfoEXT const* pInfo)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::OperationDeferredKhr], [Result::OperationNotDeferredKhr]
    pub unsafe fn copy_memory_to_micromap(
        &self,
        device: vk::Device,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        info: &CopyMemoryToMicromapInfoEXT,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.CopyMemoryToMicromapEXT(
                device.abi(), 
                deferred_operation.abi(), 
                info.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// VkResult vkCopyMicromapEXT(VkDevice device, VkDeferredOperationKHR deferredOperation, VkCopyMicromapInfoEXT const* pInfo)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::OperationDeferredKhr], [Result::OperationNotDeferredKhr]
    pub unsafe fn copy_micromap(
        &self,
        device: vk::Device,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        info: &CopyMicromapInfoEXT,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.CopyMicromapEXT(
                device.abi(), 
                deferred_operation.abi(), 
                info.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// VkResult vkCopyMicromapToMemoryEXT(VkDevice device, VkDeferredOperationKHR deferredOperation, VkCopyMicromapToMemoryInfoEXT const* pInfo)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::OperationDeferredKhr], [Result::OperationNotDeferredKhr]
    pub unsafe fn copy_micromap_to_memory(
        &self,
        device: vk::Device,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        info: &CopyMicromapToMemoryInfoEXT,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.CopyMicromapToMemoryEXT(
                device.abi(), 
                deferred_operation.abi(), 
                info.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// VkResult vkCreateMicromapEXT(VkDevice device, VkMicromapCreateInfoEXT const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkMicromapEXT* pMicromap)
    /// ```
    pub unsafe fn create_micromap(
        &self,
        device: vk::Device,
        create_info: &MicromapCreateInfoEXT,
    ) -> crate::Result<vk::MicromapEXT> {
        unsafe {
            let mut _v: Option<vk::MicromapEXT> = Default::default();
            let _r = self.0.CreateMicromapEXT(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDestroyMicromapEXT(VkDevice device, VkMicromapEXT micromap, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_micromap(
        &self,
        device: vk::Device,
        micromap: Option<vk::MicromapEXT>,
    ) -> () {
        unsafe {
            self.0.DestroyMicromapEXT(
                device.abi(), 
                micromap.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkGetDeviceMicromapCompatibilityEXT(VkDevice device, VkMicromapVersionInfoEXT const* pVersionInfo, VkAccelerationStructureCompatibilityKHR* pCompatibility)
    /// ```
    pub unsafe fn get_device_micromap_compatibility(
        &self,
        device: vk::Device,
        version_info: &MicromapVersionInfoEXT,
    ) -> AccelerationStructureCompatibilityKHR {
        unsafe {
            let mut _v: AccelerationStructureCompatibilityKHR = Default::default();
            self.0.GetDeviceMicromapCompatibilityEXT(
                device.abi(), 
                version_info.abi(), 
                (&mut _v).abi(), 
            );
            _v
        }
    }
    /// ```c
    /// void vkGetMicromapBuildSizesEXT(VkDevice device, VkAccelerationStructureBuildTypeKHR buildType, VkMicromapBuildInfoEXT const* pBuildInfo, VkMicromapBuildSizesInfoEXT* pSizeInfo)
    /// ```
    pub unsafe fn get_micromap_build_sizes(
        &self,
        device: vk::Device,
        build_type: AccelerationStructureBuildTypeKHR,
        build_info: &MicromapBuildInfoEXT,
        size_info: &mut MicromapBuildSizesInfoEXT,
    ) -> () {
        unsafe {
            self.0.GetMicromapBuildSizesEXT(
                device.abi(), 
                build_type.abi(), 
                build_info.abi(), 
                size_info.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkWriteMicromapsPropertiesEXT(VkDevice device, uint32_t micromapCount, VkMicromapEXT const* pMicromaps, VkQueryType queryType, size_t dataSize, void* pData, size_t stride)
    /// ```
    pub unsafe fn write_micromaps_properties(
        &self,
        device: vk::Device,
        micromaps: &[vk::MicromapEXT],
        query_type: QueryType,
        data_size: size_t,
        data: *mut void,
        stride: size_t,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.WriteMicromapsPropertiesEXT(
                device.abi(), 
                micromaps.len() as _, 
                micromaps.abi(), 
                query_type.abi(), 
                data_size.abi(), 
                data.abi(), 
                stride.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::opacity_micromap {
    type Commands = Device;
}

/// Device object
pub trait ExtOpacityMicromapDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkBuildMicromapsEXT(VkDevice device, VkDeferredOperationKHR deferredOperation, uint32_t infoCount, VkMicromapBuildInfoEXT const* pInfos)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::OperationDeferredKhr], [Result::OperationNotDeferredKhr]
    unsafe fn build_micromaps(
        &self,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        infos: &[MicromapBuildInfoEXT],
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().build_micromaps(
                self.raw(),
                deferred_operation,
                infos,
            )
        }
    }
    /// ```c
    /// VkResult vkCopyMemoryToMicromapEXT(VkDevice device, VkDeferredOperationKHR deferredOperation, VkCopyMemoryToMicromapInfoEXT const* pInfo)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::OperationDeferredKhr], [Result::OperationNotDeferredKhr]
    unsafe fn copy_memory_to_micromap(
        &self,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        info: &CopyMemoryToMicromapInfoEXT,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().copy_memory_to_micromap(
                self.raw(),
                deferred_operation,
                info,
            )
        }
    }
    /// ```c
    /// VkResult vkCopyMicromapEXT(VkDevice device, VkDeferredOperationKHR deferredOperation, VkCopyMicromapInfoEXT const* pInfo)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::OperationDeferredKhr], [Result::OperationNotDeferredKhr]
    unsafe fn copy_micromap(
        &self,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        info: &CopyMicromapInfoEXT,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().copy_micromap(
                self.raw(),
                deferred_operation,
                info,
            )
        }
    }
    /// ```c
    /// VkResult vkCopyMicromapToMemoryEXT(VkDevice device, VkDeferredOperationKHR deferredOperation, VkCopyMicromapToMemoryInfoEXT const* pInfo)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::OperationDeferredKhr], [Result::OperationNotDeferredKhr]
    unsafe fn copy_micromap_to_memory(
        &self,
        deferred_operation: Option<vk::DeferredOperationKHR>,
        info: &CopyMicromapToMemoryInfoEXT,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().copy_micromap_to_memory(
                self.raw(),
                deferred_operation,
                info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateMicromapEXT(VkDevice device, VkMicromapCreateInfoEXT const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkMicromapEXT* pMicromap)
    /// ```
    unsafe fn create_micromap(
        &self,
        create_info: &MicromapCreateInfoEXT,
    ) -> crate::Result<vk::MicromapEXT> {
        unsafe {
            self.commands().create_micromap(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// void vkDestroyMicromapEXT(VkDevice device, VkMicromapEXT micromap, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_micromap(
        &self,
        micromap: Option<vk::MicromapEXT>,
    ) -> () {
        unsafe {
            self.commands().destroy_micromap(
                self.raw(),
                micromap,
            )
        }
    }
    /// ```c
    /// void vkGetDeviceMicromapCompatibilityEXT(VkDevice device, VkMicromapVersionInfoEXT const* pVersionInfo, VkAccelerationStructureCompatibilityKHR* pCompatibility)
    /// ```
    unsafe fn get_device_micromap_compatibility(
        &self,
        version_info: &MicromapVersionInfoEXT,
    ) -> AccelerationStructureCompatibilityKHR {
        unsafe {
            self.commands().get_device_micromap_compatibility(
                self.raw(),
                version_info,
            )
        }
    }
    /// ```c
    /// void vkGetMicromapBuildSizesEXT(VkDevice device, VkAccelerationStructureBuildTypeKHR buildType, VkMicromapBuildInfoEXT const* pBuildInfo, VkMicromapBuildSizesInfoEXT* pSizeInfo)
    /// ```
    unsafe fn get_micromap_build_sizes(
        &self,
        build_type: AccelerationStructureBuildTypeKHR,
        build_info: &MicromapBuildInfoEXT,
        size_info: &mut MicromapBuildSizesInfoEXT,
    ) -> () {
        unsafe {
            self.commands().get_micromap_build_sizes(
                self.raw(),
                build_type,
                build_info,
                size_info,
            )
        }
    }
    /// ```c
    /// VkResult vkWriteMicromapsPropertiesEXT(VkDevice device, uint32_t micromapCount, VkMicromapEXT const* pMicromaps, VkQueryType queryType, size_t dataSize, void* pData, size_t stride)
    /// ```
    unsafe fn write_micromaps_properties(
        &self,
        micromaps: &[vk::MicromapEXT],
        query_type: QueryType,
        data_size: size_t,
        data: *mut void,
        stride: size_t,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().write_micromaps_properties(
                self.raw(),
                micromaps,
                query_type,
                data_size,
                data,
                stride,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::ext::opacity_micromap {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::opacity_micromap> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::opacity_micromap {
        type Output = crate::hnd::Device<vk::extensions::ext::opacity_micromap>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::opacity_micromap>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::opacity_micromap> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::opacity_micromap> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::opacity_micromap> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::opacity_micromap> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtOpacityMicromapDevice for crate::hnd::Device<vk::extensions::ext::opacity_micromap> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::opacity_micromap, vk::Device> for crate::hnd::Device<vk::extensions::ext::opacity_micromap> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait ExtOpacityMicromapCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdBuildMicromapsEXT(VkCommandBuffer commandBuffer, uint32_t infoCount, VkMicromapBuildInfoEXT const* pInfos)
    /// ```
    unsafe fn build_micromaps(
        &self,
        infos: &[MicromapBuildInfoEXT],
    ) -> () {
        unsafe {
            self.commands().cmd_build_micromaps(
                self.raw(),
                infos,
            )
        }
    }
    /// ```c
    /// void vkCmdCopyMemoryToMicromapEXT(VkCommandBuffer commandBuffer, VkCopyMemoryToMicromapInfoEXT const* pInfo)
    /// ```
    unsafe fn copy_memory_to_micromap(
        &self,
        info: &CopyMemoryToMicromapInfoEXT,
    ) -> () {
        unsafe {
            self.commands().cmd_copy_memory_to_micromap(
                self.raw(),
                info,
            )
        }
    }
    /// ```c
    /// void vkCmdCopyMicromapEXT(VkCommandBuffer commandBuffer, VkCopyMicromapInfoEXT const* pInfo)
    /// ```
    unsafe fn copy_micromap(
        &self,
        info: &CopyMicromapInfoEXT,
    ) -> () {
        unsafe {
            self.commands().cmd_copy_micromap(
                self.raw(),
                info,
            )
        }
    }
    /// ```c
    /// void vkCmdCopyMicromapToMemoryEXT(VkCommandBuffer commandBuffer, VkCopyMicromapToMemoryInfoEXT const* pInfo)
    /// ```
    unsafe fn copy_micromap_to_memory(
        &self,
        info: &CopyMicromapToMemoryInfoEXT,
    ) -> () {
        unsafe {
            self.commands().cmd_copy_micromap_to_memory(
                self.raw(),
                info,
            )
        }
    }
    /// ```c
    /// void vkCmdWriteMicromapsPropertiesEXT(VkCommandBuffer commandBuffer, uint32_t micromapCount, VkMicromapEXT const* pMicromaps, VkQueryType queryType, VkQueryPool queryPool, uint32_t firstQuery)
    /// ```
    unsafe fn write_micromaps_properties(
        &self,
        micromaps: &[vk::MicromapEXT],
        query_type: QueryType,
        query_pool: vk::QueryPool,
        first_query: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_write_micromaps_properties(
                self.raw(),
                micromaps,
                query_type,
                query_pool,
                first_query,
            )
        }
    }
}

impl<O: crate::HndCtx<vk::extensions::ext::opacity_micromap, vk::Device>> crate::Owner<vk::MicromapEXT, vk::extensions::ext::opacity_micromap> for O {
    fn drop(&mut self, raw: vk::MicromapEXT) {
        unsafe {
            self.commands().destroy_micromap(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::ext::opacity_micromap, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::MicromapEXT, O, vk::extensions::ext::opacity_micromap>>
    where O: crate::HndCtx<vk::extensions::ext::opacity_micromap, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::MicromapEXT> for vk::extensions::ext::opacity_micromap {
    type Impl = _hs_MicromapEXT::MicromapEXT;
}


mod _hs_MicromapEXT {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct MicromapEXT(pub(crate) crate::handle::Hnd<vk::MicromapEXT, ::alloc::sync::Arc<super::Device>>);

    impl Clone for MicromapEXT {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::MicromapEXT, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::extensions::ext::opacity_micromap, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::MicromapEXT, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_micromap(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::MicromapEXT<vk::extensions::ext::opacity_micromap>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::ext::opacity_micromap, vk::Device>, raw: vk::MicromapEXT) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::ext::opacity_micromap, vk::Device>, raw: vk::MicromapEXT, dep: impl FnOnce() -> Dep) -> Self {
            Self(MicromapEXT(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::MicromapEXT<vk::extensions::ext::opacity_micromap> {
        pub fn raw(&self) -> vk::MicromapEXT { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::MicromapEXT<vk::extensions::ext::opacity_micromap> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("MicromapEXT({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::MicromapEXT<vk::extensions::ext::opacity_micromap> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::ext::opacity_micromap> for vk::MicromapEXT
        where Ctx: crate::HndCtx<vk::extensions::ext::opacity_micromap, vk::Device>,
    {
        type Output = crate::hnd::MicromapEXT<vk::extensions::ext::opacity_micromap>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::MicromapEXT::<vk::extensions::ext::opacity_micromap>::new_with(ctx, self, dep) }
        }
    }
}
