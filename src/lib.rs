#![no_std]

pub extern crate widestring;
pub extern crate wchar;

#[macro_export]
macro_rules! u16cstr {
    ($expression:expr) => {
        unsafe { $crate::widestring::U16CStr::from_slice_with_nul_unchecked($crate::wchar::wchz!(u16, $expression)) }
    };
}

#[cfg(test)]
mod tests {
    use widestring::{U16CString};

    #[test]
    fn basic() {
        assert_eq!(u16cstr!("Test"), U16CString::from_str("Test").unwrap().as_ref());
    }
}
