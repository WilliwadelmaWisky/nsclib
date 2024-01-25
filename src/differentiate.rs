/// Get a numerically calculated derivative of a function in a specific point
/// 
/// # Arguments
/// - `f`: Function to derivate
/// - `x`: Point to derivate
/// - `h`: A small value
/// 
/// # Examples
/// 
pub fn derivative(f: fn(f64) -> f64, x: f64, h: f64) -> f64 {
    return 0.5 * (f(x + h) - f(x - h)) / h;
}