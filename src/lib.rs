/// Generates heterogeneous field accessors by enum variant values.
/// `$name` - name of the generated macro
///
/// Cases:
/// `$a` - access a field by one enum single variant value
///
///
/// [a<1-3>] - comma-separated variants of the first enum
///
///
/// [a<1-3>] - comma-separated variants of the first enum
/// [b<1-3>] - comma-separated variants of the second enum
#[macro_export]
macro_rules! enum_field_match {
    ($name:ident, $a0:ident, $a1:ident) => {
        enum_field_match!($name, $a0, $a1, $);
    };
    ($name:ident, $a0:ident, $a1:ident, $d:tt) => {
        ::paste::paste! {
            #[cfg_attr(feature = "macro-export", macro_export)]
            macro_rules! $name {
                ($this:path,$a:expr, $body:expr) => {
                    match  $a {
                        ($a0) => $body(enum_field_use!($this , $a0)),
                        ($a1) => $body(enum_field_use!($this , $a1)),
                    }
                };
            }
        }
    };
    ($name:ident, [$a0:ident, $a1:ident], $b0:ident) => {
        enum_field_match!($name, [$a0, $a1], $b0, $);
    };
    ($name:ident, [$a0:ident, $a1:ident], $b0:ident, $d:tt) => {
        ::paste::paste! {
            #[cfg_attr(feature = "macro-export", macro_export)]
            macro_rules! $name {
                ($this:path,$a:expr,$b:expr, $body:expr) => {
                    match  ($a,$b) {
                        ($a0, $b0) => $body(enum_field_use!($this , $a0 _ $b0)),
                        ($a1, $b0) => $body(enum_field_use!($this , $a1 _ $b0)),
                    }
                };
            }
        }
    };
    ($name:ident, [$a0:ident, $a1:ident], [$b0:ident,$b1:ident], $coproduct:ty) => {
        enum_field_match!($name, [$a0, $a1], [$b0, $b1], $coproduct, $);
    };
    ($name:ident, [$a0:ident, $a1:ident], [$b0:ident,$b1:ident], $coproduct:ty, $d:tt) => {
        /// Allows accessing a field on `this` by enum values (a, b) within the closure `body`
        #[cfg_attr(feature = "macro-export", macro_export)]
        macro_rules! $name {
            (mut $this:path, $a:ident _ $b:ident <- |$param:ident| $body:expr) => {
                match  ($a,$b) {
                    ($a0, $b0) => {
                        let mut $param = $coproduct::from(enum_field_use!(mut $this , $a0 _ $b0));
                        $body
                    },
                    ($a0, $b1) => {
                        let mut $param = $coproduct::from(enum_field_use!(mut $this , $a0 _ $b1));
                        $body
                    },
                    ($a1, $b0) => {
                        let mut $param = $coproduct::from(enum_field_use!(mut $this , $a1 _ $b0));
                        $body
                    },
                    ($a1, $b1) => {
                        let mut $param = $coproduct::from(enum_field_use!(mut $this , $a1 _ $b1));
                        $body
                    }
                }
            };            
            ($this:path,$a:ident _ $b:ident <- |$param:ident| $body:expr) => {
                match  ($a,$b) {
                    ($a0, $b0) => {
                        let $param = $coproduct::from(enum_field_use!($this , $a0 _ $b0));
                        $body
                    },
                    ($a0, $b1) => {
                        let $param = $coproduct::from(enum_field_use!($this , $a0 _ $b1));
                        $body
                    },
                    ($a1, $b0) => {
                        let $param = $coproduct::from(enum_field_use!($this , $a1 _ $b0));
                        $body
                    },
                    ($a1, $b1) => {
                        let $param = $coproduct::from(enum_field_use!($this , $a1 _ $b1));
                        $body
                    }
                }
            };
        }
    };
    ($name:ident, [$a0:ident, $a1:ident], [$b0:ident,$b1:ident]) => {
        enum_field_match!($name, [$a0, $a1], [$b0, $b1], $);
    };
    ($name:ident, [$a0:ident, $a1:ident], [$b0:ident,$b1:ident], $d:tt) => {
            /// Allows accessing a field on `this` by enum values (a, b) within the closure `body`
            #[cfg_attr(feature = "macro-export", macro_export)]
            macro_rules! $name {
                (mut $this:path,$a:ident _ $b:ident <- |$param:ident| $body:expr) => {
                    match  ($a,$b) {
                        ($a0, $b0) => {
                            let mut $param = enum_field_use!(mut $this , $a0 _ $b0);
                            $body
                        },
                        ($a0, $b1) => {
                            let mut $param = enum_field_use!(mut $this , $a0 _ $b1);
                            $body
                        },
                        ($a1, $b0) => {
                            let mut $param = enum_field_use!(mut $this , $a1 _ $b0);
                            $body
                        },
                        ($a1, $b1) => {
                            let mut $param = enum_field_use!(mut $this , $a1 _ $b1);
                            $body
                        }
                    }
                };                
                ($this:path,$a:ident _ $b:ident <- |$param:ident| $body:expr) => {
                    match  ($a,$b) {
                        ($a0, $b0) => {
                            let $param = enum_field_use!($this , $a0 _ $b0);
                            $body
                        },
                        ($a0, $b1) => {
                            let $param = enum_field_use!($this , $a0 _ $b1);
                            $body
                        },
                        ($a1, $b0) => {
                            let $param = enum_field_use!($this , $a1 _ $b0);
                            $body
                        },
                        ($a1, $b1) => {
                            let $param = enum_field_use!($this , $a1 _ $b1);
                            $body
                        }
                    }
                };
            }
    };
}

// TODO: simplify $$ by using feature(macro_metavar_expr) after stabilization
// NOTE: unfortunately could not expand depth and at the same time grab previous idents (tried tt muncher too)

/// Combines one or more unit-enum idents to form a snake_case field name.
#[macro_export]
macro_rules! enum_field_use {
    (mut $self:path, $a:ident) => {
        ::paste::paste! {
            &mut $self.[<$a:snake>]
        }
    };
    (mut $self:path, $a:ident _ $b:ident) => {
        ::paste::paste! {
            &mut $self.[<$a:snake _ $b:snake>]
        }
    };

    (mut $self:path, $a:ident _ $b:ident _ $c:ident) => {
        ::paste::paste! {
            &mut $self.[<$a:snake _ $b:snake _ $c:snake>]
        }
    };     
    ($self:path, $a:ident) => {
        ::paste::paste! {
            &$self.[<$a:snake>]
        }
    };   
    ($self:path, $a:ident _ $b:ident) => {
        ::paste::paste! {
            &$self.[<$a:snake _ $b:snake>]
        }
    };
    ($self:path, $a:ident _ $b:ident _ $c:ident) => {
        ::paste::paste! {
            &$self.[<$a:snake _ $b:snake _ $c:snake>]
        }
    };
}
