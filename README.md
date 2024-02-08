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
Calculate a first and second derivative of a function `f(x) = 3 * x^2` at `x = 2`.

```rust
use nsclib::{df, df2};

let f: fn(f64) -> f64 = |x| 3.0 * x.powi(2);
let first = df(f, 2.0);
let second = d2f(f, 2.0);
```

The `first` variable now contains the value of the fisrt derivative `12` and  
the `second` variable contains the value of the second derivative `6`.
