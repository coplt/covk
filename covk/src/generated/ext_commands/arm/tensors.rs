// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_ARM_tensors` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::arm::tensors::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::arm::tensors::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// void vkGetPhysicalDeviceExternalTensorPropertiesARM(VkPhysicalDevice physicalDevice, VkPhysicalDeviceExternalTensorInfoARM const* pExternalTensorInfo, VkExternalTensorPropertiesARM* pExternalTensorProperties)
    /// ```
    pub unsafe fn get_physical_device_external_tensor_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        external_tensor_info: &PhysicalDeviceExternalTensorInfoARM,
        external_tensor_properties: &mut ExternalTensorPropertiesARM,
    ) -> () {
        unsafe {
            self.0.GetPhysicalDeviceExternalTensorPropertiesARM(
                physical_device.abi(), 
                external_tensor_info.abi(), 
                external_tensor_properties.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Instance> for vk::extensions::arm::tensors {
    type Commands = Instance;
}

/// Instance object
pub trait ArmTensorsInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::arm::tensors {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::arm::tensors> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::arm::tensors {
        type Output = crate::hnd::Instance<vk::extensions::arm::tensors>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::arm::tensors>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::arm::tensors> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::arm::tensors> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::arm::tensors> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::arm::tensors> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ArmTensorsInstance for crate::hnd::Instance<vk::extensions::arm::tensors> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::arm::tensors, vk::Instance> for crate::hnd::Instance<vk::extensions::arm::tensors> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait ArmTensorsPhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// void vkGetPhysicalDeviceExternalTensorPropertiesARM(VkPhysicalDevice physicalDevice, VkPhysicalDeviceExternalTensorInfoARM const* pExternalTensorInfo, VkExternalTensorPropertiesARM* pExternalTensorProperties)
    /// ```
    unsafe fn get_external_tensor_properties(
        &self,
        external_tensor_info: &PhysicalDeviceExternalTensorInfoARM,
        external_tensor_properties: &mut ExternalTensorPropertiesARM,
    ) -> () {
        unsafe {
            self.commands().get_physical_device_external_tensor_properties(
                self.raw(),
                external_tensor_info,
                external_tensor_properties,
            )
        }
    }
}

/// `VK_ARM_tensors` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::arm::tensors::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::arm::tensors::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkBindTensorMemoryARM(VkDevice device, uint32_t bindInfoCount, VkBindTensorMemoryInfoARM const* pBindInfos)
    /// ```
    pub unsafe fn bind_tensor_memory(
        &self,
        device: vk::Device,
        bind_infos: &[BindTensorMemoryInfoARM],
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.BindTensorMemoryARM(
                device.abi(), 
                bind_infos.len() as _, 
                bind_infos.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// void vkCmdCopyTensorARM(VkCommandBuffer commandBuffer, VkCopyTensorInfoARM const* pCopyTensorInfo)
    /// ```
    pub unsafe fn cmd_copy_tensor(
        &self,
        command_buffer: vk::CommandBuffer,
        copy_tensor_info: &CopyTensorInfoARM,
    ) -> () {
        unsafe {
            self.0.CmdCopyTensorARM(
                command_buffer.abi(), 
                copy_tensor_info.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkCreateTensorARM(VkDevice device, VkTensorCreateInfoARM const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkTensorARM* pTensor)
    /// ```
    pub unsafe fn create_tensor(
        &self,
        device: vk::Device,
        create_info: &TensorCreateInfoARM,
    ) -> crate::Result<vk::TensorARM> {
        unsafe {
            let mut _v: Option<vk::TensorARM> = Default::default();
            let _r = self.0.CreateTensorARM(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateTensorViewARM(VkDevice device, VkTensorViewCreateInfoARM const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkTensorViewARM* pView)
    /// ```
    pub unsafe fn create_tensor_view(
        &self,
        device: vk::Device,
        create_info: &TensorViewCreateInfoARM,
    ) -> crate::Result<vk::TensorViewARM> {
        unsafe {
            let mut _v: Option<vk::TensorViewARM> = Default::default();
            let _r = self.0.CreateTensorViewARM(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDestroyTensorARM(VkDevice device, VkTensorARM tensor, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_tensor(
        &self,
        device: vk::Device,
        tensor: Option<vk::TensorARM>,
    ) -> () {
        unsafe {
            self.0.DestroyTensorARM(
                device.abi(), 
                tensor.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroyTensorViewARM(VkDevice device, VkTensorViewARM tensorView, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_tensor_view(
        &self,
        device: vk::Device,
        tensor_view: Option<vk::TensorViewARM>,
    ) -> () {
        unsafe {
            self.0.DestroyTensorViewARM(
                device.abi(), 
                tensor_view.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkGetDeviceTensorMemoryRequirementsARM(VkDevice device, VkDeviceTensorMemoryRequirementsARM const* pInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    pub unsafe fn get_device_tensor_memory_requirements(
        &self,
        device: vk::Device,
        info: &DeviceTensorMemoryRequirementsARM,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self.0.GetDeviceTensorMemoryRequirementsARM(
                device.abi(), 
                info.abi(), 
                memory_requirements.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetTensorMemoryRequirementsARM(VkDevice device, VkTensorMemoryRequirementsInfoARM const* pInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    pub unsafe fn get_tensor_memory_requirements(
        &self,
        device: vk::Device,
        info: &TensorMemoryRequirementsInfoARM,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self.0.GetTensorMemoryRequirementsARM(
                device.abi(), 
                info.abi(), 
                memory_requirements.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkGetTensorOpaqueCaptureDescriptorDataARM(VkDevice device, VkTensorCaptureDescriptorDataInfoARM const* pInfo, void* pData)
    /// ```
    pub unsafe fn get_tensor_opaque_capture_descriptor_data(
        &self,
        device: vk::Device,
        info: &TensorCaptureDescriptorDataInfoARM,
        data: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetTensorOpaqueCaptureDescriptorDataARM(
                device.abi(), 
                info.abi(), 
                data.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkGetTensorViewOpaqueCaptureDescriptorDataARM(VkDevice device, VkTensorViewCaptureDescriptorDataInfoARM const* pInfo, void* pData)
    /// ```
    pub unsafe fn get_tensor_view_opaque_capture_descriptor_data(
        &self,
        device: vk::Device,
        info: &TensorViewCaptureDescriptorDataInfoARM,
        data: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetTensorViewOpaqueCaptureDescriptorDataARM(
                device.abi(), 
                info.abi(), 
                data.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::arm::tensors {
    type Commands = Device;
}

/// Device object
pub trait ArmTensorsDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkBindTensorMemoryARM(VkDevice device, uint32_t bindInfoCount, VkBindTensorMemoryInfoARM const* pBindInfos)
    /// ```
    unsafe fn bind_tensor_memory(
        &self,
        bind_infos: &[BindTensorMemoryInfoARM],
    ) -> crate::Result<()> {
        unsafe {
            self.commands().bind_tensor_memory(
                self.raw(),
                bind_infos,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateTensorARM(VkDevice device, VkTensorCreateInfoARM const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkTensorARM* pTensor)
    /// ```
    unsafe fn create_tensor(
        &self,
        create_info: &TensorCreateInfoARM,
    ) -> crate::Result<vk::TensorARM> {
        unsafe {
            self.commands().create_tensor(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateTensorViewARM(VkDevice device, VkTensorViewCreateInfoARM const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkTensorViewARM* pView)
    /// ```
    unsafe fn create_tensor_view(
        &self,
        create_info: &TensorViewCreateInfoARM,
    ) -> crate::Result<vk::TensorViewARM> {
        unsafe {
            self.commands().create_tensor_view(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// void vkDestroyTensorARM(VkDevice device, VkTensorARM tensor, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_tensor(
        &self,
        tensor: Option<vk::TensorARM>,
    ) -> () {
        unsafe {
            self.commands().destroy_tensor(
                self.raw(),
                tensor,
            )
        }
    }
    /// ```c
    /// void vkDestroyTensorViewARM(VkDevice device, VkTensorViewARM tensorView, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_tensor_view(
        &self,
        tensor_view: Option<vk::TensorViewARM>,
    ) -> () {
        unsafe {
            self.commands().destroy_tensor_view(
                self.raw(),
                tensor_view,
            )
        }
    }
    /// ```c
    /// void vkGetDeviceTensorMemoryRequirementsARM(VkDevice device, VkDeviceTensorMemoryRequirementsARM const* pInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    unsafe fn get_device_tensor_memory_requirements(
        &self,
        info: &DeviceTensorMemoryRequirementsARM,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self.commands().get_device_tensor_memory_requirements(
                self.raw(),
                info,
                memory_requirements,
            )
        }
    }
    /// ```c
    /// void vkGetTensorMemoryRequirementsARM(VkDevice device, VkTensorMemoryRequirementsInfoARM const* pInfo, VkMemoryRequirements2* pMemoryRequirements)
    /// ```
    unsafe fn get_tensor_memory_requirements(
        &self,
        info: &TensorMemoryRequirementsInfoARM,
        memory_requirements: &mut MemoryRequirements2,
    ) -> () {
        unsafe {
            self.commands().get_tensor_memory_requirements(
                self.raw(),
                info,
                memory_requirements,
            )
        }
    }
    /// ```c
    /// VkResult vkGetTensorOpaqueCaptureDescriptorDataARM(VkDevice device, VkTensorCaptureDescriptorDataInfoARM const* pInfo, void* pData)
    /// ```
    unsafe fn get_tensor_opaque_capture_descriptor_data(
        &self,
        info: &TensorCaptureDescriptorDataInfoARM,
        data: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_tensor_opaque_capture_descriptor_data(
                self.raw(),
                info,
                data,
            )
        }
    }
    /// ```c
    /// VkResult vkGetTensorViewOpaqueCaptureDescriptorDataARM(VkDevice device, VkTensorViewCaptureDescriptorDataInfoARM const* pInfo, void* pData)
    /// ```
    unsafe fn get_tensor_view_opaque_capture_descriptor_data(
        &self,
        info: &TensorViewCaptureDescriptorDataInfoARM,
        data: *mut void,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_tensor_view_opaque_capture_descriptor_data(
                self.raw(),
                info,
                data,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::arm::tensors {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::arm::tensors> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::arm::tensors {
        type Output = crate::hnd::Device<vk::extensions::arm::tensors>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::arm::tensors>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::arm::tensors> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::arm::tensors> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::arm::tensors> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::arm::tensors> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ArmTensorsDevice for crate::hnd::Device<vk::extensions::arm::tensors> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::arm::tensors, vk::Device> for crate::hnd::Device<vk::extensions::arm::tensors> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait ArmTensorsCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdCopyTensorARM(VkCommandBuffer commandBuffer, VkCopyTensorInfoARM const* pCopyTensorInfo)
    /// ```
    unsafe fn copy_tensor(
        &self,
        copy_tensor_info: &CopyTensorInfoARM,
    ) -> () {
        unsafe {
            self.commands().cmd_copy_tensor(
                self.raw(),
                copy_tensor_info,
            )
        }
    }
}

impl<O: crate::HndCtx<vk::extensions::arm::tensors, vk::Device>> crate::Owner<vk::TensorARM, vk::extensions::arm::tensors> for O {
    fn drop(&mut self, raw: vk::TensorARM) {
        unsafe {
            self.commands().destroy_tensor(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::arm::tensors, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::TensorARM, O, vk::extensions::arm::tensors>>
    where O: crate::HndCtx<vk::extensions::arm::tensors, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::TensorARM> for vk::extensions::arm::tensors {
    type Impl = _hs_TensorARM::TensorARM;
}


mod _hs_TensorARM {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct TensorARM(pub(crate) crate::handle::Hnd<vk::TensorARM, ::alloc::sync::Arc<super::Device>>);

    impl Clone for TensorARM {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::TensorARM, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::extensions::arm::tensors, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::TensorARM, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_tensor(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::TensorARM<vk::extensions::arm::tensors>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::arm::tensors, vk::Device>, raw: vk::TensorARM) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::arm::tensors, vk::Device>, raw: vk::TensorARM, dep: impl FnOnce() -> Dep) -> Self {
            Self(TensorARM(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::TensorARM<vk::extensions::arm::tensors> {
        pub fn raw(&self) -> vk::TensorARM { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::TensorARM<vk::extensions::arm::tensors> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("TensorARM({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::TensorARM<vk::extensions::arm::tensors> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::arm::tensors> for vk::TensorARM
        where Ctx: crate::HndCtx<vk::extensions::arm::tensors, vk::Device>,
    {
        type Output = crate::hnd::TensorARM<vk::extensions::arm::tensors>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::TensorARM::<vk::extensions::arm::tensors>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::extensions::arm::tensors, vk::Device>> crate::Owner<vk::TensorViewARM, vk::extensions::arm::tensors> for O {
    fn drop(&mut self, raw: vk::TensorViewARM) {
        unsafe {
            self.commands().destroy_tensor_view(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::arm::tensors, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::TensorViewARM, O, vk::extensions::arm::tensors>>
    where O: crate::HndCtx<vk::extensions::arm::tensors, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::TensorViewARM> for vk::extensions::arm::tensors {
    type Impl = _hs_TensorViewARM::TensorViewARM;
}


mod _hs_TensorViewARM {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct TensorViewARM(pub(crate) crate::handle::Hnd<vk::TensorViewARM, ::alloc::sync::Arc<super::Device>>);

    impl Clone for TensorViewARM {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::TensorViewARM, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::extensions::arm::tensors, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::TensorViewARM, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_tensor_view(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::TensorViewARM<vk::extensions::arm::tensors>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::arm::tensors, vk::Device>, raw: vk::TensorViewARM) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::arm::tensors, vk::Device>, raw: vk::TensorViewARM, dep: impl FnOnce() -> Dep) -> Self {
            Self(TensorViewARM(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::TensorViewARM<vk::extensions::arm::tensors> {
        pub fn raw(&self) -> vk::TensorViewARM { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::TensorViewARM<vk::extensions::arm::tensors> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("TensorViewARM({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::TensorViewARM<vk::extensions::arm::tensors> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::arm::tensors> for vk::TensorViewARM
        where Ctx: crate::HndCtx<vk::extensions::arm::tensors, vk::Device>,
    {
        type Output = crate::hnd::TensorViewARM<vk::extensions::arm::tensors>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::TensorViewARM::<vk::extensions::arm::tensors>::new_with(ctx, self, dep) }
        }
    }
}
