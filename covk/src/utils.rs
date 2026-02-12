use core::fmt::Debug;

pub struct FormatPtr(pub *mut core::ffi::c_void);

impl Debug for FormatPtr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{:p}", self.0))
    }
}

pub struct FormatHex(pub u64);

impl Debug for FormatHex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{:#018x}", self.0))
    }
}
