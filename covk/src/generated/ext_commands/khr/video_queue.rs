// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_video_queue` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::khr::video_queue::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::video_queue::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkGetPhysicalDeviceVideoCapabilitiesKHR(VkPhysicalDevice physicalDevice, VkVideoProfileInfoKHR const* pVideoProfile, VkVideoCapabilitiesKHR* pCapabilities)
    /// ```
    pub unsafe fn get_physical_device_video_capabilities(
        &self,
        physical_device: vk::PhysicalDevice,
        video_profile: &VideoProfileInfoKHR,
        capabilities: &mut VideoCapabilitiesKHR,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetPhysicalDeviceVideoCapabilitiesKHR(
                physical_device.abi(), 
                video_profile.abi(), 
                capabilities.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkGetPhysicalDeviceVideoFormatPropertiesKHR(VkPhysicalDevice physicalDevice, VkPhysicalDeviceVideoFormatInfoKHR const* pVideoFormatInfo, uint32_t* pVideoFormatPropertyCount, VkVideoFormatPropertiesKHR* pVideoFormatProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_physical_device_video_format_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        video_format_info: &PhysicalDeviceVideoFormatInfoKHR,
        video_format_properties: Option<&mut ::alloc::vec::Vec<VideoFormatPropertiesKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetPhysicalDeviceVideoFormatPropertiesKHR(
                physical_device.abi(), 
                video_format_info.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = video_format_properties {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetPhysicalDeviceVideoFormatPropertiesKHR(
                    physical_device.abi(), 
                    video_format_info.abi(), 
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

impl crate::CommandScope<vk::Instance> for vk::extensions::khr::video_queue {
    type Commands = Instance;
}

/// Instance object
pub trait KhrVideoQueueInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::khr::video_queue {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::khr::video_queue> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::khr::video_queue {
        type Output = crate::hnd::Instance<vk::extensions::khr::video_queue>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::khr::video_queue>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::khr::video_queue> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::khr::video_queue> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::khr::video_queue> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::khr::video_queue> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrVideoQueueInstance for crate::hnd::Instance<vk::extensions::khr::video_queue> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::video_queue, vk::Instance> for crate::hnd::Instance<vk::extensions::khr::video_queue> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait KhrVideoQueuePhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkGetPhysicalDeviceVideoCapabilitiesKHR(VkPhysicalDevice physicalDevice, VkVideoProfileInfoKHR const* pVideoProfile, VkVideoCapabilitiesKHR* pCapabilities)
    /// ```
    unsafe fn get_video_capabilities(
        &self,
        video_profile: &VideoProfileInfoKHR,
        capabilities: &mut VideoCapabilitiesKHR,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_physical_device_video_capabilities(
                self.raw(),
                video_profile,
                capabilities,
            )
        }
    }
    /// ```c
    /// VkResult vkGetPhysicalDeviceVideoFormatPropertiesKHR(VkPhysicalDevice physicalDevice, VkPhysicalDeviceVideoFormatInfoKHR const* pVideoFormatInfo, uint32_t* pVideoFormatPropertyCount, VkVideoFormatPropertiesKHR* pVideoFormatProperties)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_video_format_properties(
        &self,
        video_format_info: &PhysicalDeviceVideoFormatInfoKHR,
        video_format_properties: Option<&mut ::alloc::vec::Vec<VideoFormatPropertiesKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_physical_device_video_format_properties(
                self.raw(),
                video_format_info,
                video_format_properties,
            )
        }
    }
}

/// `VK_KHR_video_queue` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::video_queue::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::video_queue::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkBindVideoSessionMemoryKHR(VkDevice device, VkVideoSessionKHR videoSession, uint32_t bindSessionMemoryInfoCount, VkBindVideoSessionMemoryInfoKHR const* pBindSessionMemoryInfos)
    /// ```
    pub unsafe fn bind_video_session_memory(
        &self,
        device: vk::Device,
        video_session: vk::VideoSessionKHR,
        bind_session_memory_infos: &[BindVideoSessionMemoryInfoKHR],
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.BindVideoSessionMemoryKHR(
                device.abi(), 
                video_session.abi(), 
                bind_session_memory_infos.len() as _, 
                bind_session_memory_infos.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// void vkCmdBeginVideoCodingKHR(VkCommandBuffer commandBuffer, VkVideoBeginCodingInfoKHR const* pBeginInfo)
    /// ```
    pub unsafe fn cmd_begin_video_coding(
        &self,
        command_buffer: vk::CommandBuffer,
        begin_info: &VideoBeginCodingInfoKHR,
    ) -> () {
        unsafe {
            self.0.CmdBeginVideoCodingKHR(
                command_buffer.abi(), 
                begin_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdControlVideoCodingKHR(VkCommandBuffer commandBuffer, VkVideoCodingControlInfoKHR const* pCodingControlInfo)
    /// ```
    pub unsafe fn cmd_control_video_coding(
        &self,
        command_buffer: vk::CommandBuffer,
        coding_control_info: &VideoCodingControlInfoKHR,
    ) -> () {
        unsafe {
            self.0.CmdControlVideoCodingKHR(
                command_buffer.abi(), 
                coding_control_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdEndVideoCodingKHR(VkCommandBuffer commandBuffer, VkVideoEndCodingInfoKHR const* pEndCodingInfo)
    /// ```
    pub unsafe fn cmd_end_video_coding(
        &self,
        command_buffer: vk::CommandBuffer,
        end_coding_info: &VideoEndCodingInfoKHR,
    ) -> () {
        unsafe {
            self.0.CmdEndVideoCodingKHR(
                command_buffer.abi(), 
                end_coding_info.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkCreateVideoSessionKHR(VkDevice device, VkVideoSessionCreateInfoKHR const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkVideoSessionKHR* pVideoSession)
    /// ```
    pub unsafe fn create_video_session(
        &self,
        device: vk::Device,
        create_info: &VideoSessionCreateInfoKHR,
    ) -> crate::Result<vk::VideoSessionKHR> {
        unsafe {
            let mut _v: Option<vk::VideoSessionKHR> = Default::default();
            let _r = self.0.CreateVideoSessionKHR(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// VkResult vkCreateVideoSessionParametersKHR(VkDevice device, VkVideoSessionParametersCreateInfoKHR const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkVideoSessionParametersKHR* pVideoSessionParameters)
    /// ```
    pub unsafe fn create_video_session_parameters(
        &self,
        device: vk::Device,
        create_info: &VideoSessionParametersCreateInfoKHR,
    ) -> crate::Result<vk::VideoSessionParametersKHR> {
        unsafe {
            let mut _v: Option<vk::VideoSessionParametersKHR> = Default::default();
            let _r = self.0.CreateVideoSessionParametersKHR(
                device.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDestroyVideoSessionKHR(VkDevice device, VkVideoSessionKHR videoSession, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_video_session(
        &self,
        device: vk::Device,
        video_session: Option<vk::VideoSessionKHR>,
    ) -> () {
        unsafe {
            self.0.DestroyVideoSessionKHR(
                device.abi(), 
                video_session.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkDestroyVideoSessionParametersKHR(VkDevice device, VkVideoSessionParametersKHR videoSessionParameters, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_video_session_parameters(
        &self,
        device: vk::Device,
        video_session_parameters: Option<vk::VideoSessionParametersKHR>,
    ) -> () {
        unsafe {
            self.0.DestroyVideoSessionParametersKHR(
                device.abi(), 
                video_session_parameters.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// VkResult vkGetVideoSessionMemoryRequirementsKHR(VkDevice device, VkVideoSessionKHR videoSession, uint32_t* pMemoryRequirementsCount, VkVideoSessionMemoryRequirementsKHR* pMemoryRequirements)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_video_session_memory_requirements(
        &self,
        device: vk::Device,
        video_session: vk::VideoSessionKHR,
        memory_requirements: Option<&mut ::alloc::vec::Vec<VideoSessionMemoryRequirementsKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetVideoSessionMemoryRequirementsKHR(
                device.abi(), 
                video_session.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = memory_requirements {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetVideoSessionMemoryRequirementsKHR(
                    device.abi(), 
                    video_session.abi(), 
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
    /// VkResult vkUpdateVideoSessionParametersKHR(VkDevice device, VkVideoSessionParametersKHR videoSessionParameters, VkVideoSessionParametersUpdateInfoKHR const* pUpdateInfo)
    /// ```
    pub unsafe fn update_video_session_parameters(
        &self,
        device: vk::Device,
        video_session_parameters: vk::VideoSessionParametersKHR,
        update_info: &VideoSessionParametersUpdateInfoKHR,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.UpdateVideoSessionParametersKHR(
                device.abi(), 
                video_session_parameters.abi(), 
                update_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::video_queue {
    type Commands = Device;
}

/// Device object
pub trait KhrVideoQueueDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkBindVideoSessionMemoryKHR(VkDevice device, VkVideoSessionKHR videoSession, uint32_t bindSessionMemoryInfoCount, VkBindVideoSessionMemoryInfoKHR const* pBindSessionMemoryInfos)
    /// ```
    unsafe fn bind_video_session_memory(
        &self,
        video_session: vk::VideoSessionKHR,
        bind_session_memory_infos: &[BindVideoSessionMemoryInfoKHR],
    ) -> crate::Result<()> {
        unsafe {
            self.commands().bind_video_session_memory(
                self.raw(),
                video_session,
                bind_session_memory_infos,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateVideoSessionKHR(VkDevice device, VkVideoSessionCreateInfoKHR const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkVideoSessionKHR* pVideoSession)
    /// ```
    unsafe fn create_video_session(
        &self,
        create_info: &VideoSessionCreateInfoKHR,
    ) -> crate::Result<vk::VideoSessionKHR> {
        unsafe {
            self.commands().create_video_session(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// VkResult vkCreateVideoSessionParametersKHR(VkDevice device, VkVideoSessionParametersCreateInfoKHR const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkVideoSessionParametersKHR* pVideoSessionParameters)
    /// ```
    unsafe fn create_video_session_parameters(
        &self,
        create_info: &VideoSessionParametersCreateInfoKHR,
    ) -> crate::Result<vk::VideoSessionParametersKHR> {
        unsafe {
            self.commands().create_video_session_parameters(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// void vkDestroyVideoSessionKHR(VkDevice device, VkVideoSessionKHR videoSession, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_video_session(
        &self,
        video_session: Option<vk::VideoSessionKHR>,
    ) -> () {
        unsafe {
            self.commands().destroy_video_session(
                self.raw(),
                video_session,
            )
        }
    }
    /// ```c
    /// void vkDestroyVideoSessionParametersKHR(VkDevice device, VkVideoSessionParametersKHR videoSessionParameters, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_video_session_parameters(
        &self,
        video_session_parameters: Option<vk::VideoSessionParametersKHR>,
    ) -> () {
        unsafe {
            self.commands().destroy_video_session_parameters(
                self.raw(),
                video_session_parameters,
            )
        }
    }
    /// ```c
    /// VkResult vkGetVideoSessionMemoryRequirementsKHR(VkDevice device, VkVideoSessionKHR videoSession, uint32_t* pMemoryRequirementsCount, VkVideoSessionMemoryRequirementsKHR* pMemoryRequirements)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_video_session_memory_requirements(
        &self,
        video_session: vk::VideoSessionKHR,
        memory_requirements: Option<&mut ::alloc::vec::Vec<VideoSessionMemoryRequirementsKHR>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_video_session_memory_requirements(
                self.raw(),
                video_session,
                memory_requirements,
            )
        }
    }
    /// ```c
    /// VkResult vkUpdateVideoSessionParametersKHR(VkDevice device, VkVideoSessionParametersKHR videoSessionParameters, VkVideoSessionParametersUpdateInfoKHR const* pUpdateInfo)
    /// ```
    unsafe fn update_video_session_parameters(
        &self,
        video_session_parameters: vk::VideoSessionParametersKHR,
        update_info: &VideoSessionParametersUpdateInfoKHR,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().update_video_session_parameters(
                self.raw(),
                video_session_parameters,
                update_info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::video_queue {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::video_queue> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::video_queue {
        type Output = crate::hnd::Device<vk::extensions::khr::video_queue>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::video_queue>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::video_queue> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::video_queue> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::video_queue> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::video_queue> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrVideoQueueDevice for crate::hnd::Device<vk::extensions::khr::video_queue> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::video_queue, vk::Device> for crate::hnd::Device<vk::extensions::khr::video_queue> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait KhrVideoQueueCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdBeginVideoCodingKHR(VkCommandBuffer commandBuffer, VkVideoBeginCodingInfoKHR const* pBeginInfo)
    /// ```
    unsafe fn begin_video_coding(
        &self,
        begin_info: &VideoBeginCodingInfoKHR,
    ) -> () {
        unsafe {
            self.commands().cmd_begin_video_coding(
                self.raw(),
                begin_info,
            )
        }
    }
    /// ```c
    /// void vkCmdControlVideoCodingKHR(VkCommandBuffer commandBuffer, VkVideoCodingControlInfoKHR const* pCodingControlInfo)
    /// ```
    unsafe fn control_video_coding(
        &self,
        coding_control_info: &VideoCodingControlInfoKHR,
    ) -> () {
        unsafe {
            self.commands().cmd_control_video_coding(
                self.raw(),
                coding_control_info,
            )
        }
    }
    /// ```c
    /// void vkCmdEndVideoCodingKHR(VkCommandBuffer commandBuffer, VkVideoEndCodingInfoKHR const* pEndCodingInfo)
    /// ```
    unsafe fn end_video_coding(
        &self,
        end_coding_info: &VideoEndCodingInfoKHR,
    ) -> () {
        unsafe {
            self.commands().cmd_end_video_coding(
                self.raw(),
                end_coding_info,
            )
        }
    }
}

impl<O: crate::HndCtx<vk::extensions::khr::video_queue, vk::Device>> crate::Owner<vk::VideoSessionKHR, vk::extensions::khr::video_queue> for O {
    fn drop(&mut self, raw: vk::VideoSessionKHR) {
        unsafe {
            self.commands().destroy_video_session(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::khr::video_queue, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::VideoSessionKHR, O, vk::extensions::khr::video_queue>>
    where O: crate::HndCtx<vk::extensions::khr::video_queue, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::VideoSessionKHR> for vk::extensions::khr::video_queue {
    type Impl = _hs_VideoSessionKHR::VideoSessionKHR;
}


mod _hs_VideoSessionKHR {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct VideoSessionKHR(pub(crate) crate::handle::Hnd<vk::VideoSessionKHR, ::alloc::sync::Arc<super::Device>>);

    impl Clone for VideoSessionKHR {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::VideoSessionKHR, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::extensions::khr::video_queue, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::VideoSessionKHR, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_video_session(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::VideoSessionKHR<vk::extensions::khr::video_queue>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::khr::video_queue, vk::Device>, raw: vk::VideoSessionKHR) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::khr::video_queue, vk::Device>, raw: vk::VideoSessionKHR, dep: impl FnOnce() -> Dep) -> Self {
            Self(VideoSessionKHR(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::VideoSessionKHR<vk::extensions::khr::video_queue> {
        pub fn raw(&self) -> vk::VideoSessionKHR { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::VideoSessionKHR<vk::extensions::khr::video_queue> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("VideoSessionKHR({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::VideoSessionKHR<vk::extensions::khr::video_queue> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::khr::video_queue> for vk::VideoSessionKHR
        where Ctx: crate::HndCtx<vk::extensions::khr::video_queue, vk::Device>,
    {
        type Output = crate::hnd::VideoSessionKHR<vk::extensions::khr::video_queue>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::VideoSessionKHR::<vk::extensions::khr::video_queue>::new_with(ctx, self, dep) }
        }
    }
}

impl<O: crate::HndCtx<vk::extensions::khr::video_queue, vk::Device>> crate::Owner<vk::VideoSessionParametersKHR, vk::extensions::khr::video_queue> for O {
    fn drop(&mut self, raw: vk::VideoSessionParametersKHR) {
        unsafe {
            self.commands().destroy_video_session_parameters(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::khr::video_queue, vk::Device> for ::alloc::sync::Arc<crate::Unique<vk::VideoSessionParametersKHR, O, vk::extensions::khr::video_queue>>
    where O: crate::HndCtx<vk::extensions::khr::video_queue, vk::Device> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Device { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Device> { self.owner.commands() }
}

impl crate::HndScope<vk::VideoSessionParametersKHR> for vk::extensions::khr::video_queue {
    type Impl = _hs_VideoSessionParametersKHR::VideoSessionParametersKHR;
}


mod _hs_VideoSessionParametersKHR {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct VideoSessionParametersKHR(pub(crate) crate::handle::Hnd<vk::VideoSessionParametersKHR, ::alloc::sync::Arc<super::Device>>);

    impl Clone for VideoSessionParametersKHR {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::VideoSessionParametersKHR, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::extensions::khr::video_queue, vk::Device>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::VideoSessionParametersKHR, ::alloc::sync::Arc<super::Device>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_video_session_parameters(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::VideoSessionParametersKHR<vk::extensions::khr::video_queue>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::khr::video_queue, vk::Device>, raw: vk::VideoSessionParametersKHR) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::khr::video_queue, vk::Device>, raw: vk::VideoSessionParametersKHR, dep: impl FnOnce() -> Dep) -> Self {
            Self(VideoSessionParametersKHR(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::VideoSessionParametersKHR<vk::extensions::khr::video_queue> {
        pub fn raw(&self) -> vk::VideoSessionParametersKHR { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::VideoSessionParametersKHR<vk::extensions::khr::video_queue> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("VideoSessionParametersKHR({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::VideoSessionParametersKHR<vk::extensions::khr::video_queue> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::khr::video_queue> for vk::VideoSessionParametersKHR
        where Ctx: crate::HndCtx<vk::extensions::khr::video_queue, vk::Device>,
    {
        type Output = crate::hnd::VideoSessionParametersKHR<vk::extensions::khr::video_queue>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::VideoSessionParametersKHR::<vk::extensions::khr::video_queue>::new_with(ctx, self, dep) }
        }
    }
}
