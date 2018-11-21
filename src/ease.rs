/*! Ready-to-use easing functions

A set of functions which take a parameter `t`, which is between `0` and `1` (or sometimes `-1` and
`1` and) transforms it to another value between `0` and `1`

 */

use compose::{ flip, mix };

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

macro_rules! flip_hv {
    ($f:ident, $t:ident) => {
        flip($f(flip($t)))
    }
}

// Smooth stop functions
def_ease! {
    /// _1 - (1 - t)^2_
    smooth_stop_2 t => flip_hv!(smooth_start_2, t),

    /// _1 - (1 - t)^3_
    smooth_stop_3 t => flip_hv!(smooth_start_3, t),

    /// _1 - (1 - t)^4_
    smooth_stop_4 t => flip_hv!(smooth_start_4, t),

    /// _1 - (1 - t)^5_
    smooth_stop_5 t => flip_hv!(smooth_start_5, t),

    /// _1 - (1 - t)^6_
    smooth_stop_6 t => flip_hv!(smooth_start_6, t),

    /// _1 - (1 - t)^7_
    smooth_stop_7 t => flip_hv!(smooth_start_7, t),

    /// _1 - (1 - t)^8_
    smooth_stop_8 t => flip_hv!(smooth_start_8, t),

    /// _1 - (1 - t)^9_
    smooth_stop_9 t => flip_hv!(smooth_start_9, t),

    /// _1 - (1 - t)^i_
    smooth_stop_i i, t => flip(smooth_start_i(i, flip(t)))
}

// Define smooth step functions

macro_rules! def_smooth_step {
    ($( $(#[$attr:meta])* $name:ident [$a:ident $b:ident] ),+) => {
        $(
            def_ease!($name t => mix($a(t), $b(t), t));
        )+
    }
}

def_smooth_step! {
    /// Slowest at the start and at the end, fastest in the center
    smooth_step_2 [smooth_start_2 smooth_stop_2],

    /// Slowest at the start and at the end, fastest in the center
    smooth_step_3 [smooth_start_3 smooth_stop_3],

    /// Slowest at the start and at the end, fastest in the center
    smooth_step_4 [smooth_start_4 smooth_stop_4],

    /// Slowest at the start and at the end, fastest in the center
    smooth_step_5 [smooth_start_5 smooth_stop_5],

    /// Slowest at the start and at the end, fastest in the center
    smooth_step_6 [smooth_start_6 smooth_stop_6],

    /// Slowest at the start and at the end, fastest in the center
    smooth_step_7 [smooth_start_7 smooth_stop_7],

    /// Slowest at the start and at the end, fastest in the center
    smooth_step_8 [smooth_start_8 smooth_stop_8],

    /// Slowest at the start and at the end, fastest in the center
    smooth_step_9 [smooth_start_9 smooth_stop_9]
}
