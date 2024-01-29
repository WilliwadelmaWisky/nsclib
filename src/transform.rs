use crate::vector;

/// Operate a vector with a matrix
/// 
/// # Arguments
/// - `m`: Transformation matrix
/// - `v`: Vector to transform
/// 
/// # Examples
pub fn transform(m: &Vec<Vec<f64>>, v: &Vec<f64>) -> Vec<f64> {
    let rows = m.len();
    let mut result = vector::zeros(rows);

    for i in 0..rows {
        let row = &m[i];
        result[i] = vector::dot_product(row, v);
    }

    return result;
}