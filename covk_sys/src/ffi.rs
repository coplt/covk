#![allow(non_camel_case_types)]
#![allow(unused)]

use core::ffi::*;

pub use core::ffi::c_char;

pub type void = c_void;
pub type float = f32;
pub type double = f64;
pub type int8_t = i8;
pub type uint8_t = u8;
pub type int16_t = i16;
pub type uint16_t = u16;
pub type int32_t = i32;
pub type uint32_t = u32;
pub type int64_t = i64;
pub type uint64_t = u64;
pub type size_t = usize;
pub type int = c_int;

pub type RROutput = c_ulong;
pub type VisualID = c_uint;
pub type Display = c_void;
pub type Window = c_ulong;
pub type xcb_connection_t = c_void;
pub type xcb_window_t = u32;
pub type xcb_visualid_t = u32;
pub type MirConnection = *const c_void;
pub type MirSurface = *const c_void;
/// <https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Foundation/struct.HANDLE.html>
pub type HANDLE = isize;
/// <https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Foundation/struct.HINSTANCE.html>
pub type HINSTANCE = HANDLE;
/// <https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Foundation/struct.HWND.html>
pub type HWND = HANDLE;
/// <https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/Gdi/struct.HMONITOR.html>
pub type HMONITOR = HANDLE;
pub type wl_display = c_void;
pub type wl_surface = c_void;
pub type DWORD = c_ulong;
pub type LPCWSTR = *const u16;
pub type zx_handle_t = u32;
pub type _screen_buffer = c_void;
pub type _screen_context = c_void;
pub type _screen_window = c_void;
pub type SECURITY_ATTRIBUTES = c_void;

// Opaque types
pub type ANativeWindow = c_void;
pub type AHardwareBuffer = c_void;
pub type CAMetalLayer = c_void;
pub type GgpStreamDescriptor = u32;
pub type GgpFrameToken = u64;
pub type IDirectFB = c_void;
pub type IDirectFBSurface = c_void;
pub type __IOSurface = c_void;
pub type IOSurfaceRef = *mut __IOSurface;
pub type MTLBuffer_id = *mut c_void;
pub type MTLCommandQueue_id = *mut c_void;
pub type MTLDevice_id = *mut c_void;
pub type MTLSharedEvent_id = *mut c_void;
pub type MTLTexture_id = *mut c_void;
pub type OHNativeWindow = c_void;
pub type OHBufferHandle = c_void;
pub type OH_NativeBuffer = c_void;
pub type NvSciSyncAttrList = *mut c_void;
pub type NvSciBufAttrList = *mut c_void;
pub type NvSciBufObj = *mut c_void;
pub type NvSciSyncObj = *mut c_void;
pub type NvSciSyncFence = c_void;
pub type ubm_device = c_void;
pub type ubm_surface = c_void;
pub type VkRemoteAddressNV = *mut c_void;
