use crate::{partial_df, Vector};


/// Calculates an uncertainty using a standard uncertainty propagation method
/// 
/// # Arguments
/// - `f`: Function to calculate a variable
/// - `point`: Point where the uncertainty is calculated
/// - `err`: List of uncertainties of variables in f
/// 
/// # Examples
/// 
pub fn standard(f: fn(&Vector) -> f64, point: &Vector, err: &Vector) -> f64 {
    let arg_count = point.length();
    let mut uncertainty = 0.0;

    for i in 0..arg_count {
        let df = partial_df(f, point, i);
        uncertainty += (df * err.get(i)).powi(2);
    }

    return uncertainty.sqrt();
}

/// Calculates an uncertainty using a minmax uncertainty propagation method
/// 
/// # Arguments
/// - `f`: Function to calculate a variable
/// - `point`: Point where the uncertainty is calculated
/// - `err`: List of uncertainties of variables in f
/// 
/// # Examples
pub fn minmax(_f: fn(&Vec<f64>) -> f64, _point: &Vec<f64>, _err: &Vec<f64>) -> f64 {
    return 0.0;
}