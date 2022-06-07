use u16cstr::{u16cstr, u16str};
use widestring::{U16CString, U16String};

#[test]
fn u16cstr() {
    assert_eq!(u16cstr!("Test"), U16CString::from_str("Test").unwrap().as_ucstr());
}

#[test]
fn u16str() {
    assert_eq!(u16str!("Test"), U16String::from_str("Test").as_ustr());
}
