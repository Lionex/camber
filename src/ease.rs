/*! Ready-to-use easing functions

A set of functions which take a parameter `t`, which is between `0` and `1` (or sometimes `-1` and
`1` and) transforms it to another value between `0` and `1` (or sometimes `-1` to `1`).

# Smooth start, step, and stop

As the name implies, the smooth _start_ funcitions have a lowest velocity when they start at `0`,
and the highest velocity when they end at `1`.

Conversely, smooth _stop_ functions have the highest velocity at `0` and the lowest velocity at `1`
where the range stops.

Finally, smooth step has the lowest velocity at the end points, and the highest in the center.

| degree | Smooth Start | Smooth Stop | Smooth Step |
|--------|--------------|-------------|-------------|
| 2      | [#][fn.start2] ![plot][start2] | [#][fn.stop2] ![plot][stop2] | [#][fn.step2] ![plot][step2] |
| 3      | [#][fn.start3] ![plot][start3] | [#][fn.stop3] ![plot][stop3] | [#][fn.step3] ![plot][step3] |
| 4      | [#][fn.start4] ![plot][start4] | [#][fn.stop4] ![plot][stop4] | [#][fn.step4] ![plot][step4] |
| 5      | [#][fn.start5] ![plot][start5] | [#][fn.stop5] ![plot][stop5] | [#][fn.step5] ![plot][step5] |
| 6      | [#][fn.start6] ![plot][start6] | [#][fn.stop6] ![plot][stop6] | [#][fn.step6] ![plot][step6] |
| 7      | [#][fn.start7] ![plot][start7] | [#][fn.stop7] ![plot][stop7] | [#][fn.step7] ![plot][step7] |
| 8      | [#][fn.start8] ![plot][start8] | [#][fn.stop8] ![plot][stop8] | [#][fn.step8] ![plot][step8] |
| 9      | [#][fn.start9] ![plot][start9] | [#][fn.stop9] ![plot][stop9] | [#][fn.step9] ![plot][step9] |

[fn.start2]: fn.smooth_start_2.html
[fn.start3]: fn.smooth_start_3.html
[fn.start4]: fn.smooth_start_4.html
[fn.start5]: fn.smooth_start_5.html
[fn.start6]: fn.smooth_start_6.html
[fn.start7]: fn.smooth_start_7.html
[fn.start8]: fn.smooth_start_8.html
[fn.start9]: fn.smooth_start_9.html
[fn.stop2]: fn.smooth_stop_2.html
[fn.stop3]: fn.smooth_stop_3.html
[fn.stop4]: fn.smooth_stop_4.html
[fn.stop5]: fn.smooth_stop_5.html
[fn.stop6]: fn.smooth_stop_6.html
[fn.stop7]: fn.smooth_stop_7.html
[fn.stop8]: fn.smooth_stop_8.html
[fn.stop9]: fn.smooth_stop_9.html
[fn.step2]: fn.smooth__2.html
[fn.step3]: fn.smooth__3.html
[fn.step4]: fn.smooth__4.html
[fn.step5]: fn.smooth__5.html
[fn.step6]: fn.smooth__6.html
[fn.step7]: fn.smooth__7.html
[fn.step8]: fn.smooth__8.html
[fn.step9]: fn.smooth__9.html


[start2]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_start_2.svg?sanitize=true
[start3]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_start_3.svg?sanitize=true
[start4]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_start_4.svg?sanitize=true
[start5]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_start_5.svg?sanitize=true
[start6]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_start_6.svg?sanitize=true
[start7]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_start_7.svg?sanitize=true
[start8]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_start_8.svg?sanitize=true
[start9]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_start_9.svg?sanitize=true
[stop2]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_stop_2.svg?sanitize=true
[stop3]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_stop_3.svg?sanitize=true
[stop4]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_stop_4.svg?sanitize=true
[stop5]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_stop_5.svg?sanitize=true
[stop6]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_stop_6.svg?sanitize=true
[stop7]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_stop_7.svg?sanitize=true
[stop8]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_stop_8.svg?sanitize=true
[stop9]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_stop_9.svg?sanitize=true
[step2]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_step_2.svg?sanitize=true
[step3]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_step_3.svg?sanitize=true
[step4]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_step_4.svg?sanitize=true
[step5]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_step_5.svg?sanitize=true
[step6]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_step_6.svg?sanitize=true
[step7]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_step_7.svg?sanitize=true
[step8]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_step_8.svg?sanitize=true
[step9]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_step_9.svg?sanitize=true
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
    /// `t^2`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_start_2.svg?sanitize=true
    smooth_start_2 t => t*t,

    /// `t^3`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_start_3.svg?sanitize=true
    smooth_start_3 t => t*t*t,

    /// `t^4`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_start_4.svg?sanitize=true
    smooth_start_4 t => t*t*t*t,

    /// `t^5`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_start_5.svg?sanitize=true
    smooth_start_5 t => t*t*t*t*t,

    /// `t^6`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_start_6.svg?sanitize=true
    smooth_start_6 t => t*t*t*t*t*t,

    /// `t^7`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_start_7.svg?sanitize=true
    smooth_start_7 t => t.powi(7),

    /// `t^8`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_start_8.svg?sanitize=true
    smooth_start_8 t => t.powi(8),

    /// `t^9`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_start_9.svg?sanitize=true
    smooth_start_9 t => t.powi(9)
}

/// Set smooth start to a certain degree, larger `i` is smoother start
#[inline]
pub fn smooth_start_i(i: i32, t: f64) -> f64 {
    t.powi(i)
}

macro_rules! flip_hv {
    ($f:ident, $t:ident) => {
        flip($f(flip($t)))
    }
}

// Smooth stop functions
def_ease! {
    /// `1 - (1 - t)^2`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_stop_2.svg?sanitize=true
    smooth_stop_2 t => flip_hv!(smooth_start_2, t),

    /// `1 - (1 - t)^3`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_stop_3.svg?sanitize=true
    smooth_stop_3 t => flip_hv!(smooth_start_3, t),

    /// `1 - (1 - t)^4`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_stop_4.svg?sanitize=true
    smooth_stop_4 t => flip_hv!(smooth_start_4, t),

    /// `1 - (1 - t)^5`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_stop_5.svg?sanitize=true
    smooth_stop_5 t => flip_hv!(smooth_start_5, t),

    /// `1 - (1 - t)^6`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_stop_6.svg?sanitize=true
    smooth_stop_6 t => flip_hv!(smooth_start_6, t),

    /// `1 - (1 - t)^7`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_stop_7.svg?sanitize=true
    smooth_stop_7 t => flip_hv!(smooth_start_7, t),

    /// `1 - (1 - t)^8`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_stop_8.svg?sanitize=true
    smooth_stop_8 t => flip_hv!(smooth_start_8, t),

    /// `1 - (1 - t)^9`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_stop_9.svg?sanitize=true
    smooth_stop_9 t => flip_hv!(smooth_start_9, t)
}

/// Stop with a smoothness of degree `i`, where larger `i` is a smoother stop
#[inline]
pub fn smooth_stop_i(i: i32, t: f64) -> f64 {
    flip(smooth_start_i(i, flip(t)))
}

// Define smooth step functions

macro_rules! def_smooth_step {
    ($( $(#[$attr:meta])* $name:ident [$a:ident $b:ident] ),+) => {
        def_ease! {
            $(
                $(#[$attr])*
                $name t => mix($a(t), $b(t), t)
            ),+
        }
    }
}

def_smooth_step! {
    /// Blend from smooth start and smooth stop with degree of smoothness `2`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_step_2.svg?sanitize=true
    smooth_step_2 [smooth_start_2 smooth_stop_2],

    /// Blend from smooth start and smooth stop with degree of smoothness `3`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_step_3.svg?sanitize=true
    smooth_step_3 [smooth_start_3 smooth_stop_3],

    /// Blend from smooth start and smooth stop with degree of smoothness `4`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_step_4.svg?sanitize=true
    smooth_step_4 [smooth_start_4 smooth_stop_4],

    /// Blend from smooth start and smooth stop with degree of smoothness `5`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_step_5.svg?sanitize=true
    smooth_step_5 [smooth_start_5 smooth_stop_5],

    /// Blend from smooth start and smooth stop with degree of smoothness `6`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_step_6.svg?sanitize=true
    smooth_step_6 [smooth_start_6 smooth_stop_6],

    /// Blend from smooth start and smooth stop with degree of smoothness `7`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_step_7.svg?sanitize=true
    smooth_step_7 [smooth_start_7 smooth_stop_7],

    /// Blend from smooth start and smooth stop with degree of smoothness `8`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_step_8.svg?sanitize=true
    smooth_step_8 [smooth_start_8 smooth_stop_8],

    /// Blend from smooth start and smooth stop with degree of smoothness `9`
    ///
    /// ![Example plot][plot]
    ///
    /// [plot]: https://raw.githubusercontent.com/glfmn/camber/develop/img/smooth_step_9.svg?sanitize=true
    smooth_step_9 [smooth_start_9 smooth_stop_9]
}

/// Step with a degree of smoothness `i`, where larger `i` is a smoother step.
#[inline]
pub fn smooth_step_i(i: i32, t: f64) -> f64 {
    mix(smooth_start_i(i, t), smooth_stop_i(i, t), t)
}
