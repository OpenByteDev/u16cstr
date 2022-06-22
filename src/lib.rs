#![no_std]

//! A macro for creating c-style u16 wide strings at compile time.
//!
//! ## Example
//! ```rust
//! use u16cstr::{u16cstr, u16str};
//! use widestring::{U16CString, U16String, U16CStr, U16Str};
//!
//! // c-style terminated wide string
//! const wide_c_string: &U16CStr = u16cstr!("Test");
//! assert_eq!(wide_c_string, U16CString::from_str("Test").unwrap().as_ucstr());
//!
//! // non-terminated wide string
//! const wide_string: &U16Str = u16str!("Test");
//! assert_eq!(wide_string, U16String::from_str("Test").as_ustr());
//! ```

pub extern crate widestring;

/// A macro for creating c-style u16 wide strings at compile time.
///
/// ## Example
/// ```rust
/// use u16cstr::u16cstr;
/// use widestring::{U16CString, U16CStr};
///
/// const wide_c_string: &U16CStr = u16cstr!("Test");
/// assert_eq!(wide_c_string, U16CString::from_str("Test").unwrap().as_ucstr());
/// ```
#[macro_export]
macro_rules! u16cstr {
    ($expression:expr) => {{
        $crate::widestring::u16cstr!($expression)
    }};
}

/// A macro for creating u16 wide strings at compile time.
///
/// ## Example
/// ```rust
/// use u16cstr::{u16cstr, u16str};
/// use widestring::{U16String, U16Str};
///
/// const wide_string: &U16Str = u16str!("Test");
/// assert_eq!(wide_string, U16String::from_str("Test").as_ustr());
/// ```
#[macro_export]
macro_rules! u16str {
    ($expression:expr) => {{
        $crate::widestring::u16str!($expression)
    }};
}
