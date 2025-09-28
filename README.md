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

allows to

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
- AB and XY enums could be in different crates
- mut variants for field accessors
- AB and XY can be up to 3 variants each and above
- fields can be different(heteregenous) return type
- works on latest stable


## Alternatives

Can functional crates in Rust do above?