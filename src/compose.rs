/*! Tools for composing and creating easing functions.
 */

/// "flip" parameter t
///
/// Depending on where the function composition happens, it will flip either horizontally or
/// vertically.
///
/// # Examples
///
/// `flip`ping the _parameter_ of an easing function will flip that function horizontally.
///
/// ```
/// use camber::Linspace;
/// # use camber::compose::flip;
/// use camber::ease::smooth_start_2;
/// let ts: Vec<f64> = Linspace::new(0., 1., 100)
///     .map(|t| smooth_start_2(flip(t)))
///     .collect();
/// ```
///
/// `flip`ping the _result_ of an easing function will flip that function vertically.
///
/// ```
/// use camber::Linspace;
/// # use camber::compose::flip;
/// use camber::ease::smooth_start_2;
/// let ts: Vec<f64> = Linspace::new(0., 1., 100)
///     .map(|t| flip(smooth_start_2(t)))
///     .collect();
/// ```
#[inline(always)]
pub fn flip(t: f64) -> f64 {
    1. - t
}
