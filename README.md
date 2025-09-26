
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

fn do(ab: AB, xy: XY) -> |&Product| -> &str {
    field!(ab,xy)
}

let item = do(B, X)(&product);
assert_eq!(item, "bx");
```

in generic way.

