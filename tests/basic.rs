use enum_field::{enum_field_match, enum_field_use};

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

enum_field_match!(match_ab_field, AB { A, B }, XY { X, Y });

enum_field_match!(match_ab_field_co, [A, B], [X, Y], u32);

#[derive(Debug)]
pub struct Product {
    pub a_x: String,
    pub a_y: u8,
    pub b_x: String,
    pub b_y: u8,
}

trait Shared {
    fn casted_to_shared_trait(&self);
    fn casted_to_shared_trait_mut(&mut self);
}

impl Shared for String {
    fn casted_to_shared_trait(&self) {
        println!("String: {self}");
    }

    fn casted_to_shared_trait_mut(&mut self) {
        *self = self.clone() + self.as_str();
    }
}

impl Shared for u8 {
    fn casted_to_shared_trait(&self) {
        println!("u8: {self}");
    }

    fn casted_to_shared_trait_mut(&mut self) {
        *self += *self * 2;
    }
}

#[test]
fn example() {
    use AB::*;
    use XY::*;

    let mut product = Product {
        a_x: "string a_x".to_string(),
        a_y: 1,
        b_x: "string b_x".to_string(),
        b_y: 2,
    };

    let a = A;
    let b = X;

    match_ab_field!(product, a _ b , |selected| selected.casted_to_shared_trait());
    {
        assert_eq!(product.a_x, enum_field_use!(product, A _ X).as_str());
    }
    {
        let a_x = product.a_x.clone();
        assert_eq!(a_x, enum_field_use!(mut product,A _ X).as_str());
        let setter = enum_field_use!(mut product,A _ X);
        *setter = "new_a_x".to_string();
    }

    match_ab_field!(mut product, a _ b , |selected| selected.casted_to_shared_trait_mut());
}
