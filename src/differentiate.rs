use crate::vector;

/// Get a numerically calculated derivative of a function in a specific point
/// 
/// # Arguments
/// - `f`: Function to derivate
/// - `x`: Point to derivate
/// - `h`: A small value
/// 
/// # Examples
/// 
pub fn df(f: fn(f64) -> f64, x: f64, h: f64) -> f64 {
    return 0.5 * (f(x + h) - f(x - h)) / h;
}

/// Get a numerically calculated partial derivative of a function in a specific point
/// 
/// # Arguments
/// - `f`: Function to derivate
/// - `x`: Point to derivate
/// - `axis`: Partial derivative index
/// - `h`: A small value
pub fn partial_df(f: fn(&Vec<f64>) -> f64, x: &Vec<f64>, axis: usize, h: f64) -> f64 {
    let mut xplus = vector::clone(x);
    xplus[axis] += h;
    let mut xminus = vector::clone(x);
    xminus[axis] -= h;
    return 0.5 * (f(&xplus) - f(&xminus)) / h;
}