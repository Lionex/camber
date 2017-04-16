//! Utility functions to use in conjunction with the curve interpolation tools.

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
/// # use camber::poly_eval;
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
/// # use camber::poly_eval;
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
/// # use camber::poly_eval;
/// # let poly = vec![1.,6.,3.];
/// #
/// (0..100).map(|x| poly_eval(&poly, f64::from(x)*0.1));
/// ```
///
pub fn poly_eval(coefficients: &Vec<f64>, x: f64) -> f64 {
    // From the form: p(x) = (((a_n*x + a_n-1)*x + ... + a_2)*x + a_1)*x + a_0
    coefficients.iter().fold(0., |b,c| (x*b) + c)
}

#[cfg(test)]
mod poly_eval {
    use super::*;

    #[test]
    // Test poly_eval's ability to compute p(x) = x^3 defined by coefficients
    fn simple_cubic() {
        let qubic = vec![1.,0.,0.,0.];
        for n in -10..10 {
            let m = poly_eval(&qubic, n.into());
            assert!((m - (n as f64).powi(3)).abs() < 1e-10,"{}^3 != {}",n,m);
        }
    }

    #[test]
    // Running with an empty vector represents the constant 0, so we expect 0
    fn empty_coefficient_vector() {
        let poly = vec![0.;0];
        assert_eq!(poly_eval(&poly, 1.),0.,"Empty vec evaluation is nonzero");
    }

    #[test]
    // A vector of all zeros should return zero
    fn zero_coefficient_vector() {
        let poly = vec![0.;11];
        assert_eq!(poly_eval(&poly, 1.),0.,"Zero vec evaluation is nonzero");
    }
}

/// Create an inclusive range of with the desired number of elements
///
/// Linspace can handle ranges in any direction, and even constant ranges like
/// `linspace(1.,1.,100);`  Linspace is also guaranteed to stay within the start
/// and end bounds.  It's useful for providing ranges over which to evaluate
/// polynomials.  A range with zero elements simply returns an empty vector.
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
/// let ys: Vec<f64> = xs.iter()
///     .map(|t| poly_eval(&vec![1.,0.,0.],*t)) // f(x) = x^2
///     .collect();
/// ```
/// or with a for loop:
/// ```
/// # use camber::{linspace, poly_eval};
/// let ys = for t in linspace(0.,1.,100) {
///     poly_eval(&vec![1.,0.,0.], t); // f(x) = x^2
/// };
/// ```
pub fn linspace(start: f64, end: f64, numel: u32) -> Vec<f64> {
    let n = numel as f64;
    // Given some desired start _s_ and end _e_, parameterize
    // _f(t) = s*(1-t) + e*(t)_ so _f(0) = s_ and _f(1) = e_,  then map over the
    // desired number of elements, and divide t by the number of elements to
    // retain the start and end bounds.
    (1..numel+1)
        .map(|t| (start*(1.-(t as f64)/n) + end*(t as f64)/n))
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
    fn constant_range() {
        for el in linspace(1.,1.,1000000) {
            assert_eq!(el,1.);
        }
    }

    #[test]
    fn respects_boundaries() {
        let (start,end) = (0.,1.);
        for el in linspace(start,end,1000000) {
            assert!((start as f64) <= el && el <= (end as f64));
        }
    }

    #[test]
    fn correct_length() {
        assert_eq!(linspace(0.,1.,1000000).len(), 1000000);
    }
}
