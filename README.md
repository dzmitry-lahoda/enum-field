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

in a generic way via a declarative macro, so that:
- AB and XY enums could be in different crates
- mut variants for field accessors
- AB and XY can be up to 3 variants each and above
- fields can be different (heterogeneous) return types
- works on latest stable


but:
- if only shared traits are used for all field types, it can compile as-is
- if types are used directly, provide a coproduct (e.g., `MyStruct::from`) for each field type
  - it is better than making each field an enum (coproduct) because you cannot set a bad field variant

## Alternatives

Can functional crates in Rust do the above?
