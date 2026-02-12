// generated file, do not modify manually
#![allow(unused_qualifications)]
#![allow(mismatched_lifetime_syntaxes)]
use crate::sys;
use crate::sys::ffi::*;
use crate::vk::*;
use crate::vk;
use crate::{Abi, Vk, Sys};

/// `VK_KHR_device_group` InstanceCommands
#[derive(Debug, Clone, Copy)]
pub struct Instance(pub sys::khr::device_group::InstanceCommands);

impl Instance {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::device_group::InstanceCommands::load(get) })
    }
}

impl Instance {
    /// ```c
    /// VkResult vkGetPhysicalDevicePresentRectanglesKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pRectCount, VkRect2D* pRects)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    pub unsafe fn get_physical_device_present_rectangles(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        rects: Option<&mut ::alloc::vec::Vec<Rect2D>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            let mut _c: uint32_t = Default::default();
            let mut _r = self.0.GetPhysicalDevicePresentRectanglesKHR(
                physical_device.abi(), 
                surface.abi(), 
                &mut _c, 
                Default::default(), 
            ).vk();
            if let Some(_b) = rects {
                _r.result(|| Some(()))?;
                _b.reserve(_c as usize);
                _r = self.0.GetPhysicalDevicePresentRectanglesKHR(
                    physical_device.abi(), 
                    surface.abi(), 
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

impl crate::CommandScope<vk::Instance> for vk::extensions::khr::device_group {
    type Commands = Instance;
}

/// Instance object
pub trait KhrDeviceGroupInstance {
    fn raw(&self) -> vk::Instance;
    fn commands(&self) -> &Instance;

}

impl crate::HndScope<vk::Instance> for vk::extensions::khr::device_group {
    type Impl = _hs_Instance::Instance;
}

mod _hs_Instance {
    use super::*;
    #[derive(Debug)]
    pub struct Instance(pub(crate) ::alloc::sync::Arc<super::Instance>, pub(crate) crate::hnd::Instance<vk::core>);

    impl Clone for Instance {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Instance<vk::extensions::khr::device_group> {
        pub unsafe fn new(base: &crate::hnd::Instance<vk::core>) -> Self {
            unsafe {
                Self(Instance(
                   ::alloc::sync::Arc::new(super::Instance::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Instance<vk::core>> for vk::extensions::khr::device_group {
        type Output = crate::hnd::Instance<vk::extensions::khr::device_group>;
        unsafe fn make(target: &crate::hnd::Instance<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Instance::<vk::extensions::khr::device_group>::new(target) }
        }
    }

    impl crate::hnd::Instance<vk::extensions::khr::device_group> {
        pub fn raw(&self) -> vk::Instance { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Instance<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Instance<vk::extensions::khr::device_group> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Instance({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Instance<vk::extensions::khr::device_group> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Instance<vk::extensions::khr::device_group> {
        type Target = super::Instance;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrDeviceGroupInstance for crate::hnd::Instance<vk::extensions::khr::device_group> {
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &super::Instance { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::device_group, vk::Instance> for crate::hnd::Instance<vk::extensions::khr::device_group> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Instance { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Instance> { self.commands() }
    }
}

/// PhysicalDevice object
pub trait KhrDeviceGroupPhysicalDevice {
    fn raw(&self) -> vk::PhysicalDevice;
    fn commands(&self) -> &Instance;

    /// ```c
    /// VkResult vkGetPhysicalDevicePresentRectanglesKHR(VkPhysicalDevice physicalDevice, VkSurfaceKHR surface, uint32_t* pRectCount, VkRect2D* pRects)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Incomplete]
    unsafe fn get_present_rectangles(
        &self,
        surface: vk::SurfaceKHR,
        rects: Option<&mut ::alloc::vec::Vec<Rect2D>>,
    ) -> crate::Result<(uint32_t, Result)> {
        unsafe {
            self.commands().get_physical_device_present_rectangles(
                self.raw(),
                surface,
                rects,
            )
        }
    }
}

/// `VK_KHR_device_group` DeviceCommands
#[derive(Debug, Clone, Copy)]
pub struct Device(pub sys::khr::device_group::DeviceCommands);

impl Device {
    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {
        Self(unsafe { sys::khr::device_group::DeviceCommands::load(get) })
    }
}

impl Device {
    /// ```c
    /// VkResult vkAcquireNextImage2KHR(VkDevice device, VkAcquireNextImageInfoKHR const* pAcquireInfo, uint32_t* pImageIndex)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Timeout], [Result::NotReady], [Result::SuboptimalKhr]
    pub unsafe fn acquire_next_image2(
        &self,
        device: vk::Device,
        acquire_info: &AcquireNextImageInfoKHR,
        image_index: *mut uint32_t,
    ) -> crate::Result<Result> {
        unsafe {
            let _r = self.0.AcquireNextImage2KHR(
                device.abi(), 
                acquire_info.abi(), 
                image_index.abi(), 
            ).vk();
            _r.result(|| Some(_r))
        }
    }
    /// ```c
    /// void vkCmdDispatchBase(VkCommandBuffer commandBuffer, uint32_t baseGroupX, uint32_t baseGroupY, uint32_t baseGroupZ, uint32_t groupCountX, uint32_t groupCountY, uint32_t groupCountZ)
    /// ```
    pub unsafe fn cmd_dispatch_base(
        &self,
        command_buffer: vk::CommandBuffer,
        base_group_x: uint32_t,
        base_group_y: uint32_t,
        base_group_z: uint32_t,
        group_count_x: uint32_t,
        group_count_y: uint32_t,
        group_count_z: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdDispatchBaseKHR(
                command_buffer.abi(), 
                base_group_x.abi(), 
                base_group_y.abi(), 
                base_group_z.abi(), 
                group_count_x.abi(), 
                group_count_y.abi(), 
                group_count_z.abi(), 
            );
        }
    }
    /// ```c
    /// void vkCmdSetDeviceMask(VkCommandBuffer commandBuffer, uint32_t deviceMask)
    /// ```
    pub unsafe fn cmd_set_device_mask(
        &self,
        command_buffer: vk::CommandBuffer,
        device_mask: uint32_t,
    ) -> () {
        unsafe {
            self.0.CmdSetDeviceMaskKHR(
                command_buffer.abi(), 
                device_mask.abi(), 
            );
        }
    }
    /// ```c
    /// void vkGetDeviceGroupPeerMemoryFeatures(VkDevice device, uint32_t heapIndex, uint32_t localDeviceIndex, uint32_t remoteDeviceIndex, VkPeerMemoryFeatureFlags* pPeerMemoryFeatures)
    /// ```
    pub unsafe fn get_device_group_peer_memory_features(
        &self,
        device: vk::Device,
        heap_index: uint32_t,
        local_device_index: uint32_t,
        remote_device_index: uint32_t,
    ) -> PeerMemoryFeatureFlags {
        unsafe {
            let mut _v: PeerMemoryFeatureFlags = Default::default();
            self.0.GetDeviceGroupPeerMemoryFeaturesKHR(
                device.abi(), 
                heap_index.abi(), 
                local_device_index.abi(), 
                remote_device_index.abi(), 
                (&mut _v).abi(), 
            );
            _v
        }
    }
    /// ```c
    /// VkResult vkGetDeviceGroupPresentCapabilitiesKHR(VkDevice device, VkDeviceGroupPresentCapabilitiesKHR* pDeviceGroupPresentCapabilities)
    /// ```
    pub unsafe fn get_device_group_present_capabilities(
        &self,
        device: vk::Device,
        device_group_present_capabilities: &mut DeviceGroupPresentCapabilitiesKHR,
    ) -> crate::Result<()> {
        unsafe {
            let _r = self.0.GetDeviceGroupPresentCapabilitiesKHR(
                device.abi(), 
                device_group_present_capabilities.abi(), 
            ).vk();
            _r.result(|| Some(()))
        }
    }
    /// ```c
    /// VkResult vkGetDeviceGroupSurfacePresentModesKHR(VkDevice device, VkSurfaceKHR surface, VkDeviceGroupPresentModeFlagsKHR* pModes)
    /// ```
    pub unsafe fn get_device_group_surface_present_modes(
        &self,
        device: vk::Device,
        surface: vk::SurfaceKHR,
    ) -> crate::Result<DeviceGroupPresentModeFlagsKHR> {
        unsafe {
            let mut _v: DeviceGroupPresentModeFlagsKHR = Default::default();
            let _r = self.0.GetDeviceGroupSurfacePresentModesKHR(
                device.abi(), 
                surface.abi(), 
                (&mut _v).abi(), 
            ).vk();
            _r.result(|| Some(_v))
        }
    }
}

impl crate::CommandScope<vk::Device> for vk::extensions::khr::device_group {
    type Commands = Device;
}

/// Device object
pub trait KhrDeviceGroupDevice {
    fn raw(&self) -> vk::Device;
    fn commands(&self) -> &Device;

    /// ```c
    /// VkResult vkAcquireNextImage2KHR(VkDevice device, VkAcquireNextImageInfoKHR const* pAcquireInfo, uint32_t* pImageIndex)
    /// ```
    ///
    /// SuccessCodes: [Result::Success], [Result::Timeout], [Result::NotReady], [Result::SuboptimalKhr]
    unsafe fn acquire_next_image2(
        &self,
        acquire_info: &AcquireNextImageInfoKHR,
        image_index: *mut uint32_t,
    ) -> crate::Result<Result> {
        unsafe {
            self.commands().acquire_next_image2(
                self.raw(),
                acquire_info,
                image_index,
            )
        }
    }
    /// ```c
    /// void vkGetDeviceGroupPeerMemoryFeatures(VkDevice device, uint32_t heapIndex, uint32_t localDeviceIndex, uint32_t remoteDeviceIndex, VkPeerMemoryFeatureFlags* pPeerMemoryFeatures)
    /// ```
    unsafe fn get_device_group_peer_memory_features(
        &self,
        heap_index: uint32_t,
        local_device_index: uint32_t,
        remote_device_index: uint32_t,
    ) -> PeerMemoryFeatureFlags {
        unsafe {
            self.commands().get_device_group_peer_memory_features(
                self.raw(),
                heap_index,
                local_device_index,
                remote_device_index,
            )
        }
    }
    /// ```c
    /// VkResult vkGetDeviceGroupPresentCapabilitiesKHR(VkDevice device, VkDeviceGroupPresentCapabilitiesKHR* pDeviceGroupPresentCapabilities)
    /// ```
    unsafe fn get_device_group_present_capabilities(
        &self,
        device_group_present_capabilities: &mut DeviceGroupPresentCapabilitiesKHR,
    ) -> crate::Result<()> {
        unsafe {
            self.commands().get_device_group_present_capabilities(
                self.raw(),
                device_group_present_capabilities,
            )
        }
    }
    /// ```c
    /// VkResult vkGetDeviceGroupSurfacePresentModesKHR(VkDevice device, VkSurfaceKHR surface, VkDeviceGroupPresentModeFlagsKHR* pModes)
    /// ```
    unsafe fn get_device_group_surface_present_modes(
        &self,
        surface: vk::SurfaceKHR,
    ) -> crate::Result<DeviceGroupPresentModeFlagsKHR> {
        unsafe {
            self.commands().get_device_group_surface_present_modes(
                self.raw(),
                surface,
            )
        }
    }
}

impl crate::HndScope<vk::Device> for vk::extensions::khr::device_group {
    type Impl = _hs_Device::Device;
}

mod _hs_Device {
    use super::*;
    #[derive(Debug)]
    pub struct Device(pub(crate) ::alloc::sync::Arc<super::Device>, pub(crate) crate::hnd::Device<vk::core>);

    impl Clone for Device {
        fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
    }

    impl crate::hnd::Device<vk::extensions::khr::device_group> {
        pub unsafe fn new(base: &crate::hnd::Device<vk::core>) -> Self {
            unsafe {
                Self(Device(
                   ::alloc::sync::Arc::new(super::Device::load(|name| unsafe { base.get_proc_addr(name) })),
                   base.clone(),
                ))
            }
        }
    }

    impl crate::Extension<crate::hnd::Device<vk::core>> for vk::extensions::khr::device_group {
        type Output = crate::hnd::Device<vk::extensions::khr::device_group>;
        unsafe fn make(target: &crate::hnd::Device<vk::core>) -> Self::Output {
            unsafe { crate::hnd::Device::<vk::extensions::khr::device_group>::new(target) }
        }
    }

    impl crate::hnd::Device<vk::extensions::khr::device_group> {
        pub fn raw(&self) -> vk::Device { self.0.1.raw() }
        pub fn commands(&self) -> &::alloc::sync::Arc<super::Device> { &self.0.0 }
        pub fn core(&self) -> &crate::hnd::Device<vk::core> { &self.0.1 }
    }

    impl ::core::fmt::Debug for crate::hnd::Device<vk::extensions::khr::device_group> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_fmt(format_args!("Device({:p})", self.raw()))
        }
    }

    impl ::core::fmt::Pointer for crate::hnd::Device<vk::extensions::khr::device_group> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.raw().fmt(f)
        }
    }

    impl ::core::ops::Deref for crate::hnd::Device<vk::extensions::khr::device_group> {
        type Target = super::Device;
        fn deref(&self) -> &Self::Target { self.commands() }
    }

    impl super::KhrDeviceGroupDevice for crate::hnd::Device<vk::extensions::khr::device_group> {
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &super::Device { self.commands() }
    }

    impl crate::HndCtx<vk::extensions::khr::device_group, vk::Device> for crate::hnd::Device<vk::extensions::khr::device_group> {
        type Ctx = Self;
        fn ctx(&self) -> Self::Ctx { self.clone() }
        fn raw(&self) -> vk::Device { self.raw() }
        fn commands(&self) -> &::alloc::sync::Arc<super::Device> { self.commands() }
    }
}

/// CommandBuffer object
pub trait KhrDeviceGroupCommandBuffer {
    fn raw(&self) -> vk::CommandBuffer;
    fn commands(&self) -> &Device;

    /// ```c
    /// void vkCmdDispatchBase(VkCommandBuffer commandBuffer, uint32_t baseGroupX, uint32_t baseGroupY, uint32_t baseGroupZ, uint32_t groupCountX, uint32_t groupCountY, uint32_t groupCountZ)
    /// ```
    unsafe fn dispatch_base(
        &self,
        base_group_x: uint32_t,
        base_group_y: uint32_t,
        base_group_z: uint32_t,
        group_count_x: uint32_t,
        group_count_y: uint32_t,
        group_count_z: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_dispatch_base(
                self.raw(),
                base_group_x,
                base_group_y,
                base_group_z,
                group_count_x,
                group_count_y,
                group_count_z,
            )
        }
    }
    /// ```c
    /// void vkCmdSetDeviceMask(VkCommandBuffer commandBuffer, uint32_t deviceMask)
    /// ```
    unsafe fn set_device_mask(
        &self,
        device_mask: uint32_t,
    ) -> () {
        unsafe {
            self.commands().cmd_set_device_mask(
                self.raw(),
                device_mask,
            )
        }
    }
}
