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

/// Absolute error assert for floating-point valued functions
#[cfg(test)]
macro_rules! abs_err_test {
    ($test:tt == $base:tt ~ $range:tt, $($tail:tt)*) => {
        assert!(($test - $base).abs() < $range, $($tail)*);
    };
    ($test:tt == [$lower:expr,$upper:expr], $($tail:tt)*) => {
        assert!($test <= $upper && $test >= $lower, $($tail)*);
    };
    ($test:tt == ($lower:expr,$upper:expr), $($tail:tt)*) => {
        assert!($test < $upper && $test > $lower, $($tail)*);
    };
}

/// Relative error assert for floating-point valued functions
#[cfg(test)]
macro_rules! rel_err_test {
    ($test:tt == $base:tt ~ $range:tt, $($tail:tt)*) => {
        assert!(($test - $base).abs()/$base < $range, $($tail)*);
    };
    ($test:tt == [$lower:expr,$upper:expr], $($tail:tt)*) => {
        assert!($test/$upper <= $upper && $test/$lower >= $lower, $($tail)*);
    };
    ($test:tt == ($lower:expr,$upper:expr), $($tail:tt)*) => {
        assert!($test/$upper < $upper && $test/$lower > $lower, $($tail)*);
    };
}

pub mod curve;
mod utility;

pub use utility::{poly_eval, linspace};
