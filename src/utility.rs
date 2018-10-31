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
        for n in -10..10 {
            let m = poly_eval(&qubic, n.into());
            assert!((m - f64::from(n).powi(3)).abs() < 1e-10,"{}^3 != {}",n,m);
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
