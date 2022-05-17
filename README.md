# jutils

[![license](https://img.shields.io/crates/l/jutils.svg)](LICENSE)
[![crates.io](https://img.shields.io/crates/d/jutils.svg)](https://crates.io/crates/jutils)
[![version](https://img.shields.io/crates/v/jutils.svg)](https://crates.io/crates/jutils/)
[![documentation](https://docs.rs/jutils/badge.svg)](https://docs.rs/jutils/)

JSON utilities for Rust.

Right now there are two utilities 
- `extend_json_str`: Extends a `Vec<u8>` with a valid JSON String.
- `extend_json_str_fragment`: Extends a `Vec<u8>` with a valid JSON String, but without the surrounding quotes.

For example:

```rust
use jutils:*

my_json.extend_json_str("hello world");
```
Which adds `"hello world"` to the `my_json` vector.

More to come. Stay tuned to this channel...
