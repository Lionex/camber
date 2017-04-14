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
