# Camber

[![Build Status](https://travis-ci.org/Lionex/camber.svg?branch=master)](https://travis-ci.org/Lionex/camber)

> **Camber**: _v._ to curve

Camber is a curve interpolation library.

As it stands, this library is an exercise in implementing the following types of polynomial interpolations:

- Lagrange Divided Difference
- Cubic Curves
- Hermite Curves
- Cubic Bezier Curves
- Bezier Curve truncation
- Catmull-Rom Splines
- Cubic Splines
- B-Splines
- Uniform B Splines
- Non-Uniform B Splines
- Non-Uniform Rational B Splines

If I'm successful and figure out an interesting or particularly useful interface for this library, I might make it into a serious project.

## References

Lengyel, Eric; _Mathematics for 3D Game Programming and Computer Graphics_
Joy, Kenneth I; [_On-Line Geometric Modeling Notes: Bernstein Polynomials_](http://www.idav.ucdavis.edu/education/CAGDNotes/Bernstein-Polynomials.pdf)

## Contributing

Please set up the `pre-commit` hook using

```
$ ln -s .pre-commit.sh .git/hooks/pre-commit
```

to run tests on every commit before comitting.
