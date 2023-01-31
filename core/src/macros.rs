
macro_rules! hook {
    ($name:ident, $addr:expr, ($($args:tt),*), $ret:ty, $func:expr) => {
        static_detour! {
            static $name: fn($( $args )*,) -> $ret;
        }

        paste! {
         unsafe fn [<enable_$name>]() -> detour3::Result<()> {
                let transmuted: fn($( $args )*,) -> $ret = std::mem::transmute($addr);
                $name.initialize(transmuted, $func)?.enable()
            }
        }
    };
    ($name:ident, $addr:expr, $ty:expr, $func:expr) => {
        static_detour! {
            static $name: $ty
        }

        paste! {
         unsafe fn [<enable_$name>]() -> detour3::Result<()> {
                let transmuted: fn($( $args )*,) -> $ret = std::mem::transmute($addr);
                $name.initialize(transmuted, $func)?.enable()
            }
        }
    };
}
