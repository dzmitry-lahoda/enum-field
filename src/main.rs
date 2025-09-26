#[macro_export]
macro_rules! field {
    ($self:ident . $side:ident _ $kind:ident) => {
        ::paste::paste! {
            $self.[<$side:snake _ $kind:snake>]
        }
    };
    ($ab:ident, $xy:ident) => {
        move |product| match ($ab, $xy) {
            (AB::A, XY::X) => &product.a_x,
            (AB::A, XY::Y) => &product.a_y,
            (AB::B, XY::X) => &product.b_x,
            (AB::B, XY::Y) => &product.b_y,
        }
    };
}

pub fn get_match(ab: AB, xy: XY) -> impl Fn(&Product) -> &str {
    field!(ab, xy)
}
#[derive(Clone, Copy)]
pub enum AB {
    A,
    B,
}
#[derive(Clone, Copy)]
pub enum XY {
    X,
    Y,
}

#[derive(Debug)]
pub struct Product {
    pub a_x: String,
    pub a_y: String,
    pub b_x: String,
    pub b_y: String,
}

fn main() {
    let product = Product {
        a_x: "a_x".to_string(),
        a_y: "a_y".to_string(),
        b_x: "b_x".to_string(),
        b_y: "b_y".to_string(),
    };

    assert_eq!(product.a_x, field!(product.A _ X));

    let matcher = get_match(AB::A, XY::X);
    assert_eq!(product.a_x, matcher(&product));
    println!("{product:?}");
}
