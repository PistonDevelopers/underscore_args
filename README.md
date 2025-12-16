# underscore_args
Macro for underscore named argumet syntax, aka [Dyon](https://github.com/pistondevelopers/dyon).

By naming a function `line__color_from_to`, one can call it like this:

```ignore
args!(line(color: [1.0; 4], from: [0.0, 0.0], to: [100.0, 100.0]))
```

- Double underscore after function name and before the name of the first argument
- Single underscore separating arguments

### Example

```rust
use underscore_args::args;

#[allow(non_snake_case)]
fn line__color_from_to(_color: [f32; 4], _from: [f32; 2], _to: [f32; 2]) {}

args!(line(color: [1.0; 4], from: [0.0, 0.0], to: [100.0, 100.0]));
```
