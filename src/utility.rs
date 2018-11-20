//! Utility functions to use alongside and drive non-linear transforms


use std::iter::Iterator;

/// Evaluate a polynomial from its coefficients
///
/// A polynomial of degree _n_ has _n_+1 coefficients.  Providing a single coefficient is the same
/// as a constant function.
///
/// Achieves O(n) time complexity given n coefficients using _Horner's Rule_.
///
/// - `coefficients`: vector of coeffients in the order `a[n] .. a[0]`
/// - `x`: the desired input for the polynomial
///
/// # Examples
///
/// Starting with a simple polynomial _p(x) = x^2 + 6x + 3_, we make a vector of its coefficients.
///
/// ```
/// # use camber::poly_eval;
/// let poly = vec![1.,6.,3.];
/// #
/// # assert!(poly_eval(&poly, 0.) == 3.);
/// # assert!(poly_eval(&poly, 1.) == 10.);
/// ```
///
/// The easiest two points solve for by hand are usually _1_ and _0_; for our particular
/// polynomial, _p(0) = 3_ and _p(1) = 10_.  Running `poly_eval` with these in mind we have:
///
/// ```
/// # use camber::poly_eval;
/// # let poly = vec![1.,6.,3.];
/// #
/// assert!(poly_eval(&poly, 0.) == 3.);
/// assert!(poly_eval(&poly, 1.) == 10.);
/// ```
///
/// `poly_eval` is especially useful in cases where you have interpolated a polynomial with some
/// method and have obtained a list of coefficients and want to get those values over a range.
/// Using the same polynomial from before we can do things like evaluate values from _0_ to _10_
/// with 10 steps.
///
/// ```
/// # use camber::poly_eval;
/// use camber::Linspace;
/// # let poly = vec![1.,6.,3.];
/// #
/// Linspace::new(0., 10., 10).map(|x| poly_eval(&poly, f64::from(x)*0.1));
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

/// Iterator over the range [0, 1] with a set number of steps or stepsize
///
/// Compared to [`Linspace`], the stepper is faster but not as accurate, and tends to terminate
/// before reaching `1` exactly due to floating point precision.
///
/// # Examples
///
/// No matter what stepsize is chosen, the stepper will always run through at least one iteration
/// starting at exactly zero.
///
/// ```
/// # use camber::Stepper;
/// assert_eq!(Some(0.), Stepper::new(1e1000).next());
/// assert_eq!(Some(0.), Stepper::new(1e-16).next());
/// ```
///
/// The stepper will run until it's value is greater than 1, where it will simply truncate.
///
/// ```
/// # use camber::Stepper;
/// let mut step = Stepper::new(0.75);
/// assert_eq!(Some(0.0), step.next());
/// assert_eq!(Some(0.75), step.next());
/// assert_eq!(None, step.next());
/// ```
///
/// If the given stepsize is larger than 1, will yield 0 as the first element and then
/// terminate.
///
/// ```
/// # use camber::Stepper;
/// let mut zero = Stepper::new(1.5);
/// assert_eq!(Some(0.0), zero.next());
/// assert_eq!(None, zero.next());
/// ```
///
/// [`Linspace`]: struct.Linspace.html
#[derive(Debug, Copy, Clone)]
pub struct Stepper {
    t: f64,
    dt: f64,
}

impl Stepper {
    /// Create a stepper which steps from 0 to 1 with the given stepsize
    pub fn new(dt: f64) -> Self {
        Stepper {
            t: 0., dt,
        }
    }

    /// Create a stepper which steps from 0 to 1 inclusive with approximately `n` steps
    ///
    /// # Examples
    ///
    /// The stepper will tend to under-estimate the correct number of steps and stop right before
    /// getting to `1.0` due to limits of numerical percision.  Usually, the total is one less than
    /// the provided number of elements.
    ///
    /// ```
    /// # use camber::Stepper;
    /// # let n = 100;
    /// let total = Stepper::with_numel(n).count();
    /// assert!(total as f64 / n as f64 > 0.99);
    /// ```
    ///
    /// If the exact number of elements is important, use [`Linspace`] instead.
    ///
    /// When asked to provide only one step, the stepper will return 0 once.
    ///
    /// ```
    /// # use camber::Stepper;
    /// let mut zero = Stepper::with_numel(1);
    /// assert_eq!(Some(0.0), zero.next());
    /// assert_eq!(None, zero.next());
    /// ```
    ///
    /// A number of elements of zero will return `None` forever.
    ///
    /// ```
    /// # use camber::Stepper;
    /// let mut none = Stepper::with_numel(0);
    /// assert_eq!(None, none.next());
    /// assert_eq!(None, none.next());
    /// assert_eq!(None, none.next());
    /// assert_eq!(None, none.next());
    /// ```
    ///
    /// [`Linspace`]: struct.Linspace.html
    pub fn with_numel(n: usize) -> Self {
        let dt = if n > 1 {
            1. / (n-1) as f64
        } else {
            2.
        };

        Stepper {
            t: if n == 0 {2.} else {0.},
            dt,
        }
    }

    pub fn restart(&mut self) -> &Self {
        self.t = 0.;
        self
    }
}

impl Iterator for Stepper {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.t > 1. {
            None
        } else {
            let t = self.t;
            self.t += self.dt;
            Some(t)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let min = (1f64 - self.t) / self.dt;
        let max = 1f64 / self.dt;
        (min as usize + 1, Some(max as usize + 1))
    }
}

#[cfg(test)]
mod stepper {
    use float_cmp::ApproxEq;
    use proptest::prelude::*;
    use std::f64::EPSILON;
    use super::Stepper;

    fn arb_length() -> impl Strategy<Value = usize> {
        1..1_000_000usize
    }

    fn is_bounds(el: f64, min: f64, max: f64) -> bool {
        el.approx_eq(&min, 3.*EPSILON, 3) || el.approx_eq(&max, 3.*EPSILON, 3)
    }

    prop_compose! {
        fn arb_hint()
            (n in 1..100_000usize)
            (hint in (Just(n), 0..n))
        -> (usize, usize) {
            hint
        }
    }

    proptest! {

        #[test]
        fn respects_boundaries(n in arb_length()) {
            let stepper = Stepper::with_numel(n);
            for el in stepper {
                let in_bounds = (0. < el && el < 1.) || is_bounds(el, 0., 1.);
                assert!(in_bounds, "{:e} outside range [0, 1]", el);
            }
        }

        #[test]
        fn size_hint((n, c) in arb_hint()) {
            let mut s = Stepper::with_numel(n);
            print!("{:?} yields {:?} => ", s, s.size_hint());
            for _ in 0..c { s.next(); }
            if let (min, Some(max)) = s.size_hint() {
                let count = s.count();
                println!("{:?}, count {}", (min, max), count);
                assert!(min.max(count) - count.min(min) < 2);
                assert!(max.max(n) - n.min(max) < 2);
            } else {
                panic!();
            }
        }

        #[test]
        fn approx_right_numel(n in 1..100_000usize) {
            let total = Stepper::with_numel(n).count();
            let pass = if n > 100 {
                let proportion = total as f64 / n as f64;
                proportion > 0.99
            } else {
                total + 1 == n || total == n
            };
            assert!(pass, "total {} doesn't approximate set number of elements {}", total, n);
        }
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
/// # See Also
///
/// - [`Linspace`]
///
/// # Examples
///
/// To get 100 floating point numbers between zero and one, including zero and
/// one, we can do:
///
/// ```
/// # use camber::linspace;
/// linspace(0.,1.,100);
/// ```
///
/// We can do some more intersting things like evaluate a polynomial over some
/// range of x values
///
/// ```
/// # use camber::{linspace, poly_eval};
/// let xs = linspace(0.,1.,100);
/// // x^3
/// let poly = [1.,0.,0.];
/// let ys: Vec<f64> = xs.iter()
///     .map(|t| poly_eval(&poly,*t)) // f(x) = x^2
///     .collect();
/// ```
///
/// [`Linspace`]: struct.Linspace.html
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
    fn backwards() {
        let xs = linspace(2., -2., 2);
        assert_eq!(xs[0], 2.);
        assert_eq!(xs[1], -2.);
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
/// In addition to [`Iterator`], `Linspace` implements [`DoubleEndedIterator`].
///
/// # See Also
///
/// - [`Stepper`]
/// - [`linspace`]
///
/// # Examples
///
/// A range with zero elements simply `None` forever.
///
/// ```
/// use camber::Linspace;
///
/// let mut empty = Linspace::new(1., -1., 0);
/// assert_eq!(empty.next(), None);
/// assert_eq!(empty.next(), None);
/// ```
///
/// It's also possible to create a range of `t` values from which to evaluate a 1D polynomial.
///
/// ```
/// # use camber::Linspace;
/// use camber::poly_eval;
///
/// let mut ts = Linspace::new(-1., 1., 50);
/// let coeffients = [1., 5., 32., 1.];
/// let ts: Vec<f64> = ts.map(|t| poly_eval(&coeffients, t)).collect();
/// ```
///
/// [`linspace`]: fn.linspace.html
/// [`Stepper`]: struct.Stepper.html
/// [`DoubleEndedIterator`]: https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html
/// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
#[derive(Debug, Copy, Clone)]
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
    /// # use camber::Linspace;
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

    /// Create inclusive range iterator which starts at the end
    ///
    /// # Examples
    ///
    /// When initialized this way, no forward elements will be yielded
    ///
    /// ```
    /// # use camber::Linspace;
    /// let mut lin = Linspace::from_end(0., 1., 2);
    /// assert_eq!(lin.next(), None);
    /// ```
    ///
    /// When moving backwards with `next_back`, will behave as with `next`, only moving from the
    /// end element to the start element.
    ///
    /// ```
    /// # use camber::Linspace;
    /// let mut lin = Linspace::from_end(0., 1., 2);
    ///
    /// // 2 backwards elements
    /// assert_eq!(lin.next_back(), Some(1.));
    /// assert_eq!(lin.next_back(), Some(0.));
    ///
    /// assert_eq!(lin.next_back(), None);
    /// ```
    pub fn from_end(start: f64, end: f64, numel: usize) -> Self {
        let mut s = Linspace::new(start, end, numel);
        s.t = s.numel;
        s
    }

    /// Inclusive range iterator over the range 0 to 1 with the desired number of elements
    ///
    /// If numerical precision and the exact number of elements is less important than speed, try
    /// [`Stepper`] instead.
    ///
    /// [`Stepper`]: struct.Stepper.html
    pub fn normal(numel: usize) -> Self {
        Self::new(0., 1., numel)
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
    /// # use camber::Linspace;
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

impl DoubleEndedIterator for Linspace {
    fn next_back(&mut self) -> Option<Self::Item> {
        if 0 == self.t {
            None
        } else {
            self.t -= 1;
            let t = Self::t_n(self.t, self.numel);
            Some(lerp(self.start, self.end, t))
        }
    }
}

#[cfg(test)]
mod linspace_iterator {
    use float_cmp::ApproxEq;
    use proptest::prelude::*;
    use std::f64::EPSILON;
    use super::Linspace;

    fn is_bounds(el: f64, min: f64, max: f64) -> bool {
        el.approx_eq(&min, 3.*EPSILON, 3) || el.approx_eq(&max, 3.*EPSILON, 3)
    }

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

            for (i, el) in linspace.enumerate() {
                assert!(i < 2, "{:?}", linspace);
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
        fn correct_first_element((start, end) in arb_bounds(), n in arb_length()) {
            let mut linspace = Linspace::new(start, end, n);
            let first = linspace.next().expect("Last element must exist");
            assert!(first.approx_eq(&start, 3.*EPSILON, 3))
        }

        #[test]
        fn correct_end_element((start, end) in arb_bounds(), n in arb_length()) {
            let range = Linspace::new(start, end, n);
            let mut tf = 0.;
            for (i, t) in range.enumerate() {
                assert!(i <= n, "t_{} of {}: {:e}", i, n, t);
                tf = t;
            }
            assert!(
                tf.approx_eq(&end, 3.*EPSILON, 3),
                "tf: {:e} != {:e}", tf, end
            );
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
                    (min < el && el < max) || is_bounds(el, min, max),
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
