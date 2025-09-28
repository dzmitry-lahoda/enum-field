
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
let product = Product { b_x: "b_x".to_string(), a_y: 2, .. }; 
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

TLDR; Non intrisuvie macro with manual list of variants for few variants enums was done. 

$$ hack with $dollar cannot form exported macro name via $dollar nor macro can be invoked under other macro to return just boidy without name.

I feel like this handling such case can be done in Haskell, may be via TypeClasses.

In Rust, to have heterogenous fileds types need macro export from macro.

In macro cannot expand `list` of values and then tt muncher, says tt muncher is less deep
nor cannot avoid manual enumartion of units(without being intrusive).

Wait for https://github.com/rust-lang/rust/issues/83527 and try again to improve macro. 