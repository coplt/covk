// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_ARM_shader_instrumentation` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::arm::shader_instrumentation::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::arm::shader_instrumentation::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM(VkPhysicalDevice physicalDevice, uint32_t* pDescriptionCount, VkShaderInstrumentationMetricDescriptionARM* pDescriptions)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn enumerate_physical_device_shader_instrumentation_metrics(
        &self,
        physical_device: vk::PhysicalDevice,
        descriptions: Option<&mut ::alloc::vec::Vec<ShaderInstrumentationMetricDescriptionARM>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.EnumeratePhysicalDeviceShaderInstrumentationMetricsARM(
                physical_device.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = descriptions {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.EnumeratePhysicalDeviceShaderInstrumentationMetricsARM(
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

impl crate::CommandScope<vk::Instance> for vk::extensions::arm::shader_instrumentation {
    type Commands = Instance;
}

/// Instance object
pub trait ArmShaderInstrumentationInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::arm::shader_instrumentation {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::arm::shader_instrumentation> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::arm::shader_instrumentation {
        type Output = crate::hnd::Instance<vk::extensions::arm::shader_instrumentation>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::arm::shader_instrumentation>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::arm::shader_instrumentation> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::arm::shader_instrumentation> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::arm::shader_instrumentation> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::arm::shader_instrumentation> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ArmShaderInstrumentationInstance for crate::hnd::Instance<vk::extensions::arm::shader_instrumentation> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::arm::shader_instrumentation, vk::Instance> for crate::hnd::Instance<vk::extensions::arm::shader_instrumentation> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait ArmShaderInstrumentationPhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM(VkPhysicalDevice physicalDevice, uint32_t* pDescriptionCount, VkShaderInstrumentationMetricDescriptionARM* pDescriptions)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn enumerate_physical_device_shader_instrumentation_metrics(
        &self,
        descriptions: Option<&mut ::alloc::vec::Vec<ShaderInstrumentationMetricDescriptionARM>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().enumerate_physical_device_shader_instrumentation_metrics(
                self.raw(),
                descriptions,
            )
        }
    }
}

/// `VK_ARM_shader_instrumentation` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::arm::shader_instrumentation::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::arm::shader_instrumentation::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkClearShaderInstrumentationMetricsARM(VkDevice device, VkShaderInstrumentationARM instrumentation)
    /// ```
    pub unsafe fn clear_shader_instrumentation_metrics(
        &self,
        device: vk::Device,
        instrumentation: vk::ShaderInstrumentationARM,
    ) -> () {
        unsafe {
            self.0.ClearShaderInstrumentationMetricsARM(
                device.abi(), 
                instrumentation.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdBeginShaderInstrumentationARM(VkCommandBuffer commandBuffer, VkShaderInstrumentationARM instrumentation)
    /// ```
    pub unsafe fn cmd_begin_shader_instrumentation(
        &self,
        command_buffer: vk::CommandBuffer,
        instrumentation: vk::ShaderInstrumentationARM,
    ) -> () {
        unsafe {
            self.0.CmdBeginShaderInstrumentationARM(
                command_buffer.abi(), 
                instrumentation.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdEndShaderInstrumentationARM(VkCommandBuffer commandBuffer)
    /// ```
    pub unsafe fn cmd_end_shader_instrumentation(
        &self,
        command_buffer: vk::CommandBuffer,
    ) -> () {
        unsafe {
            self.0.CmdEndShaderInstrumentationARM(
                command_buffer.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkCreateShaderInstrumentationARM(VkDevice device, VkShaderInstrumentationCreateInfoARM const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkShaderInstrumentationARM* pInstrumentation)
    /// ```
    pub unsafe fn create_shader_instrumentation(
        &self,
        device: vk::Device,
        create_info: &ShaderInstrumentationCreateInfoARM,
    ) -> crate::Result<vk::ShaderInstrumentationARM> {
        unsafe {
            let mut _v: Option<vk::ShaderInstrumentationARM> = Default::default();
            let _r = self.0.CreateShaderInstrumentationARM(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDestroyShaderInstrumentationARM(VkDevice device, VkShaderInstrumentationARM instrumentation, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_shader_instrumentation(
        &self,
        device: vk::Device,
        instrumentation: Option<vk::ShaderInstrumentationARM>,
    ) -> () {
        unsafe {
            self.0.DestroyShaderInstrumentationARM(
                device.abi(), 
                instrumentation.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// VkResult vkGetShaderInstrumentationValuesARM(VkDevice device, VkShaderInstrumentationARM instrumentation, uint32_t* pMetricBlockCount, void* pMetricValues, VkShaderInstrumentationValuesFlagsARM flags)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_shader_instrumentation_values(
        &self,
        device: vk::Device,
        instrumentation: vk::ShaderInstrumentationARM,
        metric_block_count: *mut uint32_t,
        metric_values: *mut void,
        flags: ShaderInstrumentationValuesFlagsARM,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.GetShaderInstrumentationValuesARM(
                device.abi(), 
                instrumentation.abi(), 
                metric_block_count.abi(), 
                metric_values.abi(), 
                flags.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::arm::shader_instrumentation {
    type Commands = Device;
}

/// Device object
pub trait ArmShaderInstrumentationDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkClearShaderInstrumentationMetricsARM(VkDevice device, VkShaderInstrumentationARM instrumentation)
    /// ```
    unsafe fn clear_shader_instrumentation_metrics(
        &self,
        instrumentation: vk::ShaderInstrumentationARM,
    ) -> () {
        unsafe {
            self.commands().clear_shader_instrumentation_metrics(
                self.raw(),
                instrumentation,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateShaderInstrumentationARM(VkDevice device, VkShaderInstrumentationCreateInfoARM const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkShaderInstrumentationARM* pInstrumentation)
    /// ```
    unsafe fn create_shader_instrumentation(
        &self,
        create_info: &ShaderInstrumentationCreateInfoARM,
    ) -> crate::Result<vk::ShaderInstrumentationARM> {
        unsafe {
            self.commands().create_shader_instrumentation(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// void vkDestroyShaderInstrumentationARM(VkDevice device, VkShaderInstrumentationARM instrumentation, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_shader_instrumentation(
        &self,
        instrumentation: Option<vk::ShaderInstrumentationARM>,
    ) -> () {
        unsafe {
            self.commands().destroy_shader_instrumentation(
                self.raw(),
                instrumentation,
            )
        }
    }
    /// ```c
    /// VkResult vkGetShaderInstrumentationValuesARM(VkDevice device, VkShaderInstrumentationARM instrumentation, uint32_t* pMetricBlockCount, void* pMetricValues, VkShaderInstrumentationValuesFlagsARM flags)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_shader_instrumentation_values(
        &self,
        instrumentation: vk::ShaderInstrumentationARM,
        metric_block_count: *mut uint32_t,
        metric_values: *mut void,
        flags: ShaderInstrumentationValuesFlagsARM,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().get_shader_instrumentation_values(
                self.raw(),
                instrumentation,
                metric_block_count,
                metric_values,
                flags,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::arm::shader_instrumentation {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::arm::shader_instrumentation> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::arm::shader_instrumentation {
        type Output = crate::hnd::Device<vk::extensions::arm::shader_instrumentation>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::arm::shader_instrumentation>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::arm::shader_instrumentation> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::arm::shader_instrumentation> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::arm::shader_instrumentation> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::arm::shader_instrumentation> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ArmShaderInstrumentationDevice for crate::hnd::Device<vk::extensions::arm::shader_instrumentation> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::arm::shader_instrumentation, vk::Device> for crate::hnd::Device<vk::extensions::arm::shader_instrumentation> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait ArmShaderInstrumentationCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdBeginShaderInstrumentationARM(VkCommandBuffer commandBuffer, VkShaderInstrumentationARM instrumentation)
    /// ```
    unsafe fn begin_shader_instrumentation(
        &self,
        instrumentation: vk::ShaderInstrumentationARM,
    ) -> () {
        unsafe {
            self.commands().cmd_begin_shader_instrumentation(
                self.raw(),
                instrumentation,
            )
        }
    }
    /// ```c
    /// void vkCmdEndShaderInstrumentationARM(VkCommandBuffer commandBuffer)
    /// ```
    unsafe fn end_shader_instrumentation(
        &self,
    ) -> () {
        unsafe {
            self.commands().cmd_end_shader_instrumentation(
                self.raw(),
            )
        }
    }
}

impl<O: crate::HndCtx<vk::extensions::arm::shader_instrumentation, vk::Device>> crate::Owner<vk::ShaderInstrumentationARM, vk::extensions::arm::shader_instrumentation> for O {
    fn drop(&mut self, raw: vk::ShaderInstrumentationARM) {
        unsafe {
            self.commands().destroy_shader_instrumentation(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::arm::shader_instrumentation, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::ShaderInstrumentationARM, O, vk::extensions::arm::shader_instrumentation>>
    where O: crate::HndCtx<vk::extensions::arm::shader_instrumentation, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::ShaderInstrumentationARM> for vk::extensions::arm::shader_instrumentation {
    type Impl = _hs_ShaderInstrumentationARM::ShaderInstrumentationARM;
}


mod _hs_ShaderInstrumentationARM {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct ShaderInstrumentationARM(pub(crate) crate::handle::Hnd<vk::ShaderInstrumentationARM, ::alloc::sync::Arc<super::Device>>);

    impl Clone for ShaderInstrumentationARM {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::ShaderInstrumentationARM, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::extensions::arm::shader_instrumentation, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::ShaderInstrumentationARM, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_shader_instrumentation(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::ShaderInstrumentationARM<vk::extensions::arm::shader_instrumentation>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::arm::shader_instrumentation, vk::Device>, raw: vk::ShaderInstrumentationARM) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::arm::shader_instrumentation, vk::Device>, raw: vk::ShaderInstrumentationARM, dep: impl FnOnce() -> Dep) -> Self {
            Self(ShaderInstrumentationARM(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::ShaderInstrumentationARM<vk::extensions::arm::shader_instrumentation> {
        pub fn raw(&self) -> vk::ShaderInstrumentationARM { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::ShaderInstrumentationARM<vk::extensions::arm::shader_instrumentation> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("ShaderInstrumentationARM({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::ShaderInstrumentationARM<vk::extensions::arm::shader_instrumentation> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::arm::shader_instrumentation> for vk::ShaderInstrumentationARM
        where Ctx: crate::HndCtx<vk::extensions::arm::shader_instrumentation, vk::Device>,
    {
        type Output = crate::hnd::ShaderInstrumentationARM<vk::extensions::arm::shader_instrumentation>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::ShaderInstrumentationARM::<vk::extensions::arm::shader_instrumentation>::new_with(ctx, self, dep) }
        }
    }
}
