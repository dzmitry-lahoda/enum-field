
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
- fields could have different return type
- works on latest stable

I assume it is possible with declarative macro, but prove of impossiblity also good solution.
