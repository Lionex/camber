//! > **Camber**: _v._ to curve (an object)
//!
//! `Camber` is a curve interpolation library that provides methods to generate
//! various types of interpolationg polynomials along with a set of related
//! utilities developed under MIT license as an educational project.
//!
//! As it stands, this library is an exercise to implement the following curve
//! interpolation methods:
//!
//! - Lagrange Divided Difference
//! - Cubic Curves
//! - Hermite Curves
//! - Cubic Bezier Curves
//! - Bezier Curve truncation
//! - Catmull-Rom Splines
//! - Cubic Splines
//! - B-Splines
//! - Uniform B Splines
//! - Non-Uniform B Splines
//! - Non-Uniform Rational B Splines


/// Evaluate a polynomial defined by its coefficients in the order _a\_n..a\_0_
///
/// Has O(n) time complexity given n coefficients using _Horner's Rule_.
pub fn poly_eval(coefficients: &Vec<f64>, x: f64) -> f64 {
    // From the form: p(x) = (((a_n*x + a_n-1)*x + ... + a_2)*x + a_1)*x + a_0
    coefficients.iter().fold(0., |b,c| (x*b) + c)
}

#[cfg(test)]
mod poly_eval_test {
    use super::*;
    use test::Bencher;

    #[test]
    // Test poly_eval's ability to compute p(x) = x^3 defined by coefficients
    fn simple_cubic() {
        let qubic = vec![1.,0.,0.,0.];
        for n in -10..10 {
            let m = poly_eval(&qubic, n.into());
            assert!((m - f64::from(n).powi(3)).abs() < 1e-10,"{}^3 != {}",n,m);
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

    #[bench]
    fn degree_2(b: &mut Bencher) {
        let poly = vec![1.; 3];
        b.iter(|| poly_eval(&poly, 1.));
    }

    #[bench]
    fn degree_8(b: &mut Bencher) {
        let poly = vec![1.; 9];
        b.iter(|| poly_eval(&poly, 1.));
    }

    #[bench]
    fn degree_16(b: &mut Bencher) {
        let poly = vec![1.; 17];
        b.iter(|| poly_eval(&poly, 1.))
    }

    #[bench]
    fn degree_32(b: &mut Bencher) {
        let poly = vec![1.; 33];
        b.iter(|| poly_eval(&poly, 1.))
    }
}
