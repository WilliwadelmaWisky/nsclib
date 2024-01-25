use crate::vector::linspace;

/// Calculate an integral of a function
/// 
/// # Arguments
/// - `f`: Function
/// - `x0`: Start point
/// - `x1`: End point
/// - `h`: Small value
/// 
/// # Examples
/// 
pub fn integral(f: fn(f64) -> f64, x0: f64, x1: f64, h: f64) -> f64 {
    let num = 1 + ((x1 - x0) / h).floor() as usize;
    let x_values = linspace(x0, x1, num);
    let mut sum = 0.0;
    for x in x_values {
        sum += f(x);
    }
    return h * sum;
}