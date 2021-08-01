#![no_std]

pub extern crate widestring;
pub extern crate wchar;

#[macro_export]
macro_rules! u16cstr {
    ($expression:expr) => {{
        // the following would be nice to use but it is sadly not const.
        // unsafe { $crate::widestring::U16CStr::from_slice_with_nul_unchecked($crate::wchar::wchz!(u16, $expression)) }

        unsafe { ::core::mem::transmute::<&'static [u16], &'static $crate::widestring::U16CStr>($crate::wchar::wchz!(u16, $expression)) }
    }};
}

#[macro_export]
macro_rules! u16str {
    ($expression:expr) => {{
        // the following would be nice to use but it is sadly not const.
        // unsafe { $crate::widestring::U16Str::from_slice($crate::wchar::wch!(u16, $expression)) }

        unsafe { ::core::mem::transmute::<&'static [u16], &'static $crate::widestring::U16Str>($crate::wchar::wch!(u16, $expression)) }
    }};
}
