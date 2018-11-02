//! Utility functions to use in conjunction with the curve interpolation tools.

use std::iter::Iterator;

/// Evaluate a polynomial from its coefficients
///
/// A polynomial of degree _n_ has _n_+1 coefficients.  Providing a single
/// coefficient is the same as a constant function.
/// Achieves O(n) time complexity given n coefficients using _Horner's Rule_.
///
/// - `coefficients`: vector of coeffients in the order `a[n] .. a[0]`
/// - `x`: the desired input for the polynomial
///
/// # Examples
///
/// Starting with a simple polynomial _p(x) = x^2 + 6x + 3_, we make a vector
/// of its coefficients.
///
/// ```
/// # use camber::utility::poly_eval;
/// let poly = vec![1.,6.,3.];
/// #
/// # assert!(poly_eval(&poly, 0.) == 3.);
/// # assert!(poly_eval(&poly, 1.) == 10.);
/// ```
///
/// The easiest two points solve for by hand are usually _1_ and _0_; for our
/// particular polynomial, _p(0) = 3_ and _p(1) = 10_.  Running `poly_eval` with
/// these in mind we have:
///
/// ```
/// # use camber::utility::poly_eval;
/// # let poly = vec![1.,6.,3.];
/// #
/// assert!(poly_eval(&poly, 0.) == 3.);
/// assert!(poly_eval(&poly, 1.) == 10.);
/// ```
///
/// `poly_eval` is especially useful in cases where you have interpolated a
/// polynomial with some method and have obtained a list of coefficients and
/// want to get those values over a range.  Using the same polynomial from
/// before we can do things like evaluate values from _0_ to _10_ with steps of
/// _0.1_.
///
/// ```
/// # use camber::utility::poly_eval;
/// # let poly = vec![1.,6.,3.];
/// #
/// (0..100).map(|x| poly_eval(&poly, f64::from(x)*0.1));
/// ```
///
pub fn poly_eval(coefficients: &[f64], x: f64) -> f64 {
    // From the form: p(x) = (((a_n*x + a_n-1)*x + ... + a_2)*x + a_1)*x + a_0
    coefficients.iter().fold(0., |b,c| (x*b) + c)
}

#[cfg(test)]
mod poly_eval {
    use super::*;

    #[test]
    // Test poly_eval's ability to compute p(x) = x^3 defined by coefficients
    fn simple_cubic() {
        let qubic = [1.,0.,0.,0.];
        for t in linspace(-10.,10.,100) {
            let x = poly_eval(&qubic, t.into());
            assert!((x - (t).powi(3)).abs() < 1e-10,"{}^3 != {}",t,x);
        }
    }

    #[test]
    // Running with an empty vector represents the constant 0, so we expect 0
    fn empty_coefficient_vector() {
        let poly = [];
        assert_eq!(poly_eval(&poly, 1.),0.,"Empty vec evaluation is nonzero");
    }

    #[test]
    // A vector of all zeros should return zero
    fn zero_coefficient_vector() {
        let poly = [0.;11];
        assert_eq!(poly_eval(&poly, 1.),0.,"Zero vec evaluation is nonzero");
    }
}

#[inline(always)]
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a*(1.-t) + b*t
}

/// Create an inclusive range of with the desired number of elements
///
/// Linspace can handle ranges in any direction, and even constant ranges like
/// `linspace(1.,1.,100);`  Linspace is also guaranteed to stay within the start and end bounds.
/// It's useful for providing linear ranges over which to evaluate 1D transforms or polynomials.
///
/// A range with zero elements simply returns an empty vector.
///
/// _O(n)_ time complexity.
///
/// - `start`: The first value of the range
/// - `end`: The last value of the range
/// - `numel`: The number of elements in the range
///
/// # Examples
///
/// To get 100 floating point numbers between zero and one, including zero and
/// one, we can do:
///
/// ```
/// # use camber::utility::linspace;
/// linspace(0.,1.,100);
/// ```
///
/// We can do some more intersting things like evaluate a polynomial over some
/// range of x values
///
/// ```
/// # use camber::utility::{linspace, poly_eval};
/// let xs = linspace(0.,1.,100);
/// // x^3
/// let poly = [1.,0.,0.];
/// let ys: Vec<f64> = xs.iter()
///     .map(|t| poly_eval(&poly,*t)) // f(x) = x^2
///     .collect();
/// ```
pub fn linspace(start: f64, end: f64, numel: u32) -> Vec<f64> {
    if numel == 0 { return Vec::new(); }
    // Given some desired start _s_ and end _e_, parameterize
    // _f(t) = s*(1-t) + e*(t)_ so _f(0) = s_ and _f(1) = e_,  then map over the
    // desired number of elements, and divide t by the number of elements to
    // retain the start and end bounds.
    let n = (numel - 1) as f64;
    (0..numel)
        .map(|t| {
            let t = t as f64 / n;
            lerp(start, end, t)
        })
        .collect()
}

#[cfg(test)]
mod linspace {
    use super::linspace;

    #[test]
    fn zero_elements() {
        assert_eq!(linspace(0.,0.,0).len(), 0);
    }

    #[test]
    fn first_is_start_last_is_end() {
        let xs = linspace(-2., 2., 2);
        assert_eq!(xs[0], -2.);
        assert_eq!(xs[1], 2.);
    }

    #[test]
    fn constant_range() {
        for el in linspace(1.,1.,1000000) {
            assert_eq!(el,1.);
        }
    }

    #[test]
    fn respects_boundaries() {
        let (start,end) = (0.,1.);
        for el in linspace(start,end,1000000) {
            assert!(start as f64 <= el && el <= end as f64);
        }
    }

    #[test]
    fn correct_length() {
        assert_eq!(linspace(0.,1.,1000000).len(), 1000000);
    }
}

/// An inclusive range iterater with the desired number of elements
///
/// `Linspace` can handle ranges in any direction, and even constant ranges. Linspace is guaranteed
/// to stay within the start and end bounds.  It's useful for providing linear ranges over which to
/// evaluate 1D transforms or polynomials.
///
/// # Examples
///
/// A range with zero elements simply `None` forever.
///
/// ```
/// use camber::utility::Linspace;
///
/// let mut empty = Linspace::new(1., -1., 0);
/// assert_eq!(empty.next(), None);
/// assert_eq!(empty.next(), None);
/// ```
///
/// It's also possible to create a range of `t` values from which to evaluate a 1D polynomial.
///
/// ```
/// # use camber::utility::Linspace;
/// use camber::utility::poly_eval;
///
/// let mut ts = Linspace::new(-1., 1., 50);
/// let coeffients = [1., 5., 32., 1.];
/// let ys: Vec<f64> = ts.map(|t| poly_eval(&coeffients, t)).collect();
/// ```
///
/// This is better than using the `linspace` function as it does not allocate a vector which could
/// be wastefull
#[derive(Debug, Clone, Copy)]
pub struct Linspace {
    start: f64,
    end: f64,
    numel: usize,
    t: usize,
}

impl Linspace {
    /// Create inclusive range iterator over `numel` elements between `start` and `end`
    ///
    /// The _total_ number of elements generated is `numel` including the `start` and `end`. For
    /// example with 100 elements:
    ///
    /// ```
    /// # use camber::utility::Linspace;
    /// let mut lin = Linspace::new(0., 1., 100);
    /// assert_eq!(lin.count(), 100);
    /// ```
    pub fn new(start: f64, end: f64, mut numel: usize) -> Self {
        let mut t = 0;
        if numel == 1 {
            t = 1;
            numel = 2;
        }
        Linspace {
            start,
            end,
            numel,
            t,
        }
    }


    /// Create inclusive range iterater with a stepsize approximately equal to `step`
    pub fn with_stepsize(start: f64, end: f64, step: f64) -> Self {
        let numel = ((end-start) / step) as usize;
        Linspace {
            start,
            end,
            numel,
            t: 0,
        }
    }

    /// Set a new number of elements from the `current` element to the end
    ///
    /// This _will_ change the size of each step moving forward
    pub fn numel(&mut self, numel: usize) -> &Self {
        self.numel = self.t + numel;
        self
    }

    /// Start over again from the original `start` value
    ///
    /// ```
    /// # use camber::utility::Linspace;
    /// let mut lin = Linspace::new(0., 1., 2);
    ///
    /// // Consume all of the elements
    /// assert_eq!(lin.next(), Some(0.));
    /// assert_eq!(lin.next(), Some(1.));
    /// assert!(lin.next().is_none());
    ///
    /// lin.restart();
    /// assert_eq!(lin.next(), Some(0.));
    /// ```
    pub fn restart(&mut self) -> &Self {
        self.t = 0;
        self
    }

    #[inline(always)]
    fn t_n(t: usize, numel: usize) -> f64 {
        t as f64 / (numel - 1) as f64
    }
}

impl Iterator for Linspace {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.numel == self.t {
            None
        } else {
            let t = Self::t_n(self.t, self.numel);
            self.t += 1;
            Some(lerp(self.start, self.end, t))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.numel - self.t;
        (remaining, Some(self.numel))
    }

    fn last(self) -> Option<Self::Item> {
        let t = Self::t_n(self.numel - 1, self.numel);
        Some(lerp(self.start, self.end, t))
    }
}

#[cfg(test)]
mod linspace_iterator {
    use float_cmp::ApproxEq;
    use proptest::prelude::*;
    use std::f64::EPSILON;
    use super::Linspace;

    fn arb_bounds() -> impl Strategy<Value = (f64, f64)> {
        (any::<f64>(), any::<f64>())
    }

    fn arb_length() -> impl Strategy<Value = usize> {
        1..100_000usize
    }

    proptest! {
        #[test]
        fn zero_elements((start, end) in arb_bounds()) {
            let linspace = Linspace::new(start, end, 0);

            assert_eq!(linspace.count(), 0);
        }

        #[test]
        fn one_element((start, end) in arb_bounds()) {
            let linspace = Linspace::new(start, end, 1);

            for el in linspace {
                assert!(end.approx_eq(&el, 2.*EPSILON, 2))
            }
        }

        #[test]
        fn correct_last_element((start, end) in arb_bounds(), n in arb_length()) {
            let linspace = Linspace::new(start, end, n);
            let last = linspace.last().expect("Last element must exist");
            assert!(last.approx_eq(&end, 3.*EPSILON, 3))
        }

        #[test]
        fn correct_end_element((start, end) in arb_bounds(), n in arb_length()) {
            let range = Linspace::new(start, end, n);
            let mut tf = 0.;
            for t in range {
                tf = t;
            }
            assert_eq!(tf, end);
        }

        #[test]
        fn constant_range(constant in any::<f64>(), numel in arb_length()) {
            for el in Linspace::new(constant,constant,numel) {
                assert!(el.approx_eq(&constant, 3.*EPSILON, 3), "\tel {:e} == {:e}", el, constant);
            }
        }

        #[test]
        fn respects_boundaries((start, end) in arb_bounds(), n in arb_length()) {
            let min = start.min(end);
            let max = start.max(end);
            let linspace = Linspace::new(start, end, n);
            for el in linspace {
                assert! {
                    (min < el && el < max) ||
                        (el.approx_eq(&min, 3.*EPSILON, 3) || el.approx_eq(&max, 3.*EPSILON, 3)),
                    "el {:e} outside range [{:e}, {:e}]",
                    el,
                    min,
                    max
                };
            }
        }

        #[test]
        fn correct_length((start, end) in arb_bounds(), n in arb_length()) {
            let linspace = Linspace::new(start, end, n);
            assert_eq!(linspace.count(), n);
        }
    }
}
