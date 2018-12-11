# underscore_args
Macro for underscore named argumet syntax, aka [Dyon](https://github.com/pistondevelopers/dyon).

This library requires Rust 2018 nightly and only works for functions, not methods.

```rust
#![feature(concat_idents)]

use underscore_args::args;

#[allow(non_snake_case)]
fn line__color_from_to(_color: [f32; 4], _from: [f32; 2], _to: [f32; 2]) {}

args!(line(color: [1.0; 4], from: [0.0, 0.0], to: [100.0, 100.0]));
```
