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
let item = enum_field_use!(product . B _ X);
assert_eq!(item, "bx");

let item = enum_field_match_ab(&product.  A _ X, |x| println!("{x}"));
```

in generic way via declarative macro, so that:
- AB and XY enums could be in different crates
- mut variants for field accessors
- AB and XY can be up to 3 variants each and above
- fields can be different(heteregenous) return type
- works on latest stable


but:
- if only shared traits used for all fields types, can compile as is
- if types used directly, but provide coproduct (MyStruct::from) for each field type
  - it is better then doing each field to be enum(coproduct) because cannot set bad field variant

## Alternatives

Can functional crates in Rust do above?