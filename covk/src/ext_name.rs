use super::*;
use core::ffi::CStr;
use core::hash::Hash;
use core::ops::Deref;

/// Vulkan extension name
#[derive(Clone, Copy, Eq)]
pub struct ExtName<'a>(&'a CStr);

impl<'a> From<&'a CStr> for ExtName<'a> {
    fn from(value: &'a CStr) -> Self {
        Self(value)
    }
}

impl<'a> ExtName<'a> {
    /// Create a new `ExtName` from a CStr. The caller must ensure that the CStr is null-terminated and valid UTF-8.
    pub const unsafe fn new(name: &'a CStr) -> Self {
        Self(name)
    }
    pub const fn as_cstr(self) -> &'a CStr {
        self.0
    }
    pub const fn as_ptr(self) -> *const i8 {
        self.0.as_ptr()
    }
    pub const fn as_str(self) -> &'a str {
        unsafe { core::str::from_utf8_unchecked(self.0.to_bytes()) }
    }
}

impl Deref for ExtName<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl core::fmt::Debug for ExtName<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl core::fmt::Display for ExtName<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl PartialEq for ExtName<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str().eq(other.as_str())
    }
    fn ne(&self, other: &Self) -> bool {
        self.as_str().ne(other.as_str())
    }
}

impl PartialOrd for ExtName<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.as_str().partial_cmp(&other.as_str())
    }

    fn lt(&self, other: &Self) -> bool {
        self.as_str().lt(&other.as_str())
    }

    fn le(&self, other: &Self) -> bool {
        self.as_str().le(&other.as_str())
    }

    fn gt(&self, other: &Self) -> bool {
        self.as_str().gt(&other.as_str())
    }

    fn ge(&self, other: &Self) -> bool {
        self.as_str().ge(&other.as_str())
    }
}

impl PartialEq<str> for ExtName<'_> {
    fn eq(&self, other: &str) -> bool {
        self.as_str().eq(other)
    }
    fn ne(&self, other: &str) -> bool {
        self.as_str().ne(other)
    }
}

impl<'a, 'b> PartialEq<&'a str> for ExtName<'b> {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str().eq(*other)
    }
    fn ne(&self, other: &&'a str) -> bool {
        self.as_str().ne(*other)
    }
}

impl PartialEq<ExtName<'_>> for str {
    fn eq(&self, other: &ExtName<'_>) -> bool {
        other.as_str().eq(self)
    }
    fn ne(&self, other: &ExtName<'_>) -> bool {
        other.as_str().ne(self)
    }
}

impl<'a, 'b> PartialEq<ExtName<'a>> for &'b str {
    fn eq(&self, other: &ExtName<'a>) -> bool {
        other.as_str().eq(*self)
    }
    fn ne(&self, other: &ExtName<'a>) -> bool {
        other.as_str().ne(*self)
    }
}

impl<'a, 'b> PartialEq<&'a CStr> for ExtName<'b> {
    fn eq(&self, other: &&'a CStr) -> bool {
        self.as_cstr().eq(other)
    }
    fn ne(&self, other: &&'a CStr) -> bool {
        self.as_cstr().ne(other)
    }
}

impl PartialEq<CStr> for ExtName<'_> {
    fn eq(&self, other: &CStr) -> bool {
        self.as_cstr().eq(other)
    }
    fn ne(&self, other: &CStr) -> bool {
        self.as_cstr().ne(other)
    }
}

impl<'a, 'b> PartialEq<ExtName<'a>> for &'b CStr {
    fn eq(&self, other: &ExtName<'a>) -> bool {
        other.as_cstr().eq(self)
    }
    fn ne(&self, other: &ExtName<'a>) -> bool {
        other.as_cstr().ne(self)
    }
}

impl PartialEq<ExtName<'_>> for CStr {
    fn eq(&self, other: &ExtName<'_>) -> bool {
        other.as_cstr().eq(self)
    }
    fn ne(&self, other: &ExtName<'_>) -> bool {
        other.as_cstr().ne(self)
    }
}

impl Ord for ExtName<'_> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Hash for ExtName<'_> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}
