use core::ffi::{c_char, CStr};
use core::fmt::Formatter;
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};

/// A wrapper for fixed-size arrays with dynamic size
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ArraySlice<T, const N: usize> {
    array: [T; N],
    size: usize,
}

impl<T: Copy, const N: usize> ArraySlice<T, N> {
    pub fn new(slice: &[T]) -> Self {
        unsafe {
            let mut array = MaybeUninit::<[T; N]>::zeroed();
            let size = slice.len().min(N);
            core::ptr::copy_nonoverlapping(
                slice.as_ptr(),
                array.assume_init_mut().as_mut_ptr(),
                size,
            );
            Self {
                array: array.assume_init(),
                size,
            }
        }
    }
}

impl<T: Copy, const N: usize> ArraySlice<T, N> {
    pub const unsafe fn from_raw(size: usize, array: [T; N]) -> Self {
        Self { array, size }
    }
    pub const unsafe fn array(&self) -> &[T; N] {
        &self.array
    }
    pub const fn size(&self) -> usize {
        self.size
    }
}

impl<'a, T: Copy, const N: usize> From<&'a [T]> for ArraySlice<T, N> {
    fn from(slice: &'a [T]) -> Self {
        Self::new(slice)
    }
}

impl<T, const N: usize> From<ArraySlice<T, N>> for [T; N] {
    fn from(slice: ArraySlice<T, N>) -> Self {
        slice.array
    }
}

impl<T, const N: usize> Deref for ArraySlice<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.array[..self.size]
    }
}

impl<T, const N: usize> DerefMut for ArraySlice<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.array[..self.size]
    }
}

impl<T, const N: usize> crate::Abi<[T; N]> for ArraySlice<T, N> {
    fn abi(self) -> [T; N] {
        self.array
    }
}

/// A wrapper for fixed-size strings with dynamic size
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ArrayStr<const N: usize> {
    array: [c_char; N],
    // size included null terminator
    size: usize,
}

impl<const N: usize> ArrayStr<N> {
    pub fn new(str: &CStr) -> Self {
        unsafe {
            let mut array = MaybeUninit::<[c_char; N]>::zeroed().assume_init();
            let size = { str.count_bytes() + 1 }.min(N);
            core::ptr::copy_nonoverlapping(str.as_ptr(), array.as_mut_ptr(), size);
            if N > 0 {
                array[N - 1] = 0;
            }
            Self { array, size }
        }
    }

    pub fn from_str(str: &str) -> Self {
        unsafe {
            let mut array = MaybeUninit::<[c_char; N]>::zeroed().assume_init();
            let size = { str.len() + 1 }.min(N);
            core::ptr::copy_nonoverlapping(str.as_ptr().cast(), array.as_mut_ptr(), size);
            if N > 0 {
                array[N - 1] = 0;
            }
            Self { array, size }
        }
    }

    pub fn from_slice(str: &[c_char]) -> Self {
        unsafe {
            let mut array = MaybeUninit::<[c_char; N]>::zeroed().assume_init();
            let size = { str.len() + 1 }.min(N);
            core::ptr::copy_nonoverlapping(str.as_ptr(), array.as_mut_ptr(), size);
            if N > 0 {
                array[N - 1] = 0;
            }
            Self { array, size }
        }
    }

    pub fn from_bytes(str: &[u8]) -> Self {
        unsafe {
            let mut array = MaybeUninit::<[c_char; N]>::zeroed().assume_init();
            let size = { str.len() + 1 }.min(N);
            core::ptr::copy_nonoverlapping(str.as_ptr().cast(), array.as_mut_ptr(), size);
            if N > 0 {
                array[N - 1] = 0;
            }
            Self { array, size }
        }
    }

    pub const unsafe fn from_raw(size: usize, array: [c_char; N]) -> Self {
        Self { array, size }
    }
    pub const fn array(&self) -> &[c_char; N] {
        &self.array
    }
    pub const fn len(&self) -> usize {
        if self.size > 0 { self.size - 1 } else { 0 }
    }
    pub fn as_str(&self) -> &str {
        let len = self.len();
        unsafe { core::mem::transmute(&self.array[..len]) }
    }
}

impl<'a, const N: usize> From<&'a CStr> for ArrayStr<N> {
    fn from(str: &'a CStr) -> Self {
        Self::new(str)
    }
}

impl<'a, const N: usize> From<&'a str> for ArrayStr<N> {
    fn from(str: &'a str) -> Self {
        Self::from_str(str)
    }
}

impl<'a, const N: usize> From<&'a [u8]> for ArrayStr<N> {
    fn from(slice: &'a [u8]) -> Self {
        Self::from_bytes(slice)
    }
}

impl<'a, const N: usize> From<&'a [i8]> for ArrayStr<N> {
    fn from(slice: &'a [i8]) -> Self {
        Self::from_bytes(unsafe { core::mem::transmute(slice) })
    }
}

impl<const N: usize> From<ArrayStr<N>> for [c_char; N] {
    fn from(slice: ArrayStr<N>) -> Self {
        slice.array
    }
}

impl<const N: usize> Deref for ArrayStr<N> {
    type Target = CStr;

    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(&self.array[..self.size]) }
    }
}

impl<const N: usize> DerefMut for ArrayStr<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::mem::transmute(&mut self.array[..self.size]) }
    }
}

impl<const N: usize> core::fmt::Debug for ArrayStr<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<const N: usize> core::fmt::Display for ArrayStr<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<const N: usize> crate::Abi<[c_char; N]> for ArrayStr<N> {
    fn abi(self) -> [c_char; N] {
        self.array
    }
}
