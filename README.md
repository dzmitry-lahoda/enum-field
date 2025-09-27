
Given

```rust
enum AB { A, B }
enum XY { X, Y }

struct Product {
    pub a_x : String,
    pub a_y : u8,
    pub b_x : String,
    pub b_y : u8,
}
```

allow to

```rust
use AB::*;
use XY::*;
let product = Product { bx: "bx".to_string(), .. }; 
let item = field_use!(product . B _ X);
assert_eq!(item, "bx");

let item = field_match(B, X)(&product);
assert_eq!(item, "bx");
```

in generic way via declarative macro, so that:
- AB and XY enums could be in different crates(or at least modules), so under my full control
- mut variants for accessors
- AB and XY can add more variants in easy way
- fields can be different return type
- works on latest stable

I assume it is possible with declarative macro, but prove of impossiblity also good solution.

## Results

To have heterogenous fileds types need macro export from macro, which is hard on stable.

On top of that, cannot expand `list` of values and then tt muncher, says tt muncher is less deep.

TLRD; Wait for https://github.com/rust-lang/rust/issues/83527 and try again. 