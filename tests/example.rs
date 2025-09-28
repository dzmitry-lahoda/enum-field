use rust_enums_values_types_fields::{enum_field_match, enum_field_use};

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

enum_field_match!(match_ab_field, [A, B], [X, Y]);

#[derive(Debug)]
pub struct Product {
    pub a_x: String,
    pub a_y: u8,
    pub b_x: String,
    pub b_y: u8,
}

#[test]
fn example() {
    use AB::*;
    use XY::*;

    let mut product = Product {
        a_x: "a_x".to_string(),
        a_y: 1,
        b_x: "b_x".to_string(),
        b_y: 2,
    };

    match_ab_field!(product, AB::B, XY::X, |field| {
        let field: &String = field;
        println!("field: {field}")
    });
    {
        assert_eq!(product.a_x, enum_field_use!(product.A _ X).as_str());
    }
    // {
    //     let a_x = product.a_x.clone();
    //     assert_eq!(a_x, enum_field_use!(mut product.A _ X).as_str());
    //     let setter = enum_field_use!(mut product.A _ X);
    //     *setter = "new_a_x".to_string();
    // }
    // let matcher = get_match!(AB::A, XY::X);
    // assert_eq!(product.a_x, matcher(&product));
    // println!("{product:?}");
    panic!();
}
