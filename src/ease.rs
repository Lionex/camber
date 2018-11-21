/*!

# Easing Functions

A set of functions which take a parameter `t`, which is between `0` and `1` (or sometimes `-1` and
`1` and) transforms it to another value between `0` and `1`

 */

macro_rules! def_ease {
    ( $( $(#[$attr:meta])* $name:ident $($t:ident),+ => $expr:expr ),+ ) => {
        $(
            $(#[$attr])*
            pub fn $name(
                $($t: f64,)+
            ) -> f64 {
                $expr
            }
        )+
    }
}

// Smooth Start functions
def_ease!{
    /// _t^2_
    smooth_start_2 t => t*t,

    /// _t^3_
    smooth_start_3 t => t*t*t,

    /// _t^4_
    smooth_start_4 t => t*t*t*t,

    /// _t^5_
    smooth_start_5 t => t*t*t*t*t,

    /// _t^6_
    smooth_start_6 t => t*t*t*t*t*t,

    /// _t^7_
    smooth_start_7 t => t.powi(7),

    /// _t^8_
    smooth_start_8 t => t.powi(8),

    /// _t^9_
    smooth_start_9 t => t.powi(9),

    /// _t^i_
    smooth_start_i i, t => t.powf(i)
}
