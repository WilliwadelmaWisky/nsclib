# Numerical Scientific Computing Library (nsclib)

A library for numerical scientific computing. No dependencies required to use.

Work in Progress

## Features
- Derivative and Integral
- Linear algebra
- Uncertainty propagation
- Non-linear approximation

## Installation
NOT READY TO BE USED!!

## Examples
NOTE: Use statements may vary depending on where the files are located!

### Derivative
Calculate a derivative of a function `f(x) = 3 * x^2` at `x = 2`.

```rust
use nsclib::df;

let f: fn(f64) -> f64 = |x| 3.0 * x.powi(2);
let result = df(f, 2.0);
```

The `result` variable now contains the value of the derivative `12`.
Calculate a second derivative of a function `f(x) = 3 * x^2` at `x = 2`.

```rust
use nsclib::d2f;

let f: fn(f64) -> f64 = |x| 3.0 * x.powi(2);
let result = d2f(f, 2.0);
```

The `result` variable now contains the value of the second derivative `6`.

