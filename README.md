
Given

```rust
enum AB { A, B }
enum XY { X, Y }

struct Product {
    pub a_x : String,
    pub a_y : String,
    pub b_x : String,
    pub b_y : String,
}
```

allow to

```rust
use AB::*;
use XY::*;
let product = Product { bx: "bx".to_string(), .. }; 
let item = field!(product . B _ X);
assert_eq!(item, "bx");

/// macro generated method tp match on input enums to access field
fn get_match(ab: AB, xy: XY) -> impl Fn(&Product) -> &String  {
   todo!("...")
}

let item = get_match(B, X)(&product);
assert_eq!(item, "bx");
```

in generic way.

