//! n-degree Polynomial interpolation
//!
//! # Introduction
//!
//! Polynomial interpolation takes pairs of values _(t,f(t))_ and fits an
//! interpolating polynomial to those points.  In other words, it constructs a
//! polynomial that goes through a series of points _exactly_, with some
//! limitations.
//!
//! In some applications this is done to reconstruct some sort of
//! experimental data to get points between the originally sampled points.
//! Curve interpolation also often appears in computer graphics applications:
//! vector art software, as an example, essentially just interpolates curves
//! between points set by an artist.
//!
//! The methods here in some way or another satisfy the following conditions on
//! their interpolating polynomials _p(x)_: given a point _t_ and some value
//! _f(t)_, _p(t) == f(t)_.  Don't forget that _p(x)_ does not necessarily equal
//! _f(x)_ in between the interpolating points.  For further reading see
//! [Runge's Phenomenon][1] and UC Davis's comprehensive
//! [_On-Line Geometric Modeling Notes_][2].
//!
//! [1]: https://en.wikipedia.org/wiki/Runge%27s_phenomenon
//! [2]: http://idav.ucdavis.edu/education/CAGDNotes/CAGDNotes/homepage.html

/// Create a bernstein polynomial _B(t)_ defined for _0 <= t <= 1_
///
/// For any degreen n, n+1 bernstein polynomials exist.  Summing all of them
/// together for any _t_ where _0 <= t <= 1_ results in a value of one.  These
/// polynomials often serve as belnding functions for a curve that interpolates
/// _n+1_ points with a polynomial of degree _n_.
///
/// - `n`: the degree of the berinstein polynomial
/// - `k`: identifies a particular bernstein polynomial where _0 <= k <= n_
/// - `t`: the specific point at which to evaluate the polynomial.
///
/// Notable properties of the Bernstein Polynomials include:
/// - All Bernstein polynomials are Non-Negative from _0 <= t <= 1_
/// - Any of the lower-degree Bernstein polynomials (degree < n) can be
///   expressed as a linear combination of Bernstein polynomials of degree n
/// - Derivatives of the _n_th degree Bernstein polynomials are polynomials of
///   degree _n−1_.
///
/// For a more comprehensive discussion, see [Kenneth Joy's notes.][1]
///
/// # Examples
///
/// [1]: http://idav.ucdavis.edu/education/CAGDNotes/Bernstein-Polynomials.pdf
pub fn bernstein(n: u32, k: u32,t: f64) -> f64 {
    assert!(k <= n);
    (choose(n,k) as f64) * t.powi(k as i32) * (1.-t).powi((n - k) as i32)
}

// Calculate binomial coefficients recursively in O(n) time with support for
// large n
fn choose(n: u32, k: u32) -> u32 {
    assert!(n >= k);
    if k == 0 {
        1
    } else if k > n / 2 {
        choose(n,n-k)
    } else {
        n * choose(n-1,k-1) / k
    }
}

#[cfg(test)]
mod bernstein {
    use super::bernstein;
    use utility::*;

    #[test]
    fn non_negative() {
        for t in linspace(0.,1.,100) {
            assert!(bernstein(1,1,t) >= 0.);
        }
    }

    #[test]
    // For any value t, all of the berinstein polynomials of degree n should sum
    // to 1, forming a partition of unity.
    fn partition_of_unity() {
        for n in 0..13 {
            for t in linspace(0.,1.,100) {
                // Sum all of the berinstein polynomials of degree n together
                let unit = (0..n+1).map(|k|bernstein(n,k,t)).fold(0.,|s,v|s+v);
                assert!((unit - 1.).abs() < 1e-3, "Expected 1 got {} with n of {} for t of {}", unit, n, t);
            }
        }
    }

    #[test]
    fn large_n() {
        let n = u32::max_value();
        assert!(bernstein(n,n,1.) == 1.);
    }
}
