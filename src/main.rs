#[macro_export]
macro_rules! enum_2_field {
    ($self:ident . $side:ident _ $kind:ident) => {
        ::paste::paste! {
            $self.[<$side:snake _ $kind:snake>]
        }
    };
}

fn main() {
    println!("Hello, world!");
}
