use crate::partial_df;


/// Calculates an uncertainty using a standard uncertainty propagation method
/// 
/// # Arguments
/// - `f`: Function to calculate a variable
/// - `point`: Point where the uncertainty is calculated
/// - `err`: List of uncertainties of variables in f
/// 
/// # Examples
/// 
pub fn standard(f: fn(&Vec<f64>) -> f64, point: &Vec<f64>, err: &Vec<f64>) -> f64 {
    let mut uncertainty = 0.0;

    for i in 0..err.len() {
        let df = partial_df(f, point, i);
        uncertainty += (df * err[i]).powi(2);
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