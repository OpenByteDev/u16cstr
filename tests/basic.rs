use u16cstr::u16cstr;
use widestring::U16CString;

#[test]
fn basic() {
    assert_eq!(u16cstr!("Test"), U16CString::from_str("Test").unwrap().as_ref());
}
