// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_EXT_debug_utils` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::ext::debug_utils::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::debug_utils::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkCreateDebugUtilsMessengerEXT(VkInstance instance, VkDebugUtilsMessengerCreateInfoEXT const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkDebugUtilsMessengerEXT* pMessenger)
    /// ```
    pub unsafe fn create_debug_utils_messenger(
        &self,
        instance: vk::Instance,
        create_info: &DebugUtilsMessengerCreateInfoEXT,
    ) -> crate::Result<vk::DebugUtilsMessengerEXT> {
        unsafe {
            let mut _v: Option<vk::DebugUtilsMessengerEXT> = Default::default();
            let _r = self.0.CreateDebugUtilsMessengerEXT(
                instance.abi(), 
                create_info.abi(), 
                Default::default(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| _v)
        }
    }
    /// ```c
    /// void vkDestroyDebugUtilsMessengerEXT(VkInstance instance, VkDebugUtilsMessengerEXT messenger, VkAllocationCallbacks const* pAllocator)
    /// ```
    pub unsafe fn destroy_debug_utils_messenger(
        &self,
        instance: vk::Instance,
        messenger: Option<vk::DebugUtilsMessengerEXT>,
    ) -> () {
        unsafe {
            self.0.DestroyDebugUtilsMessengerEXT(
                instance.abi(), 
                messenger.abi(), 
                Default::default(), 
            );
        }
    }
    /// ```c
    /// void vkSubmitDebugUtilsMessageEXT(VkInstance instance, VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageTypes, VkDebugUtilsMessengerCallbackDataEXT const* pCallbackData)
    /// ```
    pub unsafe fn submit_debug_utils_message(
        &self,
        instance: vk::Instance,
        message_severity: DebugUtilsMessageSeverityFlagsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        callback_data: &DebugUtilsMessengerCallbackDataEXT,
    ) -> () {
        unsafe {
            self.0.SubmitDebugUtilsMessageEXT(
                instance.abi(), 
                message_severity.abi(), 
                message_types.abi(), 
                callback_data.abi(), 
            );
        }
    }
}

impl crate::CommandScope<vk::Instance> for vk::extensions::ext::debug_utils {
    type Commands = Instance;
}

/// Instance object
pub trait ExtDebugUtilsInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkCreateDebugUtilsMessengerEXT(VkInstance instance, VkDebugUtilsMessengerCreateInfoEXT const* pCreateInfo, VkAllocationCallbacks const* pAllocator, VkDebugUtilsMessengerEXT* pMessenger)
    /// ```
    unsafe fn create_debug_utils_messenger(
        &self,
        create_info: &DebugUtilsMessengerCreateInfoEXT,
    ) -> crate::Result<vk::DebugUtilsMessengerEXT> {
        unsafe {
            self.commands().create_debug_utils_messenger(
                self.raw(),
                create_info,
            )
        }
    }
    /// ```c
    /// void vkDestroyDebugUtilsMessengerEXT(VkInstance instance, VkDebugUtilsMessengerEXT messenger, VkAllocationCallbacks const* pAllocator)
    /// ```
    unsafe fn destroy_debug_utils_messenger(
        &self,
        messenger: Option<vk::DebugUtilsMessengerEXT>,
    ) -> () {
        unsafe {
            self.commands().destroy_debug_utils_messenger(
                self.raw(),
                messenger,
            )
        }
    }
    /// ```c
    /// void vkSubmitDebugUtilsMessageEXT(VkInstance instance, VkDebugUtilsMessageSeverityFlagBitsEXT messageSeverity, VkDebugUtilsMessageTypeFlagsEXT messageTypes, VkDebugUtilsMessengerCallbackDataEXT const* pCallbackData)
    /// ```
    unsafe fn submit_debug_utils_message(
        &self,
        message_severity: DebugUtilsMessageSeverityFlagsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        callback_data: &DebugUtilsMessengerCallbackDataEXT,
    ) -> () {
        unsafe {
            self.commands().submit_debug_utils_message(
                self.raw(),
                message_severity,
                message_types,
                callback_data,
            )
        }
    }
}

impl crate::HndScope<vk::Instance> for vk::extensions::ext::debug_utils {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::ext::debug_utils> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::ext::debug_utils {
        type Output = crate::hnd::Instance<vk::extensions::ext::debug_utils>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::ext::debug_utils>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::ext::debug_utils> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::ext::debug_utils> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::ext::debug_utils> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::ext::debug_utils> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtDebugUtilsInstance for crate::hnd::Instance<vk::extensions::ext::debug_utils> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::debug_utils, vk::Instance> for crate::hnd::Instance<vk::extensions::ext::debug_utils> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

impl<O: crate::HndCtx<vk::extensions::ext::debug_utils, vk::Instance>> crate::Owner<vk::DebugUtilsMessengerEXT, vk::extensions::ext::debug_utils> for O {
    fn drop(&mut self, raw: vk::DebugUtilsMessengerEXT) {
        unsafe {
            self.commands().destroy_debug_utils_messenger(self.raw(), Some(raw))
        }
    }
}
impl<O> crate::HndCtx<vk::extensions::ext::debug_utils, vk::Instance> for ::alloc::sync::Arc<crate::Unique<vk::DebugUtilsMessengerEXT, O, vk::extensions::ext::debug_utils>>
    where O: crate::HndCtx<vk::extensions::ext::debug_utils, vk::Instance> + Send + Sync + 'static,
{
    type Ctx = Self;
    fn ctx(&self) -> Self::Ctx { self.clone() }
    fn raw(&self) -> vk::Instance { self.owner.raw() }
    fn commands(&self) -> &::alloc::sync::Arc<Instance> { self.owner.commands() }
}

impl crate::HndScope<vk::DebugUtilsMessengerEXT> for vk::extensions::ext::debug_utils {
    type Impl = _hs_DebugUtilsMessengerEXT::DebugUtilsMessengerEXT;
}


mod _hs_DebugUtilsMessengerEXT {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct DebugUtilsMessengerEXT(pub(crate) crate::handle::Hnd<vk::DebugUtilsMessengerEXT, ::alloc::sync::Arc<super::Instance>>);

    impl Clone for DebugUtilsMessengerEXT {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }

    #[derive(Debug, Clone, Copy)]
    struct Inst;
    impl<Ctx, Dep> crate::handle::HndInst<vk::DebugUtilsMessengerEXT, ::alloc::sync::Arc<super::Instance>, (Ctx, Dep)> for Inst
        where Ctx: crate::HndCtx<vk::extensions::ext::debug_utils, vk::Instance>,
    {
        fn drop(this: &mut crate::handle::HndData<vk::DebugUtilsMessengerEXT, ::alloc::sync::Arc<super::Instance>, (Ctx, Dep)>) {
            unsafe {
                this.scope.destroy_debug_utils_messenger(this.dep.0.raw(), Some(this.raw))
            }
        }
    }

    impl crate::hnd::DebugUtilsMessengerEXT<vk::extensions::ext::debug_utils>
    {
        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::extensions::ext::debug_utils, vk::Instance>, raw: vk::DebugUtilsMessengerEXT) -> Self {
            unsafe { Self::new_with(ctx, raw, || ()) }
        }
        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::extensions::ext::debug_utils, vk::Instance>, raw: vk::DebugUtilsMessengerEXT, dep: impl FnOnce() -> Dep) -> Self {
            Self(DebugUtilsMessengerEXT(crate::handle::Hnd::new(
                ctx.commands().clone(),
                raw,
                (ctx.ctx(), dep()),
                Inst,
            )))
        }
    }

    impl crate::hnd::DebugUtilsMessengerEXT<vk::extensions::ext::debug_utils> {
        pub fn raw(&self) -> vk::DebugUtilsMessengerEXT { self.0.0.raw }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0.scope }
    }

    impl ::core::fmt::Debug for crate::hnd::DebugUtilsMessengerEXT<vk::extensions::ext::debug_utils> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("DebugUtilsMessengerEXT({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::DebugUtilsMessengerEXT<vk::extensions::ext::debug_utils> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl<Ctx> crate::MakeHnd<Ctx, vk::extensions::ext::debug_utils> for vk::DebugUtilsMessengerEXT
        where Ctx: crate::HndCtx<vk::extensions::ext::debug_utils, vk::Instance>,
    {
        type Output = crate::hnd::DebugUtilsMessengerEXT<vk::extensions::ext::debug_utils>;
        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {
            unsafe { crate::hnd::DebugUtilsMessengerEXT::<vk::extensions::ext::debug_utils>::new_with(ctx, self, dep) }
        }
    }
}

/// `VK_EXT_debug_utils` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::ext::debug_utils::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::ext::debug_utils::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// void vkCmdBeginDebugUtilsLabelEXT(VkCommandBuffer commandBuffer, VkDebugUtilsLabelEXT const* pLabelInfo)
    /// ```
    pub unsafe fn cmd_begin_debug_utils_label(
        &self,
        command_buffer: vk::CommandBuffer,
        label_info: &DebugUtilsLabelEXT,
    ) -> () {
        unsafe {
            self.0.CmdBeginDebugUtilsLabelEXT(
                command_buffer.abi(), 
                label_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdEndDebugUtilsLabelEXT(VkCommandBuffer commandBuffer)
    /// ```
    pub unsafe fn cmd_end_debug_utils_label(
        &self,
        command_buffer: vk::CommandBuffer,
    ) -> () {
        unsafe {
            self.0.CmdEndDebugUtilsLabelEXT(
                command_buffer.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdInsertDebugUtilsLabelEXT(VkCommandBuffer commandBuffer, VkDebugUtilsLabelEXT const* pLabelInfo)
    /// ```
    pub unsafe fn cmd_insert_debug_utils_label(
        &self,
        command_buffer: vk::CommandBuffer,
        label_info: &DebugUtilsLabelEXT,
    ) -> () {
        unsafe {
            self.0.CmdInsertDebugUtilsLabelEXT(
                command_buffer.abi(), 
                label_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkQueueBeginDebugUtilsLabelEXT(VkQueue queue, VkDebugUtilsLabelEXT const* pLabelInfo)
    /// ```
    pub unsafe fn queue_begin_debug_utils_label(
        &self,
        queue: vk::Queue,
        label_info: &DebugUtilsLabelEXT,
    ) -> () {
        unsafe {
            self.0.QueueBeginDebugUtilsLabelEXT(
                queue.abi(), 
                label_info.abi(), 
            );
        }
    }
    /// ```c
    /// void vkQueueEndDebugUtilsLabelEXT(VkQueue queue)
    /// ```
    pub unsafe fn queue_end_debug_utils_label(
        &self,
        queue: vk::Queue,
    ) -> () {
        unsafe {
            self.0.QueueEndDebugUtilsLabelEXT(
                queue.abi(), 
            );
        }
    }
    /// ```c
    /// void vkQueueInsertDebugUtilsLabelEXT(VkQueue queue, VkDebugUtilsLabelEXT const* pLabelInfo)
    /// ```
    pub unsafe fn queue_insert_debug_utils_label(
        &self,
        queue: vk::Queue,
        label_info: &DebugUtilsLabelEXT,
    ) -> () {
        unsafe {
            self.0.QueueInsertDebugUtilsLabelEXT(
                queue.abi(), 
                label_info.abi(), 
            );
        }
    }
    /// ```c
    /// VkResult vkSetDebugUtilsObjectNameEXT(VkDevice device, VkDebugUtilsObjectNameInfoEXT const* pNameInfo)
    /// ```
    pub unsafe fn set_debug_utils_object_name(
        &self,
        device: vk::Device,
        name_info: &DebugUtilsObjectNameInfoEXT,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.SetDebugUtilsObjectNameEXT(
                device.abi(), 
                name_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkSetDebugUtilsObjectTagEXT(VkDevice device, VkDebugUtilsObjectTagInfoEXT const* pTagInfo)
    /// ```
    pub unsafe fn set_debug_utils_object_tag(
        &self,
        device: vk::Device,
        tag_info: &DebugUtilsObjectTagInfoEXT,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.SetDebugUtilsObjectTagEXT(
                device.abi(), 
                tag_info.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::ext::debug_utils {
    type Commands = Device;
}

/// Device object
pub trait ExtDebugUtilsDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkSetDebugUtilsObjectNameEXT(VkDevice device, VkDebugUtilsObjectNameInfoEXT const* pNameInfo)
    /// ```
    unsafe fn set_debug_utils_object_name(
        &self,
        name_info: &DebugUtilsObjectNameInfoEXT,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().set_debug_utils_object_name(
                self.raw(),
                name_info,
            )
        }
    }
    /// ```c
    /// VkResult vkSetDebugUtilsObjectTagEXT(VkDevice device, VkDebugUtilsObjectTagInfoEXT const* pTagInfo)
    /// ```
    unsafe fn set_debug_utils_object_tag(
        &self,
        tag_info: &DebugUtilsObjectTagInfoEXT,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().set_debug_utils_object_tag(
                self.raw(),
                tag_info,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::ext::debug_utils {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::ext::debug_utils> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::ext::debug_utils {
        type Output = crate::hnd::Device<vk::extensions::ext::debug_utils>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::ext::debug_utils>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::ext::debug_utils> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::ext::debug_utils> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::ext::debug_utils> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::ext::debug_utils> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::ExtDebugUtilsDevice for crate::hnd::Device<vk::extensions::ext::debug_utils> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::ext::debug_utils, vk::Device> for crate::hnd::Device<vk::extensions::ext::debug_utils> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait ExtDebugUtilsCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdBeginDebugUtilsLabelEXT(VkCommandBuffer commandBuffer, VkDebugUtilsLabelEXT const* pLabelInfo)
    /// ```
    unsafe fn begin_debug_utils_label(
        &self,
        label_info: &DebugUtilsLabelEXT,
    ) -> () {
        unsafe {
            self.commands().cmd_begin_debug_utils_label(
                self.raw(),
                label_info,
            )
        }
    }
    /// ```c
    /// void vkCmdEndDebugUtilsLabelEXT(VkCommandBuffer commandBuffer)
    /// ```
    unsafe fn end_debug_utils_label(
        &self,
    ) -> () {
        unsafe {
            self.commands().cmd_end_debug_utils_label(
                self.raw(),
            )
        }
    }
    /// ```c
    /// void vkCmdInsertDebugUtilsLabelEXT(VkCommandBuffer commandBuffer, VkDebugUtilsLabelEXT const* pLabelInfo)
    /// ```
    unsafe fn insert_debug_utils_label(
        &self,
        label_info: &DebugUtilsLabelEXT,
    ) -> () {
        unsafe {
            self.commands().cmd_insert_debug_utils_label(
                self.raw(),
                label_info,
            )
        }
    }
}

/// Queue object
pub trait ExtDebugUtilsQueue {
    fn raw(&self) -> vk::Queue;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkQueueBeginDebugUtilsLabelEXT(VkQueue queue, VkDebugUtilsLabelEXT const* pLabelInfo)
    /// ```
    unsafe fn begin_debug_utils_label(
        &self,
        label_info: &DebugUtilsLabelEXT,
    ) -> () {
        unsafe {
            self.commands().queue_begin_debug_utils_label(
                self.raw(),
                label_info,
            )
        }
    }
    /// ```c
    /// void vkQueueEndDebugUtilsLabelEXT(VkQueue queue)
    /// ```
    unsafe fn end_debug_utils_label(
        &self,
    ) -> () {
        unsafe {
            self.commands().queue_end_debug_utils_label(
                self.raw(),
            )
        }
    }
    /// ```c
    /// void vkQueueInsertDebugUtilsLabelEXT(VkQueue queue, VkDebugUtilsLabelEXT const* pLabelInfo)
    /// ```
    unsafe fn insert_debug_utils_label(
        &self,
        label_info: &DebugUtilsLabelEXT,
    ) -> () {
        unsafe {
            self.commands().queue_insert_debug_utils_label(
                self.raw(),
                label_info,
            )
        }
    }
}
