use paste::paste;

macro_rules! filter_methods {
    ($($name:ident => $value:literal),* $(,)?) => {
        paste! {
            pub trait ServiceFilterExt {
                $(
                    fn [<$name _left>](self) -> String;
                    fn [<$name _right>](self) -> String;
                )*

                fn equal_op(self, other: &str, left: bool) -> String;
            }

            impl ServiceFilterExt for &str {
                $(
                    fn [<$name _left>](self) -> String {
                        self.equal_op($value, true)
                    }

                    fn [<$name _right>](self) -> String {
                        self.equal_op($value, false)
                    }
                )*

                fn equal_op(self, other: &str, left: bool) -> String {
                    if left {
                        format!("{other}={self}")
                    } else {
                        format!("{self}={other}")
                    }
                }
            }
        }
    };
}

filter_methods! {
    sv_name => "name",
    sv_id   => "id",
    sv_mode => "mode",
}
