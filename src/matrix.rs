use crate::vector;

/// Create a matrix of zeros
pub fn zeros(rows: usize, cols: usize) -> Vec<Vec<f64>> {
    let row = vector::zeros(cols);
    return vec![row; rows];
}

/// Create a matrix of ones
pub fn ones(rows: usize, cols: usize) -> Vec<Vec<f64>> {
    let row = vector::ones(cols);
    return vec![row; rows];
}
