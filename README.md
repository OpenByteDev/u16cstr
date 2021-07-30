# u16cstr

[![Build](https://github.com/OpenByteDev/u16cstr/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/OpenByteDev/u16cstr/actions/workflows/build-and-test.yml)
[![crates.io](https://img.shields.io/crates/v/u16cstr.svg)](https://crates.io/crates/u16cstr)
[![Documentation](https://docs.rs/u16cstr/badge.svg)](https://docs.rs/u16cstr)
[![dependency status](https://deps.rs/repo/github/openbytedev/u16cstr/status.svg)](https://deps.rs/repo/github/openbytedev/u16cstr)
[![MIT](https://img.shields.io/crates/l/u16cstr.svg)](https://github.com/OpenByteDev/u16cstr/blob/master/LICENSE)

A macro for creating c-style u16 wide strings at compile time.

## Setup
Add this to your `Cargo.toml`:
```toml
[dependencies]
u16cstr = "0.1"
```

## Example
```rust
use u16cstr::u16cstr;
use widestring::U16CStr;

const wide_string: &U16CStr = u16cstr!("Test");
```

## License
Licensed under MIT license ([LICENSE](https://github.com/OpenByteDev/u16cstr/blob/master/LICENSE) or http://opensource.org/licenses/MIT)
