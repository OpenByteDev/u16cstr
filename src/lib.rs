#![no_std]

#[macro_export]
macro_rules! u16cstr {
    ($expression:expr) => {
        unsafe { widestring::U16CStr::from_slice_with_nul_unchecked(wchar::wchz!(u16, $expression)) }
    };
}

#[cfg(test)]
mod tests {
    use widestring::{U16CStr, U16CString};

    #[test]
    fn basic() {
        assert_eq!(u16cstr!("Test"), U16CString::from_str("Test").unwrap().as_ref());
    }
}
