# This project has been deprecated as it is now a part of [`widestring`](https://crates.io/crates/widestring/).

# u16cstr

[![Build](https://github.com/OpenByteDev/u16cstr/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/OpenByteDev/u16cstr/actions/workflows/build-and-test.yml)
[![crates.io](https://img.shields.io/crates/v/u16cstr.svg)](https://crates.io/crates/u16cstr)
[![Documentation](https://docs.rs/u16cstr/badge.svg)](https://docs.rs/u16cstr)
[![dependency status](https://deps.rs/repo/github/openbytedev/u16cstr/status.svg)](https://deps.rs/repo/github/openbytedev/u16cstr)
[![MIT](https://img.shields.io/crates/l/u16cstr.svg)](https://github.com/OpenByteDev/u16cstr/blob/master/LICENSE)

A macro for creating c-style u16 wide strings at compile time.

## Example
```rust
use u16cstr::{u16cstr, u16str};
use widestring::{U16CString, U16String, U16CStr, U16Str};

// c-style terminated wide string
const wide_c_string: &U16CStr = u16cstr!("Test");
assert_eq!(wide_c_string, U16CString::from_str("Test").unwrap().as_ucstr());

// non-terminated wide string
const wide_string: &U16Str = u16str!("Test");
assert_eq!(wide_string, U16String::from_str("Test").as_ustr());
```

## License
Licensed under MIT license ([LICENSE](https://github.com/OpenByteDev/u16cstr/blob/master/LICENSE) or http://opensource.org/licenses/MIT)
