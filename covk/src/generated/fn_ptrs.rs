// generated file, do not modify manually
#![allow(non_camel_case_types)]

use crate::{vk::*, vk::ffi::*};

/// `PFN_vkAllocationFunction`
pub type PFN_vkAllocationFunction = Option<FN_vkAllocationFunction>;
/// `FN_vkAllocationFunction`
pub type FN_vkAllocationFunction = unsafe extern "system" fn(
    user_data: *mut void, 
    size: size_t, 
    alignment: size_t, 
    allocation_scope: SystemAllocationScope, 
) -> *mut void;

/// `PFN_vkDebugReportCallbackEXT`
pub type PFN_vkDebugReportCallbackEXT = Option<FN_vkDebugReportCallbackEXT>;
/// `FN_vkDebugReportCallbackEXT`
pub type FN_vkDebugReportCallbackEXT = unsafe extern "system" fn(
    flags: DebugReportFlagsEXT, 
    object_type: DebugReportObjectTypeEXT, 
    object: uint64_t, 
    location: size_t, 
    message_code: int32_t, 
    layer_prefix: *const c_char, 
    message: *const c_char, 
    user_data: *mut void, 
) -> Bool;

/// `PFN_vkDebugUtilsMessengerCallbackEXT`
pub type PFN_vkDebugUtilsMessengerCallbackEXT = Option<FN_vkDebugUtilsMessengerCallbackEXT>;
/// `FN_vkDebugUtilsMessengerCallbackEXT`
pub type FN_vkDebugUtilsMessengerCallbackEXT = unsafe extern "system" fn(
    message_severity: DebugUtilsMessageSeverityFlagsEXT, 
    message_types: DebugUtilsMessageTypeFlagsEXT, 
    callback_data: *const DebugUtilsMessengerCallbackDataEXT, 
    user_data: *mut void, 
) -> Bool;

/// `PFN_vkDeviceMemoryReportCallbackEXT`
pub type PFN_vkDeviceMemoryReportCallbackEXT = Option<FN_vkDeviceMemoryReportCallbackEXT>;
/// `FN_vkDeviceMemoryReportCallbackEXT`
pub type FN_vkDeviceMemoryReportCallbackEXT = unsafe extern "system" fn(
    callback_data: *const DeviceMemoryReportCallbackDataEXT, 
    user_data: *mut void, 
) -> ();

/// `PFN_vkFaultCallbackFunction`
pub type PFN_vkFaultCallbackFunction = Option<FN_vkFaultCallbackFunction>;
/// `FN_vkFaultCallbackFunction`
pub type FN_vkFaultCallbackFunction = unsafe extern "system" fn(
    unrecorded_faults: Bool, 
    fault_count: uint32_t, 
    faults: *const FaultData, 
) -> ();

/// `PFN_vkFreeFunction`
pub type PFN_vkFreeFunction = Option<FN_vkFreeFunction>;
/// `FN_vkFreeFunction`
pub type FN_vkFreeFunction = unsafe extern "system" fn(
    user_data: *mut void, 
    memory: *mut void, 
) -> ();

/// `PFN_vkGetInstanceProcAddrLUNARG`
pub type PFN_vkGetInstanceProcAddrLUNARG = Option<FN_vkGetInstanceProcAddrLUNARG>;
/// `FN_vkGetInstanceProcAddrLUNARG`
pub type FN_vkGetInstanceProcAddrLUNARG = unsafe extern "system" fn(
    instance: Instance, 
    name: *const c_char, 
) -> PFN_vkVoidFunction;

/// `PFN_vkInternalAllocationNotification`
pub type PFN_vkInternalAllocationNotification = Option<FN_vkInternalAllocationNotification>;
/// `FN_vkInternalAllocationNotification`
pub type FN_vkInternalAllocationNotification = unsafe extern "system" fn(
    user_data: *mut void, 
    size: size_t, 
    allocation_type: InternalAllocationType, 
    allocation_scope: SystemAllocationScope, 
) -> ();

/// `PFN_vkInternalFreeNotification`
pub type PFN_vkInternalFreeNotification = Option<FN_vkInternalFreeNotification>;
/// `FN_vkInternalFreeNotification`
pub type FN_vkInternalFreeNotification = unsafe extern "system" fn(
    user_data: *mut void, 
    size: size_t, 
    allocation_type: InternalAllocationType, 
    allocation_scope: SystemAllocationScope, 
) -> ();

/// `PFN_vkReallocationFunction`
pub type PFN_vkReallocationFunction = Option<FN_vkReallocationFunction>;
/// `FN_vkReallocationFunction`
pub type FN_vkReallocationFunction = unsafe extern "system" fn(
    user_data: *mut void, 
    original: *mut void, 
    size: size_t, 
    alignment: size_t, 
    allocation_scope: SystemAllocationScope, 
) -> *mut void;

/// `PFN_vkVoidFunction`
pub type PFN_vkVoidFunction = Option<FN_vkVoidFunction>;
/// `FN_vkVoidFunction`
pub type FN_vkVoidFunction = unsafe extern "system" fn() -> ();

