#![no_std]

pub extern crate widestring;
pub extern crate wchar;

#[macro_export]
macro_rules! u16cstr {
    ($expression:expr) => {
        unsafe { $crate::widestring::U16CStr::from_slice_with_nul_unchecked($crate::wchar::wchz!(u16, $expression)) }
    };
}

#[macro_export]
macro_rules! u16str {
    ($expression:expr) => {
        unsafe { $crate::widestring::U16Str::from_slice($crate::wchar::wch!(u16, $expression)) }
    };
}
