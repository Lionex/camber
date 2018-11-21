/*! Ready-to-use easing functions

A set of functions which take a parameter `t`, which is between `0` and `1` (or sometimes `-1` and
`1` and) transforms it to another value between `0` and `1`

 */

use compose::{ flip };

macro_rules! def_ease {
    ( $( $(#[$attr:meta])* $name:ident $($t:ident),+ => $expr:expr ),+ ) => {
        $(
            $(#[$attr])*
            #[inline]
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

// Smooth stop functions
def_ease! {
    /// _1 - (1 - t)^2_
    smooth_stop_2 t => flip(smooth_start_2(flip(t))),

    /// _1 - (1 - t)^3_
    smooth_stop_3 t => flip(smooth_start_3(flip(t))),

    /// _1 - (1 - t)^4_
    smooth_stop_4 t => flip(smooth_start_4(flip(t))),

    /// _1 - (1 - t)^5_
    smooth_stop_5 t => flip(smooth_start_5(flip(t))),

    /// _1 - (1 - t)^6_
    smooth_stop_6 t => flip(smooth_start_6(flip(t))),

    /// _1 - (1 - t)^7_
    smooth_stop_7 t => flip(smooth_start_7(flip(t))),

    /// _1 - (1 - t)^8_
    smooth_stop_8 t => flip(smooth_start_8(flip(t))),

    /// _1 - (1 - t)^9_
    smooth_stop_9 t => flip(smooth_start_9(flip(t))),

    /// _1 - (1 - t)^i_
    smooth_stop_i i, t => flip(smooth_start_i(i, flip(t)))
}
