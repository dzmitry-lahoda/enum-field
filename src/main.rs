#[macro_export]
macro_rules! field {
    ($self:ident . $side:ident _ $kind:ident) => {
        ::paste::paste! {
            $self.[<$side:snake _ $kind:snake>]
        }
    };
}

pub enum AB { A, B }
pub enum XY { X, Y }

pub struct Product {
    pub a_x : String,
    pub a_y : String,
    pub b_x : String,
    pub b_y : String,
}

fn main() {
    let product = Product {
        a_x: "a_x".to_string(),
        a_y: "a_y".to_string(),
        b_x: "b_x".to_string(),
        b_y: "b_y".to_string(),
    };

    assert_eq!(product.a_x, field!(product.A _ X));
}
