// generated file, do not modify manually


use crate::vk::*;
use ::core::ptr::NonNull;
use ::core::num::NonZeroU64;
use crate::sys::ffi::*;
use crate::sys;

/// `VkAccelerationStructureKHR` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccelerationStructureKHR(pub NonZeroU64);
from_by_transmute!(AccelerationStructureKHR => sys::VkAccelerationStructureKHR);
impl_raw_handle!(AccelerationStructureKHR: VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR => sys::VkAccelerationStructureKHR; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(AccelerationStructureKHR);

impl AccelerationStructureKHR {
   /// Creates a new `AccelerationStructureKHR` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `AccelerationStructureKHR` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkAccelerationStructureNV` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccelerationStructureNV(pub NonZeroU64);
from_by_transmute!(AccelerationStructureNV => sys::VkAccelerationStructureNV);
impl_raw_handle!(AccelerationStructureNV: VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV => sys::VkAccelerationStructureNV; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(AccelerationStructureNV);

impl AccelerationStructureNV {
   /// Creates a new `AccelerationStructureNV` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `AccelerationStructureNV` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkBuffer` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Buffer(pub NonZeroU64);
from_by_transmute!(Buffer => sys::VkBuffer);
impl_raw_handle!(Buffer: VK_OBJECT_TYPE_BUFFER => sys::VkBuffer; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(Buffer);

impl Buffer {
   /// Creates a new `Buffer` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `Buffer` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkBufferCollectionFUCHSIA` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferCollectionFUCHSIA(pub NonZeroU64);
from_by_transmute!(BufferCollectionFUCHSIA => sys::VkBufferCollectionFUCHSIA);
impl_raw_handle!(BufferCollectionFUCHSIA: VK_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA => sys::VkBufferCollectionFUCHSIA; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(BufferCollectionFUCHSIA);

impl BufferCollectionFUCHSIA {
   /// Creates a new `BufferCollectionFUCHSIA` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `BufferCollectionFUCHSIA` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkBufferView` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferView(pub NonZeroU64);
from_by_transmute!(BufferView => sys::VkBufferView);
impl_raw_handle!(BufferView: VK_OBJECT_TYPE_BUFFER_VIEW => sys::VkBufferView; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(BufferView);

impl BufferView {
   /// Creates a new `BufferView` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `BufferView` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkCommandBuffer` : `VkCommandPool`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandBuffer(pub NonNull<void>);
from_by_transmute!(CommandBuffer => sys::VkCommandBuffer);
impl_raw_handle!(CommandBuffer: VK_OBJECT_TYPE_COMMAND_BUFFER => sys::VkCommandBuffer; RawSubHandle = CommandPool);
impl_debug_for_raw_dispatchable_handle!(CommandBuffer);

impl CommandBuffer {
    /// Creates a new `CommandBuffer` from a raw handle value. Returns `None` if the value is null.
    pub fn new(raw: *mut void) -> Option<Self> {
        NonNull::new(raw).map(Self)
    }
    /// Creates a new `CommandBuffer` from a raw handle value without checking if it's null. The caller must ensure that the value is not null.
    pub unsafe fn new_unchecked(raw: *mut void) -> Self {
        Self(unsafe { NonNull::new_unchecked(raw) })
    }
}

/// `VkCommandPool` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandPool(pub NonZeroU64);
from_by_transmute!(CommandPool => sys::VkCommandPool);
impl_raw_handle!(CommandPool: VK_OBJECT_TYPE_COMMAND_POOL => sys::VkCommandPool; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(CommandPool);

impl CommandPool {
   /// Creates a new `CommandPool` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `CommandPool` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkCuFunctionNVX` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CuFunctionNVX(pub NonZeroU64);
from_by_transmute!(CuFunctionNVX => sys::VkCuFunctionNVX);
impl_raw_handle!(CuFunctionNVX: VK_OBJECT_TYPE_CU_FUNCTION_NVX => sys::VkCuFunctionNVX; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(CuFunctionNVX);

impl CuFunctionNVX {
   /// Creates a new `CuFunctionNVX` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `CuFunctionNVX` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkCuModuleNVX` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CuModuleNVX(pub NonZeroU64);
from_by_transmute!(CuModuleNVX => sys::VkCuModuleNVX);
impl_raw_handle!(CuModuleNVX: VK_OBJECT_TYPE_CU_MODULE_NVX => sys::VkCuModuleNVX; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(CuModuleNVX);

impl CuModuleNVX {
   /// Creates a new `CuModuleNVX` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `CuModuleNVX` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkCudaFunctionNV` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CudaFunctionNV(pub NonZeroU64);
from_by_transmute!(CudaFunctionNV => sys::VkCudaFunctionNV);
impl_raw_handle!(CudaFunctionNV: VK_OBJECT_TYPE_CUDA_FUNCTION_NV => sys::VkCudaFunctionNV; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(CudaFunctionNV);

impl CudaFunctionNV {
   /// Creates a new `CudaFunctionNV` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `CudaFunctionNV` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkCudaModuleNV` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CudaModuleNV(pub NonZeroU64);
from_by_transmute!(CudaModuleNV => sys::VkCudaModuleNV);
impl_raw_handle!(CudaModuleNV: VK_OBJECT_TYPE_CUDA_MODULE_NV => sys::VkCudaModuleNV; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(CudaModuleNV);

impl CudaModuleNV {
   /// Creates a new `CudaModuleNV` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `CudaModuleNV` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkDataGraphPipelineSessionARM` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataGraphPipelineSessionARM(pub NonZeroU64);
from_by_transmute!(DataGraphPipelineSessionARM => sys::VkDataGraphPipelineSessionARM);
impl_raw_handle!(DataGraphPipelineSessionARM: VK_OBJECT_TYPE_DATA_GRAPH_PIPELINE_SESSION_ARM => sys::VkDataGraphPipelineSessionARM; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(DataGraphPipelineSessionARM);

impl DataGraphPipelineSessionARM {
   /// Creates a new `DataGraphPipelineSessionARM` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `DataGraphPipelineSessionARM` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkDebugReportCallbackEXT` : `VkInstance`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugReportCallbackEXT(pub NonZeroU64);
from_by_transmute!(DebugReportCallbackEXT => sys::VkDebugReportCallbackEXT);
impl_raw_handle!(DebugReportCallbackEXT: VK_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT => sys::VkDebugReportCallbackEXT; RawSubHandle = Instance);
impl_debug_for_raw_non_dispatchable_handle!(DebugReportCallbackEXT);

impl DebugReportCallbackEXT {
   /// Creates a new `DebugReportCallbackEXT` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `DebugReportCallbackEXT` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkDebugUtilsMessengerEXT` : `VkInstance`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugUtilsMessengerEXT(pub NonZeroU64);
from_by_transmute!(DebugUtilsMessengerEXT => sys::VkDebugUtilsMessengerEXT);
impl_raw_handle!(DebugUtilsMessengerEXT: VK_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT => sys::VkDebugUtilsMessengerEXT; RawSubHandle = Instance);
impl_debug_for_raw_non_dispatchable_handle!(DebugUtilsMessengerEXT);

impl DebugUtilsMessengerEXT {
   /// Creates a new `DebugUtilsMessengerEXT` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `DebugUtilsMessengerEXT` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkDeferredOperationKHR` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeferredOperationKHR(pub NonZeroU64);
from_by_transmute!(DeferredOperationKHR => sys::VkDeferredOperationKHR);
impl_raw_handle!(DeferredOperationKHR: VK_OBJECT_TYPE_DEFERRED_OPERATION_KHR => sys::VkDeferredOperationKHR; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(DeferredOperationKHR);

impl DeferredOperationKHR {
   /// Creates a new `DeferredOperationKHR` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `DeferredOperationKHR` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkDescriptorPool` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorPool(pub NonZeroU64);
from_by_transmute!(DescriptorPool => sys::VkDescriptorPool);
impl_raw_handle!(DescriptorPool: VK_OBJECT_TYPE_DESCRIPTOR_POOL => sys::VkDescriptorPool; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(DescriptorPool);

impl DescriptorPool {
   /// Creates a new `DescriptorPool` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `DescriptorPool` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkDescriptorSet` : `VkDescriptorPool`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorSet(pub NonZeroU64);
from_by_transmute!(DescriptorSet => sys::VkDescriptorSet);
impl_raw_handle!(DescriptorSet: VK_OBJECT_TYPE_DESCRIPTOR_SET => sys::VkDescriptorSet; RawSubHandle = DescriptorPool);
impl_debug_for_raw_non_dispatchable_handle!(DescriptorSet);

impl DescriptorSet {
   /// Creates a new `DescriptorSet` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `DescriptorSet` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkDescriptorSetLayout` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorSetLayout(pub NonZeroU64);
from_by_transmute!(DescriptorSetLayout => sys::VkDescriptorSetLayout);
impl_raw_handle!(DescriptorSetLayout: VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT => sys::VkDescriptorSetLayout; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(DescriptorSetLayout);

impl DescriptorSetLayout {
   /// Creates a new `DescriptorSetLayout` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `DescriptorSetLayout` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkDescriptorUpdateTemplate` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorUpdateTemplate(pub NonZeroU64);
from_by_transmute!(DescriptorUpdateTemplate => sys::VkDescriptorUpdateTemplate);
impl_raw_handle!(DescriptorUpdateTemplate: VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE => sys::VkDescriptorUpdateTemplate; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(DescriptorUpdateTemplate);

impl DescriptorUpdateTemplate {
   /// Creates a new `DescriptorUpdateTemplate` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `DescriptorUpdateTemplate` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkDescriptorUpdateTemplateKHR` = `VkDescriptorUpdateTemplate`
pub type DescriptorUpdateTemplateKHR = DescriptorUpdateTemplate;

/// `VkDevice` : `VkPhysicalDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Device(pub NonNull<void>);
from_by_transmute!(Device => sys::VkDevice);
impl_raw_handle!(Device: VK_OBJECT_TYPE_DEVICE => sys::VkDevice; RawSubHandle = PhysicalDevice);
impl_debug_for_raw_dispatchable_handle!(Device);

impl Device {
    /// Creates a new `Device` from a raw handle value. Returns `None` if the value is null.
    pub fn new(raw: *mut void) -> Option<Self> {
        NonNull::new(raw).map(Self)
    }
    /// Creates a new `Device` from a raw handle value without checking if it's null. The caller must ensure that the value is not null.
    pub unsafe fn new_unchecked(raw: *mut void) -> Self {
        Self(unsafe { NonNull::new_unchecked(raw) })
    }
}

/// `VkDeviceMemory` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceMemory(pub NonZeroU64);
from_by_transmute!(DeviceMemory => sys::VkDeviceMemory);
impl_raw_handle!(DeviceMemory: VK_OBJECT_TYPE_DEVICE_MEMORY => sys::VkDeviceMemory; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(DeviceMemory);

impl DeviceMemory {
   /// Creates a new `DeviceMemory` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `DeviceMemory` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkDisplayKHR` : `VkPhysicalDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayKHR(pub NonZeroU64);
from_by_transmute!(DisplayKHR => sys::VkDisplayKHR);
impl_raw_handle!(DisplayKHR: VK_OBJECT_TYPE_DISPLAY_KHR => sys::VkDisplayKHR; RawSubHandle = PhysicalDevice);
impl_debug_for_raw_non_dispatchable_handle!(DisplayKHR);

impl DisplayKHR {
   /// Creates a new `DisplayKHR` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `DisplayKHR` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkDisplayModeKHR` : `VkDisplayKHR`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayModeKHR(pub NonZeroU64);
from_by_transmute!(DisplayModeKHR => sys::VkDisplayModeKHR);
impl_raw_handle!(DisplayModeKHR: VK_OBJECT_TYPE_DISPLAY_MODE_KHR => sys::VkDisplayModeKHR; RawSubHandle = DisplayKHR);
impl_debug_for_raw_non_dispatchable_handle!(DisplayModeKHR);

impl DisplayModeKHR {
   /// Creates a new `DisplayModeKHR` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `DisplayModeKHR` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkEvent` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Event(pub NonZeroU64);
from_by_transmute!(Event => sys::VkEvent);
impl_raw_handle!(Event: VK_OBJECT_TYPE_EVENT => sys::VkEvent; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(Event);

impl Event {
   /// Creates a new `Event` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `Event` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkExternalComputeQueueNV` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalComputeQueueNV(pub NonNull<void>);
from_by_transmute!(ExternalComputeQueueNV => sys::VkExternalComputeQueueNV);
impl_raw_handle!(ExternalComputeQueueNV: VK_OBJECT_TYPE_EXTERNAL_COMPUTE_QUEUE_NV => sys::VkExternalComputeQueueNV; RawSubHandle = Device);
impl_debug_for_raw_dispatchable_handle!(ExternalComputeQueueNV);

impl ExternalComputeQueueNV {
    /// Creates a new `ExternalComputeQueueNV` from a raw handle value. Returns `None` if the value is null.
    pub fn new(raw: *mut void) -> Option<Self> {
        NonNull::new(raw).map(Self)
    }
    /// Creates a new `ExternalComputeQueueNV` from a raw handle value without checking if it's null. The caller must ensure that the value is not null.
    pub unsafe fn new_unchecked(raw: *mut void) -> Self {
        Self(unsafe { NonNull::new_unchecked(raw) })
    }
}

/// `VkFence` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fence(pub NonZeroU64);
from_by_transmute!(Fence => sys::VkFence);
impl_raw_handle!(Fence: VK_OBJECT_TYPE_FENCE => sys::VkFence; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(Fence);

impl Fence {
   /// Creates a new `Fence` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `Fence` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkFramebuffer` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Framebuffer(pub NonZeroU64);
from_by_transmute!(Framebuffer => sys::VkFramebuffer);
impl_raw_handle!(Framebuffer: VK_OBJECT_TYPE_FRAMEBUFFER => sys::VkFramebuffer; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(Framebuffer);

impl Framebuffer {
   /// Creates a new `Framebuffer` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `Framebuffer` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkImage` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Image(pub NonZeroU64);
from_by_transmute!(Image => sys::VkImage);
impl_raw_handle!(Image: VK_OBJECT_TYPE_IMAGE => sys::VkImage; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(Image);

impl Image {
   /// Creates a new `Image` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `Image` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkImageView` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageView(pub NonZeroU64);
from_by_transmute!(ImageView => sys::VkImageView);
impl_raw_handle!(ImageView: VK_OBJECT_TYPE_IMAGE_VIEW => sys::VkImageView; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(ImageView);

impl ImageView {
   /// Creates a new `ImageView` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `ImageView` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkIndirectCommandsLayoutEXT` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectCommandsLayoutEXT(pub NonZeroU64);
from_by_transmute!(IndirectCommandsLayoutEXT => sys::VkIndirectCommandsLayoutEXT);
impl_raw_handle!(IndirectCommandsLayoutEXT: VK_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_EXT => sys::VkIndirectCommandsLayoutEXT; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(IndirectCommandsLayoutEXT);

impl IndirectCommandsLayoutEXT {
   /// Creates a new `IndirectCommandsLayoutEXT` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `IndirectCommandsLayoutEXT` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkIndirectCommandsLayoutNV` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectCommandsLayoutNV(pub NonZeroU64);
from_by_transmute!(IndirectCommandsLayoutNV => sys::VkIndirectCommandsLayoutNV);
impl_raw_handle!(IndirectCommandsLayoutNV: VK_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NV => sys::VkIndirectCommandsLayoutNV; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(IndirectCommandsLayoutNV);

impl IndirectCommandsLayoutNV {
   /// Creates a new `IndirectCommandsLayoutNV` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `IndirectCommandsLayoutNV` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkIndirectExecutionSetEXT` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectExecutionSetEXT(pub NonZeroU64);
from_by_transmute!(IndirectExecutionSetEXT => sys::VkIndirectExecutionSetEXT);
impl_raw_handle!(IndirectExecutionSetEXT: VK_OBJECT_TYPE_INDIRECT_EXECUTION_SET_EXT => sys::VkIndirectExecutionSetEXT; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(IndirectExecutionSetEXT);

impl IndirectExecutionSetEXT {
   /// Creates a new `IndirectExecutionSetEXT` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `IndirectExecutionSetEXT` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkInstance`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instance(pub NonNull<void>);
from_by_transmute!(Instance => sys::VkInstance);
impl_raw_handle!(Instance: VK_OBJECT_TYPE_INSTANCE => sys::VkInstance; RawRootHandle);
impl_debug_for_raw_dispatchable_handle!(Instance);

impl Instance {
    /// Creates a new `Instance` from a raw handle value. Returns `None` if the value is null.
    pub fn new(raw: *mut void) -> Option<Self> {
        NonNull::new(raw).map(Self)
    }
    /// Creates a new `Instance` from a raw handle value without checking if it's null. The caller must ensure that the value is not null.
    pub unsafe fn new_unchecked(raw: *mut void) -> Self {
        Self(unsafe { NonNull::new_unchecked(raw) })
    }
}

/// `VkMicromapEXT` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MicromapEXT(pub NonZeroU64);
from_by_transmute!(MicromapEXT => sys::VkMicromapEXT);
impl_raw_handle!(MicromapEXT: VK_OBJECT_TYPE_MICROMAP_EXT => sys::VkMicromapEXT; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(MicromapEXT);

impl MicromapEXT {
   /// Creates a new `MicromapEXT` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `MicromapEXT` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkOpticalFlowSessionNV` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpticalFlowSessionNV(pub NonZeroU64);
from_by_transmute!(OpticalFlowSessionNV => sys::VkOpticalFlowSessionNV);
impl_raw_handle!(OpticalFlowSessionNV: VK_OBJECT_TYPE_OPTICAL_FLOW_SESSION_NV => sys::VkOpticalFlowSessionNV; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(OpticalFlowSessionNV);

impl OpticalFlowSessionNV {
   /// Creates a new `OpticalFlowSessionNV` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `OpticalFlowSessionNV` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkPerformanceConfigurationINTEL` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerformanceConfigurationINTEL(pub NonZeroU64);
from_by_transmute!(PerformanceConfigurationINTEL => sys::VkPerformanceConfigurationINTEL);
impl_raw_handle!(PerformanceConfigurationINTEL: VK_OBJECT_TYPE_PERFORMANCE_CONFIGURATION_INTEL => sys::VkPerformanceConfigurationINTEL; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(PerformanceConfigurationINTEL);

impl PerformanceConfigurationINTEL {
   /// Creates a new `PerformanceConfigurationINTEL` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `PerformanceConfigurationINTEL` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkPhysicalDevice` : `VkInstance`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalDevice(pub NonNull<void>);
from_by_transmute!(PhysicalDevice => sys::VkPhysicalDevice);
impl_raw_handle!(PhysicalDevice: VK_OBJECT_TYPE_PHYSICAL_DEVICE => sys::VkPhysicalDevice; RawSubHandle = Instance);
impl_debug_for_raw_dispatchable_handle!(PhysicalDevice);

impl PhysicalDevice {
    /// Creates a new `PhysicalDevice` from a raw handle value. Returns `None` if the value is null.
    pub fn new(raw: *mut void) -> Option<Self> {
        NonNull::new(raw).map(Self)
    }
    /// Creates a new `PhysicalDevice` from a raw handle value without checking if it's null. The caller must ensure that the value is not null.
    pub unsafe fn new_unchecked(raw: *mut void) -> Self {
        Self(unsafe { NonNull::new_unchecked(raw) })
    }
}

/// `VkPipeline` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pipeline(pub NonZeroU64);
from_by_transmute!(Pipeline => sys::VkPipeline);
impl_raw_handle!(Pipeline: VK_OBJECT_TYPE_PIPELINE => sys::VkPipeline; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(Pipeline);

impl Pipeline {
   /// Creates a new `Pipeline` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `Pipeline` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkPipelineBinaryKHR` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineBinaryKHR(pub NonZeroU64);
from_by_transmute!(PipelineBinaryKHR => sys::VkPipelineBinaryKHR);
impl_raw_handle!(PipelineBinaryKHR: VK_OBJECT_TYPE_PIPELINE_BINARY_KHR => sys::VkPipelineBinaryKHR; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(PipelineBinaryKHR);

impl PipelineBinaryKHR {
   /// Creates a new `PipelineBinaryKHR` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `PipelineBinaryKHR` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkPipelineCache` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCache(pub NonZeroU64);
from_by_transmute!(PipelineCache => sys::VkPipelineCache);
impl_raw_handle!(PipelineCache: VK_OBJECT_TYPE_PIPELINE_CACHE => sys::VkPipelineCache; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(PipelineCache);

impl PipelineCache {
   /// Creates a new `PipelineCache` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `PipelineCache` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkPipelineLayout` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineLayout(pub NonZeroU64);
from_by_transmute!(PipelineLayout => sys::VkPipelineLayout);
impl_raw_handle!(PipelineLayout: VK_OBJECT_TYPE_PIPELINE_LAYOUT => sys::VkPipelineLayout; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(PipelineLayout);

impl PipelineLayout {
   /// Creates a new `PipelineLayout` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `PipelineLayout` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkPrivateDataSlot` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrivateDataSlot(pub NonZeroU64);
from_by_transmute!(PrivateDataSlot => sys::VkPrivateDataSlot);
impl_raw_handle!(PrivateDataSlot: VK_OBJECT_TYPE_PRIVATE_DATA_SLOT => sys::VkPrivateDataSlot; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(PrivateDataSlot);

impl PrivateDataSlot {
   /// Creates a new `PrivateDataSlot` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `PrivateDataSlot` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkPrivateDataSlotEXT` = `VkPrivateDataSlot`
pub type PrivateDataSlotEXT = PrivateDataSlot;

/// `VkQueryPool` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryPool(pub NonZeroU64);
from_by_transmute!(QueryPool => sys::VkQueryPool);
impl_raw_handle!(QueryPool: VK_OBJECT_TYPE_QUERY_POOL => sys::VkQueryPool; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(QueryPool);

impl QueryPool {
   /// Creates a new `QueryPool` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `QueryPool` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkQueue` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Queue(pub NonNull<void>);
from_by_transmute!(Queue => sys::VkQueue);
impl_raw_handle!(Queue: VK_OBJECT_TYPE_QUEUE => sys::VkQueue; RawSubHandle = Device);
impl_debug_for_raw_dispatchable_handle!(Queue);

impl Queue {
    /// Creates a new `Queue` from a raw handle value. Returns `None` if the value is null.
    pub fn new(raw: *mut void) -> Option<Self> {
        NonNull::new(raw).map(Self)
    }
    /// Creates a new `Queue` from a raw handle value without checking if it's null. The caller must ensure that the value is not null.
    pub unsafe fn new_unchecked(raw: *mut void) -> Self {
        Self(unsafe { NonNull::new_unchecked(raw) })
    }
}

/// `VkRenderPass` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RenderPass(pub NonZeroU64);
from_by_transmute!(RenderPass => sys::VkRenderPass);
impl_raw_handle!(RenderPass: VK_OBJECT_TYPE_RENDER_PASS => sys::VkRenderPass; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(RenderPass);

impl RenderPass {
   /// Creates a new `RenderPass` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `RenderPass` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkSampler` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sampler(pub NonZeroU64);
from_by_transmute!(Sampler => sys::VkSampler);
impl_raw_handle!(Sampler: VK_OBJECT_TYPE_SAMPLER => sys::VkSampler; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(Sampler);

impl Sampler {
   /// Creates a new `Sampler` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `Sampler` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkSamplerYcbcrConversion` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SamplerYcbcrConversion(pub NonZeroU64);
from_by_transmute!(SamplerYcbcrConversion => sys::VkSamplerYcbcrConversion);
impl_raw_handle!(SamplerYcbcrConversion: VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION => sys::VkSamplerYcbcrConversion; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(SamplerYcbcrConversion);

impl SamplerYcbcrConversion {
   /// Creates a new `SamplerYcbcrConversion` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `SamplerYcbcrConversion` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkSamplerYcbcrConversionKHR` = `VkSamplerYcbcrConversion`
pub type SamplerYcbcrConversionKHR = SamplerYcbcrConversion;

/// `VkSemaphore` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Semaphore(pub NonZeroU64);
from_by_transmute!(Semaphore => sys::VkSemaphore);
impl_raw_handle!(Semaphore: VK_OBJECT_TYPE_SEMAPHORE => sys::VkSemaphore; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(Semaphore);

impl Semaphore {
   /// Creates a new `Semaphore` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `Semaphore` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkSemaphoreSciSyncPoolNV` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemaphoreSciSyncPoolNV(pub NonZeroU64);
from_by_transmute!(SemaphoreSciSyncPoolNV => sys::VkSemaphoreSciSyncPoolNV);
impl_raw_handle!(SemaphoreSciSyncPoolNV: VK_OBJECT_TYPE_SEMAPHORE_SCI_SYNC_POOL_NV => sys::VkSemaphoreSciSyncPoolNV; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(SemaphoreSciSyncPoolNV);

impl SemaphoreSciSyncPoolNV {
   /// Creates a new `SemaphoreSciSyncPoolNV` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `SemaphoreSciSyncPoolNV` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkShaderEXT` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderEXT(pub NonZeroU64);
from_by_transmute!(ShaderEXT => sys::VkShaderEXT);
impl_raw_handle!(ShaderEXT: VK_OBJECT_TYPE_SHADER_EXT => sys::VkShaderEXT; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(ShaderEXT);

impl ShaderEXT {
   /// Creates a new `ShaderEXT` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `ShaderEXT` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkShaderInstrumentationARM` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderInstrumentationARM(pub NonZeroU64);
from_by_transmute!(ShaderInstrumentationARM => sys::VkShaderInstrumentationARM);
impl_raw_handle!(ShaderInstrumentationARM: VK_OBJECT_TYPE_SHADER_INSTRUMENTATION_ARM => sys::VkShaderInstrumentationARM; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(ShaderInstrumentationARM);

impl ShaderInstrumentationARM {
   /// Creates a new `ShaderInstrumentationARM` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `ShaderInstrumentationARM` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkShaderModule` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderModule(pub NonZeroU64);
from_by_transmute!(ShaderModule => sys::VkShaderModule);
impl_raw_handle!(ShaderModule: VK_OBJECT_TYPE_SHADER_MODULE => sys::VkShaderModule; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(ShaderModule);

impl ShaderModule {
   /// Creates a new `ShaderModule` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `ShaderModule` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkSurfaceKHR` : `VkInstance`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SurfaceKHR(pub NonZeroU64);
from_by_transmute!(SurfaceKHR => sys::VkSurfaceKHR);
impl_raw_handle!(SurfaceKHR: VK_OBJECT_TYPE_SURFACE_KHR => sys::VkSurfaceKHR; RawSubHandle = Instance);
impl_debug_for_raw_non_dispatchable_handle!(SurfaceKHR);

impl SurfaceKHR {
   /// Creates a new `SurfaceKHR` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `SurfaceKHR` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkSwapchainKHR` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SwapchainKHR(pub NonZeroU64);
from_by_transmute!(SwapchainKHR => sys::VkSwapchainKHR);
impl_raw_handle!(SwapchainKHR: VK_OBJECT_TYPE_SWAPCHAIN_KHR => sys::VkSwapchainKHR; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(SwapchainKHR);

impl SwapchainKHR {
   /// Creates a new `SwapchainKHR` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `SwapchainKHR` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkTensorARM` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TensorARM(pub NonZeroU64);
from_by_transmute!(TensorARM => sys::VkTensorARM);
impl_raw_handle!(TensorARM: VK_OBJECT_TYPE_TENSOR_ARM => sys::VkTensorARM; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(TensorARM);

impl TensorARM {
   /// Creates a new `TensorARM` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `TensorARM` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkTensorViewARM` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TensorViewARM(pub NonZeroU64);
from_by_transmute!(TensorViewARM => sys::VkTensorViewARM);
impl_raw_handle!(TensorViewARM: VK_OBJECT_TYPE_TENSOR_VIEW_ARM => sys::VkTensorViewARM; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(TensorViewARM);

impl TensorViewARM {
   /// Creates a new `TensorViewARM` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `TensorViewARM` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkValidationCacheEXT` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidationCacheEXT(pub NonZeroU64);
from_by_transmute!(ValidationCacheEXT => sys::VkValidationCacheEXT);
impl_raw_handle!(ValidationCacheEXT: VK_OBJECT_TYPE_VALIDATION_CACHE_EXT => sys::VkValidationCacheEXT; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(ValidationCacheEXT);

impl ValidationCacheEXT {
   /// Creates a new `ValidationCacheEXT` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `ValidationCacheEXT` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkVideoSessionKHR` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoSessionKHR(pub NonZeroU64);
from_by_transmute!(VideoSessionKHR => sys::VkVideoSessionKHR);
impl_raw_handle!(VideoSessionKHR: VK_OBJECT_TYPE_VIDEO_SESSION_KHR => sys::VkVideoSessionKHR; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(VideoSessionKHR);

impl VideoSessionKHR {
   /// Creates a new `VideoSessionKHR` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `VideoSessionKHR` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

/// `VkVideoSessionParametersKHR` : `VkDevice`
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoSessionParametersKHR(pub NonZeroU64);
from_by_transmute!(VideoSessionParametersKHR => sys::VkVideoSessionParametersKHR);
impl_raw_handle!(VideoSessionParametersKHR: VK_OBJECT_TYPE_VIDEO_SESSION_PARAMETERS_KHR => sys::VkVideoSessionParametersKHR; RawSubHandle = Device);
impl_debug_for_raw_non_dispatchable_handle!(VideoSessionParametersKHR);

impl VideoSessionParametersKHR {
   /// Creates a new `VideoSessionParametersKHR` from a raw handle value. Returns `None` if the value is zero.
   pub fn new(raw: u64) -> Option<Self> {
       NonZeroU64::new(raw).map(Self)
   }
   /// Creates a new `VideoSessionParametersKHR` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.
   pub unsafe fn new_unchecked(raw: u64) -> Self {
       Self(unsafe { NonZeroU64::new_unchecked(raw) })
   }
}

