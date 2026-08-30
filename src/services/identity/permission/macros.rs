/// Declares the canonical resource marker types.
macro_rules! define_resources {
    ($($name:ident => $value:literal),+ $(,)?) => {
        $(
            pub struct $name;
            impl super::PermissionResource for $name {
                const NAME: &'static str = $value;
            }
        )+
    };
}

/// Declares the canonical operation marker types.
macro_rules! define_operations {
    ($($name:ident => $value:literal),+ $(,)?) => {
        $(
            pub struct $name;
            impl super::PermissionOperation for $name {
                const NAME: &'static str = $value;
            }
        )+
    };
}

pub(super) use {define_operations, define_resources};
