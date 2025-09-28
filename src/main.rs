
// unfortunately could not exand depth and at same time grabbing previous idents (tried tt muncher too)
// also attempt to export macro from macro with many inputs hard without #![feature(macro_metavar_expr)]
#[macro_export]
macro_rules! enum_field_match2 {
    ($name:ident, $this:ty, $ab:ty, $xy:ty, $r:ty; $i1:ident, $i2:ident; $j1:ident ) => {
        fn $name(ab: $ab, xy: $xy) -> impl Fn(&$this) -> $r {
            move |this| match (ab, xy) {
                (<$ab>::$i1, <$xy>::$j1) => &this.a_x,
                (<$ab>::$i2, <$xy>::$j1) => &this.b_x,
            }
        }
    };
    ($name:ident, $this:ty, $ab:ty, $xy:ty, $r:ty; $i1:ident, $i2:ident; $j1:ident, $j2:ident ) => {
        //NOTE: must be macro to handle heteregeneous return types
        fn $name(ab: $ab, xy: $xy) -> impl Fn(&$this) -> $r {
            move |this| match (ab, xy) {
                (<$ab>::$i1, <$xy>::$j1) => field!(this . $i1 _ $j1),
                (<$ab>::$i2, <$xy>::$j1) =>  field!(this . $i2 _ $j1),
                (<$ab>::$i1, <$xy>::$j2) =>  field!(this . $i1 _ $j2),
                (<$ab>::$i2, <$xy>::$j2) =>   field!(this . $i2 _ $j2),
            }
        }
        ::paste::paste! {
            fn [<$name _ mut>](ab: $ab, xy: $xy) -> impl Fn(&$this) -> $r {
                move |this| match (ab, xy) {
                    (<$ab>::$i1, <$xy>::$j1) => field!(this . $i1 _ $j1),
                    (<$ab>::$i2, <$xy>::$j1) =>  field!(this . $i2 _ $j1),
                    (<$ab>::$i1, <$xy>::$j2) =>  field!(this . $i1 _ $j2),
                    (<$ab>::$i2, <$xy>::$j2) =>   field!(this . $i2 _ $j2),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! make_matcher {
    ($name:ident, $fragment_type:ident, $d:tt) => {
        #[macro_export]
        macro_rules! $name {
            ($d _:$fragment_type) => { true };
            (const { 0 }) => { false };
        }
    };
}

make_matcher!(is_expr_from_2021, expr, $);

#[macro_export]
macro_rules! enum_field_match {
    ($name:ident, $a0:ident, $d:tt) => {   
        ::paste::paste! {
            #[macro_export]
            macro_rules! $name {
                ($a:expr) => {
                    match  $a {
                        $a0 => { println!("a0");} 
                    }
                };
            }
        }
    };
    ($name:ident, $a0:ident, $a1:ident, $d:tt) => {   
        ::paste::paste! {
            #[macro_export]
            macro_rules! $name {
                ($a:expr) => {
                    match  $a {
                        $a0 => { println!("a0");} 
                        $a1 => { println!("a1");} 
                    }
                };
            }
        }
    };
    ($name:ident, [$a0:ident, $a1:ident], $b0:ident, $d:tt) => {   
        ::paste::paste! {
            #[macro_export]
            macro_rules! $name {
                ($a:expr,$b:expr) => {
                    match  ($a,$b) {
                        ($a0, $b0) => { println!("a0,b0");} 
                        ($a1, $b0) => { println!("a1,b0");} 
                    }
                };
            }
        }
    };    
}

#[macro_export]
macro_rules! test1 {
    ($a0:ident) => {

         $a0
    };
}

/// Combines one or more unit enums idents to form snake cases enum name.
#[macro_export]
macro_rules! enum_field_use {
    ($self:ident . $a:ident) => {
        ::paste::paste! {
            &$self.[<$a:snake>]
        }
    };    
    (mut $self:ident . $a:ident) => {
        ::paste::paste! {
            &mut $self.[<$a:snake>]
        }
    };        
    ($self:ident . $a:ident _ $b:ident) => {
        ::paste::paste! {
            &$self.[<$a:snake _ $b:snake>]
        }
    };
    (mut $self:ident . $a:ident _ $b:ident) => {
        ::paste::paste! {
            &mut $self.[<$a:snake _ $b:snake>]
        }
    };
    ($self:ident . $a:ident _ $b:ident _ $c:ident) => {
        ::paste::paste! {
            &$self.[<$a:snake _ $b:snake _ $c:snake>]
        }
    };
    (mut $self:ident . $a:ident _ $b:ident _ $c:ident) => {
        ::paste::paste! {
            &mut $self.[<$a:snake _ $b:snake _ $c:snake>]
        }
    };
}

// in theory could make macro arounds enums to get all variants, but seems to intrusive for now (expecially if some enum is very generic for use case)
// enum_field_match!(get_match, Product, AB, XY, &str; A, B; X, Y);

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
    pub a_y: u8,
    pub b_x: String,
    pub b_y: u8,
}




fn main() {
    use AB::*;
    use XY::*;
    enum_field_match!(match_ab_field, [A, B], X, $);
    match_ab_field!(AB::B, XY::X);
    let mut product = Product {
        a_x: "a_x".to_string(),
        a_y: 1,
        b_x: "b_x".to_string(),
        b_y: 2,
    };

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
}
