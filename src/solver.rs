use crate::differentiate;


/// Find a root of a function with Newton-Raphson method
/// 
/// # Arguments
/// - `f`: Function to calculate the root of
/// - `x0`: Initial quess where the root might be
/// - `max_iteration`: Maximum iteration count, prevents infinite loops if cannot find the root
/// 
/// # Examples
/// 
pub fn newton(f: fn(f64) -> f64, x0: f64, max_iteration: usize) -> f64 {
    let tolerance = 0.000001;
    let mut x = x0;

    for _ in 0..max_iteration {
        let xi = x - f(x) / differentiate::df(f, x, tolerance);
        if (xi - x).abs() <= tolerance {
            break;
        }
        
        x = xi;
    }

    return x;
}

/// Find a root of a function with Brent's method
/// 
/// # Arguments
/// - `f`: Function to calculate the root of
/// - `a`: Start value of an interval
/// - `b`: End value of an interval
/// - `max_iteration`: Maximum iteration count, prevents infinite loops if cannot find the root
/// 
/// # Examples
/// 
pub fn brent(f: fn(f64) -> f64, a: f64, b: f64, max_iteration: usize) -> f64 {
    let tolerance = 0.000001;
    let mut x1 = a;
    let mut x2 = b;
    
    for _ in 0..max_iteration {
        if (x1 - x2).abs() <= tolerance {
            break;
        }

        let xi = x1 - (x2 - x1) / (f(x2) - f(x1)) * f(x1);
        if f(xi) * f(x1) < 0.0 {
            x2 = xi;
            continue;
        }

        x1 = xi;
    }

    return x1;
}