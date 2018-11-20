//! > **Camber**: _v._ to curve (an object)
//!
//! `Camber` is a library of fast, composeable, non-linear, 1D transformation functions also known
//! as _easing functions_ or _normalized utility functions_.

#[cfg_attr(test, macro_use)]
#[cfg(test)]
extern crate proptest;
#[cfg(test)]
extern crate float_cmp;

mod utility;
pub use utility::*;
