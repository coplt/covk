#![no_std]
#![allow(unpredictable_function_pointer_comparisons)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused)]
#![allow(rustdoc::broken_intra_doc_links)]

extern crate alloc;

#[macro_export]
/// Create ExtName from literal UTF-8 string with a trailing null terminator.
///
/// ```
/// # use ::core::ffi::CStr;
/// # use covk::e;
/// let ext_name: ExtName<'_> = e!("VK_EXT_test");
/// assert_eq!(
///     ext_name.as_cstr(),
///     CStr::from_bytes_with_nul(b"VK_EXT_test\0").unwrap()
/// );
/// ```
macro_rules! e {
    ( $s:literal ) => {
        unsafe { $crate::ExtName::new($crate::c!($s)) }
    };
}

macro_rules! impl_union_debug {
    ( $name:ident <'a> ) => {
        impl<'a> ::core::fmt::Debug for $name<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(stringify!($name)).finish_non_exhaustive()
            }
        }
    };
    ( $name:ident ) => {
        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(stringify!($name)).finish_non_exhaustive()
            }
        }
    };
}

macro_rules! from_into_transparent {
    ( $t:ty : $u:ty ) => {
        impl From<$t> for $u {
            fn from(value: $t) -> Self {
                value.0
            }
        }
        impl From<$u> for $t {
            fn from(value: $u) -> Self {
                Self(value)
            }
        }
    };
}

macro_rules! from_by_transmute {
    ( <'a> $t:ty => $u:ty ) => {
        impl<'a> From<$t> for $u {
            fn from(value: $t) -> Self {
                unsafe { ::core::mem::transmute(value) }
            }
        }
    };
    ( $t:ty => $u:ty ) => {
        impl From<$t> for $u {
            fn from(value: $t) -> Self {
                unsafe { ::core::mem::transmute(value) }
            }
        }
    };
}

macro_rules! flags {
    ( $t:ty ) => {
        impl $t {
            /// `0`
            pub const fn empty() -> Self {
                Self(0)
            }
            /// return is `0`
            pub const fn is_empty(self) -> bool {
                self.0 == 0
            }
            /// return is all bits in `flags` are set
            pub const fn has_flags(self, flags: Self) -> bool {
                (self.0 & flags.0) == flags.0
            }
            /// return is any bits in `flags` are set
            pub const fn has_any_flags(self, flags: Self) -> bool {
                (self.0 & flags.0) != 0
            }
            /// return is only bits in `flags` are set
            pub const fn has_flags_only(self, flags: Self) -> bool {
                (self.0 & !flags.0) == 0
            }
        }
        impl ::core::ops::Not for $t {
            type Output = Self;

            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }
        impl ::core::ops::BitOr for $t {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }
        impl ::core::ops::BitAnd for $t {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }
        impl ::core::ops::BitXor for $t {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }
        impl ::core::ops::BitAndAssign for $t {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }
        impl ::core::ops::BitOrAssign for $t {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }
        impl ::core::ops::BitXorAssign for $t {
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0;
            }
        }
    };
}

macro_rules! impl_raw_handle {
    ( $t:ty : $objtyp:ident => $sys:ty; $det:ident $(= $parent:ty)? ) => {
        impl $crate::ObjectType for $t{
            const TYPE: $crate::vk::ObjectType = $crate::vk::ObjectType($crate::sys::$objtyp.0);
        }
        impl $crate::RawHandle for $t { }
        impl $crate::$det for $t {
            $(type Parent = $parent;)?
        }
        unsafe impl Send for $t {}
        unsafe impl Sync for $t {}
        impl<'a> $crate::Vk<'a> for $sys {
            type Target = Option<$t>;

            fn vk(self) -> Self::Target {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl<'a> $crate::Sys<'a> for $t {
            type Target = $sys;

            fn sys(self) -> Self::Target {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl<'a> $crate::Sys<'a> for Option<$t> {
            type Target = $sys;

            fn sys(self) -> Self::Target {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl $crate::Abi<$sys> for $t {
            fn abi(self) -> $sys {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl $crate::Abi<$sys> for Option<$t> {
            fn abi(self) -> $sys {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl $crate::Abi<Option<$t>> for $t {
            fn abi(self) -> Option<$t> {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl<'a> $crate::Abi<*const $sys> for &'a $t {
            fn abi(self) -> *const $sys {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl<'a> $crate::Abi<*const $sys> for &'a [$t] {
            fn abi(self) -> *const $sys {
                unsafe { self.as_ptr().cast() }
            }
        }
        impl<'a> $crate::Abi<*const $sys> for &'a Option<$t> {
            fn abi(self) -> *const $sys {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl<'a> $crate::Abi<*const $sys> for &'a [Option<$t>] {
            fn abi(self) -> *const $sys {
                unsafe { self.as_ptr().cast() }
            }
        }
        impl<'a> $crate::Abi<*mut $sys> for &'a mut Option<$t> {
            fn abi(self) -> *mut $sys {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl<'a> $crate::Abi<*mut $sys> for &'a mut [Option<$t>] {
            fn abi(self) -> *mut $sys {
                unsafe { self.as_mut_ptr().cast() }
            }
        }
        impl<'a> $crate::Abi<&'a Option<$t>> for &'a $t {
            fn abi(self) -> &'a Option<$t> {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl<'a> $crate::Abi<*const Option<$t>> for &'a $t {
            fn abi(self) -> *const Option<$t> {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl<'a> $crate::Abi<*const Option<$t>> for &'a [$t] {
            fn abi(self) -> *const Option<$t> {
                self.as_ptr().cast()
            }
        }
        impl<const N: usize> $crate::Abi<[Option<$t>; N]> for $crate::ArraySlice<$t, N> {
            fn abi(self) -> [Option<$t>; N] {
                unsafe { ::core::mem::transmute_copy(self.array()) }
            }
        }
        impl<'a> $crate::Abi<*const $sys> for Option<&'a [Option<$t>]> {
            fn abi(self) -> *const $sys {
                unsafe { self.map(|a| a.as_ptr()).unwrap_or_default().cast() }
            }
        }
        impl<'a> $crate::Abi<*mut $sys> for Option<&'a mut [Option<$t>]> {
            fn abi(self) -> *mut $sys {
                unsafe { self.map(|a| a.as_mut_ptr()).unwrap_or_default().cast() }
            }
        }
        impl<'a> $crate::Abi<*const $sys> for Option<&'a [$t]> {
            fn abi(self) -> *const $sys {
                unsafe { self.map(|a| a.as_ptr()).unwrap_or_default().cast() }
            }
        }
        impl<'a> $crate::Abi<*mut $sys> for Option<&'a mut [$t]> {
            fn abi(self) -> *mut $sys {
                unsafe { self.map(|a| a.as_mut_ptr()).unwrap_or_default().cast() }
            }
        }
    };
}

macro_rules! impl_debug_for_raw_dispatchable_handle {
    ( $t:ty) => {
        impl ::core::fmt::Debug for $t {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_tuple(stringify!($t))
                    .field(&$crate::utils::FormatPtr(self.0.as_ptr()))
                    .finish()
            }
        }
        impl ::core::fmt::Pointer for $t {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{:p}", self.0.as_ptr())
            }
        }
    };
}

macro_rules! impl_debug_for_raw_non_dispatchable_handle {
    ( $t:ty) => {
        impl ::core::fmt::Debug for $t {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_tuple(stringify!($t))
                    .field(&$crate::utils::FormatHex(self.0.get()))
                    .finish()
            }
        }
        impl ::core::fmt::Pointer for $t {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{:#018x}", self.0.get())
            }
        }
    };
}

macro_rules! impl_enum_display {
    ( $t:ty = $sys:ident { $($item:ident),* } ) => {
        impl ::core::fmt::Display for $t
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
            {
                sys::$sys(self.0).fmt(f)
            }
        }
        impl ::core::fmt::Debug for $t {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match *self {
                    $(
                        Self::$item => write!(f, "{}::{}", stringify!($t), stringify!($item)),
                    )*
                    _ => write!(f, "{}({})", stringify!($t), self.0),
                }
            }
        }
    };
}

macro_rules! impl_flags_display {
    ( $t:ident = $sys:ident { $($item:ident),* } ) => {
        impl ::core::fmt::Display for $t
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
            {
                sys::$sys(self.0).fmt(f)
            }
        }
        impl ::core::fmt::Debug for $t {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                if self.is_empty() {
                    return write!(f, "{}(0)", stringify!($t));
                }
                let mut first = true;
                $(
                    if self.has_flags(Self::$item)
                    {
                        if first { first = false; } else { write!(f, " | ")?; }
                        write!(f, "{}::{}", stringify!($t), stringify!($item))?;
                    }
                )*
                const ALL_FLAGS: $t = const { $t ( 0 $(| $t::$item.0)* ) };
                if ALL_FLAGS.is_empty() {
                    return write!(f, "0");
                }
                let remaining = *self & !ALL_FLAGS;
                if !remaining.is_empty() {
                    if !first {
                        write!(f, " | ")?;
                    }
                    write!(f, "{}(0x{:#x})", stringify!($t), remaining.0)?;
                }
                Ok(())
            }
        }
    };
}

macro_rules! impl_to_sys {
    ( $t:ty => $sys:ty ) => {
        impl<'a> $crate::Sys<'a> for $t {
            type Target = $sys;
            fn sys(self) -> $sys {
                self.into()
            }
        }
        impl<'a> $crate::Vk<'a> for $sys {
            type Target = $t;
            fn vk(self) -> $t {
                self.into()
            }
        }
    };
}

macro_rules! impl_enum
{
    { $t:ty => $sys:ty } =>
    {
        impl $crate::Abi<$sys> for $t {
            fn abi(self) -> $sys {
                self.into()
            }
        }
        impl<'a> $crate::Abi<*const $sys> for &'a $t {
            fn abi(self) -> *const $sys {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl<'a> $crate::Abi<*mut $sys> for &'a mut $t {
            fn abi(self) -> *mut $sys {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl<'a> $crate::Abi<*const $sys> for &'a [$t] {
            fn abi(self) -> *const $sys {
                unsafe { self.as_ptr().cast() }
            }
        }
        impl<'a> $crate::Abi<*mut $sys> for &'a mut [$t] {
            fn abi(self) -> *mut $sys {
                unsafe { self.as_mut_ptr().cast() }
            }
        }
        impl<'a> $crate::Abi<*const $sys> for Option<&'a $t> {
            fn abi(self) -> *const $sys {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl<'a> $crate::Abi<*mut $sys> for Option<&'a mut $t> {
            fn abi(self) -> *mut $sys {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl<'a> $crate::Abi<*const $sys> for Option<&'a [$t]> {
            fn abi(self) -> *const $sys {
                unsafe { self.map(|a| a.as_ptr()).unwrap_or_default().cast() }
            }
        }
        impl<'a> $crate::Abi<*mut $sys> for Option<&'a mut [$t]> {
            fn abi(self) -> *mut $sys {
                unsafe { self.map(|a| a.as_mut_ptr()).unwrap_or_default().cast() }
            }
        }
    }
}

macro_rules! impl_raw_struct
{
    { $name:ident <'a> => $sys:ty } => {
        from_by_transmute!( <'a> $name <'a> => $sys );
        from_by_transmute!( <'a> $sys => $name <'a> );
        impl_to_sys!( $name <'a> => $sys );
        impl<'a> $crate::Abi<$sys> for $name <'a> {
            fn abi(self) -> $sys {
                self.into()
            }
        }
        impl<'a> $crate::Abi<*const $sys> for *const $name <'a> {
            fn abi(self) -> *const $sys {
                unsafe { self.cast() }
            }
        }
        impl<'a> $crate::Abi<*mut $sys> for *mut $name <'a> {
            fn abi(self) -> *mut $sys {
                unsafe { self.cast() }
            }
        }
        impl<'a, 'b> $crate::Abi<*const $sys> for &'b $name <'a> {
            fn abi(self) -> *const $sys {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl<'a, 'b> $crate::Abi<*mut $sys> for &'b mut $name <'a> {
            fn abi(self) -> *mut $sys {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl<'a, 'b> $crate::Abi<*const $sys> for &'b [$name <'a>] {
            fn abi(self) -> *const $sys {
                unsafe { self.as_ptr().cast() }
            }
        }
        impl<'a, 'b> $crate::Abi<*mut $sys> for &'b mut [$name <'a>] {
            fn abi(self) -> *mut $sys {
                unsafe { self.as_mut_ptr().cast() }
            }
        }
        impl<'a, 'b> $crate::Abi<*const $sys> for Option<&'b $name <'a>> {
            fn abi(self) -> *const $sys {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl<'a, 'b> $crate::Abi<*mut $sys> for Option<&'b mut $name <'a>> {
            fn abi(self) -> *mut $sys {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl<'a, 'b> $crate::Abi<*const $sys> for Option<&'b [$name <'a>]> {
            fn abi(self) -> *const $sys {
                unsafe { self.map(|a| a.as_ptr()).unwrap_or_default().cast() }
            }
        }
        impl<'a, 'b> $crate::Abi<*mut $sys> for Option<&'b mut [$name <'a>]> {
            fn abi(self) -> *mut $sys {
                unsafe { self.map(|a| a.as_mut_ptr()).unwrap_or_default().cast() }
            }
        }
        impl<'a> Default for $name <'a> {
            fn default() -> Self {
                <$sys as Default>::default().into()
            }
        }
    };
    { $name:ident => $sys:ty } => {
        from_by_transmute!( $name => $sys );
        from_by_transmute!( $sys => $name );
        impl_to_sys!( $name => $sys );
        impl $crate::Abi<$sys> for $name {
            fn abi(self) -> $sys {
                self.into()
            }
        }
        impl $crate::Abi<*const $sys> for *const $name {
            fn abi(self) -> *const $sys {
                unsafe { self.cast() }
            }
        }
        impl $crate::Abi<*mut $sys> for *mut $name {
            fn abi(self) -> *mut $sys {
                unsafe { self.cast() }
            }
        }
        impl<'a> $crate::Abi<*const $sys> for &'a $name {
            fn abi(self) -> *const $sys {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl<'a> $crate::Abi<*mut $sys> for &'a mut $name {
            fn abi(self) -> *mut $sys {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl<'a> $crate::Abi<*const $sys> for &'a [$name] {
            fn abi(self) -> *const $sys {
                unsafe { self.as_ptr().cast() }
            }
        }
        impl<'a> $crate::Abi<*const $sys> for Option<&'a $name> {
            fn abi(self) -> *const $sys {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl<'a> $crate::Abi<*mut $sys> for Option<&'a mut $name> {
            fn abi(self) -> *mut $sys {
                unsafe { ::core::mem::transmute(self) }
            }
        }
        impl<'a> $crate::Abi<*mut $sys> for &'a mut [$name] {
            fn abi(self) -> *mut $sys {
                unsafe { self.as_mut_ptr().cast() }
            }
        }
        impl<'a> $crate::Abi<*const $sys> for Option<&'a [$name]> {
            fn abi(self) -> *const $sys {
                unsafe { self.map(|a| a.as_ptr()).unwrap_or_default().cast() }
            }
        }
        impl<'a> $crate::Abi<*mut $sys> for Option<&'a mut [$name]> {
            fn abi(self) -> *mut $sys {
                unsafe { self.map(|a| a.as_mut_ptr()).unwrap_or_default().cast() }
            }
        }
        impl<'a> $crate::Abi<*const *const $sys> for &'a [*const $name] {
            fn abi(self) -> *const *const $sys {
                unsafe { self.as_ptr().cast() }
            }
        }
        impl<'a> $crate::Abi<*const *const $sys> for Option<&'a [*const $name]> {
            fn abi(self) -> *const *const $sys {
                unsafe { self.map(|a| a.as_ptr()).unwrap_or_default().cast() }
            }
        }
        impl Default for $name {
            fn default() -> Self {
                <$sys as Default>::default().into()
            }
        }
    };
}

macro_rules! impl_raw_struct_s_type
{
    { $name:ident <'a> : $typ:ident } => {
        impl<'a> $crate::StructType for $name <'a> {
            #[doc = concat!("`", stringify!($typ), "`")]
            const TYPE: $crate::vk::StructureType = $crate::vk::StructureType::$typ;
        }
        impl<'a> $crate::Chainable for $name <'a>
        {
            fn p_next(&self) -> &*const ::core::ffi::c_void {
                unsafe { ::core::mem::transmute(&self.p_next) }
            }
            fn p_next_mut(&mut self) -> &mut *mut ::core::ffi::c_void {
                unsafe { ::core::mem::transmute(&mut self.p_next) }
            }
        }
    };
    { $name:ident : $typ:ident } => {
        impl $crate::StructType for $name {
            #[doc = concat!("`", stringify!($typ), "`")]
            const TYPE: $crate::vk::StructureType = $crate::vk::StructureType::$typ;
        }
        impl $crate::Chainable for $name
        {
            fn p_next(&self) -> &*const ::core::ffi::c_void {
                unsafe { ::core::mem::transmute(&self.p_next) }
            }
            fn p_next_mut(&mut self) -> &mut *mut ::core::ffi::c_void {
                unsafe { ::core::mem::transmute(&mut self.p_next) }
            }
        }
    };
}

macro_rules! impl_extends {
    { <'a> $this:ty : $($ex:ty),+ } => {
        $(
            impl<'a> $crate::Extend<$ex> for $this {}
        )+
    };
    { $this:ty : $($ex:ty),+ } => {
        $(
            impl $crate::Extend<$ex> for $this {}
        )+
    };
}

macro_rules! impl_object {
    { $name:ident ($scope:ty) } => {
        impl<S: crate::HndScope<$scope>> Clone for $name<S>
            where S::Impl: Clone
        {
            fn clone(&self) -> Self
            {
                Self(self.0.clone())
            }
        }
        impl<S: crate::HndScope<$scope>> ::core::fmt::Debug for $name<S>
            where S::Impl: ::core::fmt::Pointer
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
            {
                f.write_fmt(format_args!("{}({:p})", stringify!($name), self.0))
            }
        }
        impl<S: crate::HndScope<$scope>> ::core::fmt::Pointer for $name<S>
            where S::Impl: ::core::fmt::Pointer
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
            {
                self.0.fmt(f)
            }
        }
        impl<S: crate::HndScope<$scope>> crate::MakeExt for $name<S> { }
    };
}

macro_rules! impl_ext {
    { $name:ident } => {
        impl ::core::ops::Deref for $name {
            type Target = crate::ExtName<'static>;

            fn deref(&self) -> &Self::Target {
                &Self::NAME
            }
        }
        impl ::core::fmt::Debug for $name {
           fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
               Self::NAME.fmt(f)
           }
        }
        impl ::core::fmt::Display for $name {
           fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
               Self::NAME.fmt(f)
           }
        }
        impl PartialEq<str> for $name {
           fn eq(&self, other: &str) -> bool {
               self.as_str().eq(other)
           }
           fn ne(&self, other: &str) -> bool {
               self.as_str().ne(other)
           }
        }
        impl PartialEq<::core::ffi::CStr> for $name {
           fn eq(&self, other: &::core::ffi::CStr) -> bool {
               self.as_cstr().eq(other)
           }
           fn ne(&self, other: &::core::ffi::CStr) -> bool {
               self.as_cstr().ne(other)
           }
        }
        impl PartialEq<$name> for str {
            fn eq(&self, other: &$name) -> bool {
                other.as_str().eq(self)
            }
            fn ne(&self, other: &$name) -> bool {
                other.as_str().ne(self)
            }
        }
        impl PartialEq<$name> for ::core::ffi::CStr {
            fn eq(&self, other: &$name) -> bool {
                other.as_cstr().eq(self)
            }
            fn ne(&self, other: &$name) -> bool {
                other.as_cstr().ne(self)
            }
        }
        impl<'a> PartialEq<&'a str> for $name {
           fn eq(&self, other: &&'a str) -> bool {
               self.as_str().eq(*other)
           }
           fn ne(&self, other: &&'a str) -> bool {
               self.as_str().ne(*other)
           }
        }
        impl<'a> PartialEq<&'a ::core::ffi::CStr> for $name {
           fn eq(&self, other: &&'a ::core::ffi::CStr) -> bool {
               self.as_cstr().eq(*other)
           }
           fn ne(&self, other: &&'a ::core::ffi::CStr) -> bool {
               self.as_cstr().ne(*other)
           }
        }
        impl<'a> PartialEq<$name> for &'a str {
            fn eq(&self, other: &$name) -> bool {
                other.as_str().eq(*self)
            }
            fn ne(&self, other: &$name) -> bool {
                other.as_str().ne(*self)
            }
        }
        impl<'a> PartialEq<$name> for &'a ::core::ffi::CStr {
            fn eq(&self, other: &$name) -> bool {
                other.as_cstr().eq(*self)
            }
            fn ne(&self, other: &$name) -> bool {
                other.as_cstr().ne(*self)
            }
        }
    };
}

mod arr;
mod ext_name;
mod generated;
mod handle;
mod raii_hnd;
mod result;
mod traits;
mod unique;
pub(crate) mod utils;
/// Vulkan for rust
pub mod vk;
mod vulkan;

/// Vulkan bindings
pub mod sys {
    pub use covk_sys::*;
}
/// Vulkan prelude
pub mod prelude {
    pub use crate::generated::prelude::*;
    pub use crate::{
        Chainable, CoreCommandBuffer, CoreDevice, CoreInstance, CorePhysicalDevice, CoreQueue,
        Extend, MakeExt, MakeHnd, ObjectType, StructType, Sys, Vk, Vulkan, vk,
    };
}

pub use arr::*;
pub use commands::{CoreCommandBuffer, CoreDevice, CoreInstance, CorePhysicalDevice, CoreQueue};
pub use ext_name::*;
pub use generated::extensions;
pub use generated::hnd;
pub use generated::main_commands as commands;
pub use generated::new;
pub use loader::ProcAddr;
pub use raii_hnd::*;
pub use result::*;
pub use sys::c;
pub use sys::loader;
pub use traits::*;
pub use unique::*;
pub use uuid::Uuid;
pub use vulkan::*;

#[cfg(test)]
mod test_e {
    use super::*;
    use ::core::ffi::CStr;

    #[test]
    fn test_e() {
        let ext_name = e!("VK_EXT_test");
        assert_eq!(
            ext_name.as_cstr(),
            CStr::from_bytes_with_nul(b"VK_EXT_test\0").unwrap()
        );
    }
}
