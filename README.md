# jutils

JSON utilities for Rust.

Right now there is one utility `extend_json_str` which extends a `Vec<u8>` with
a valid JSON string.

For example:

```rust
use jutils:*

my_json.extend_json_str("hello world");
```
Which adds `"hello world"` to the `my_json` vector.

*more to come. stay tuned to this channel*
