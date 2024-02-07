/// Operate a vector with a matrix
/// 
/// # Arguments
/// - `m`: Transformation matrix
/// - `v`: Vector to transform
/// 
/// # Examples
pub fn transform(m: &Vec<Vec<f64>>, v: &Vec<f64>) -> Vec<f64> {
    let rows = m.len();
    let mut result = zeros(rows);

    for i in 0..rows {
        let row = &m[i];
        result[i] = dot_product(row, v);
    }

    return result;
}



/// Create a vector with specified components
/// 
/// # Arguments
/// - `values`: Components of a created vector
/// 
/// # Examples
pub fn new(values: &[f64]) -> Vec<f64> {
    let mut v = zeros(values.len());
    for i in 0..values.len() {
        v[i] = values[i];
    }

    return v;
}

/// Create a vector with specified components (all whole numbers)
/// 
/// # Arguments
/// - `values`: Components of a created vector¨
/// 
/// # Examples
pub fn newi(values: &[i64]) -> Vec<f64> {
    let mut v = zeros(values.len());
    for i in 0..values.len() {
        v[i] = values[i] as f64;
    }

    return v;
}

/// Clone a vector
pub fn clone(vector: &Vec<f64>) -> Vec<f64> {
    let mut v = zeros(vector.len());
    for i in 0..vector.len() {
        v[i] = vector[i];
    }

    return v;
}

/// Returns a list of zeros with specified size
pub fn zeros(len: usize) -> Vec<f64> {
    return vec![0.0; len];
}

/// Returns a list of ones with specified size
pub fn ones(len: usize) -> Vec<f64> {
    return vec![1.0; len];
}

/// Returns a list of values evenly separated
/// 
/// # Arguments
/// - `start`: A start point of the list
/// * `end`: An end point of the list
/// * `num`: A size of a list
/// 
/// # Examples
/// 
pub fn linspace(start: f64, end: f64, num: usize) -> Vec<f64> {
    let mut v: Vec<f64> = zeros(num);
    let h: f64 = (end - start) / (num - 1) as f64;
    for i in 0..num {
        v[i] = start + i as f64 * h;
    }

    return v;
}

/// Calculates the sum of all the components of a given vector
/// 
/// # Arguments
/// - `v`: Vector
/// 
/// # Examples
/// 
pub fn sum(v: &Vec<f64>) -> f64 {
    let mut result: f64 = 0.0;
    for i in v.iter() {
        result += i;
    }

    return result;
}

/// Calculates a square magnitude (length^2) of a given vector
/// 
/// # Arguments
/// - `v`: Vector
/// 
/// # Examples
/// 
pub fn sqr_magnitude(v: &Vec<f64>) -> f64 {
    let mut sqr_sum = 0.0;
    for vi in v.iter() {
        sqr_sum += vi * vi;
    }

    return sqr_sum;
}

/// Calculates a magnitude (length) of a given vector
/// 
/// # Arguments
/// - `v`: Vector
/// 
/// # Examples
/// 
pub fn magnitude(v: &Vec<f64>) -> f64 {
    let sqr_magnitude = sqr_magnitude(v);
    return sqr_magnitude.sqrt();
}

/// Calculates a normalized vector from a given vector, does not change the original
/// 
/// # Arguments
/// - `v`: Vector
/// 
/// # Examples
pub fn normalize(v: &Vec<f64>) -> Vec<f64> {
    let len = v.len();
    let mut norm_v = zeros(len);
    let inverse_magnitude = 1.0 / magnitude(v);

    for i in 0..len {
        norm_v[i] = inverse_magnitude * v[i];
    }

    return norm_v;
}

/// Dot product of two vectors
/// 
/// # Arguments
/// - `v1`: First vector
/// - `v2`: Second vector
/// 
/// # Examples
/// 
pub fn dot_product(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    let mut dot = 0.0;

    for i in 0..v1.len() {
        dot += v1[i] * v2[i];
    }

    return dot;
}

/// Cross product of two 3d vectors (v1 x v2)
/// 
/// # Arguments
/// - `v1`: First vector
/// - `v2`: Second vector
/// 
/// # Examples
pub fn cross_product(v1: &Vec<f64>, v2: &Vec<f64>) -> Vec<f64> {
    return vec![
        v1[1] * v2[2] - v1[2] * v2[1],
        v1[2] * v2[0] - v1[0] * v2[2],
        v1[0] * v2[1] - v1[1] * v2[0]
    ];
}