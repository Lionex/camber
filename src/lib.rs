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

pub mod curve;
mod utility;

pub use utility::{poly_eval};
