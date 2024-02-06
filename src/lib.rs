pub mod optimize;
pub mod linalg;
pub mod constant;
pub mod uncertainty;

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

/// Get a numerically calculated second derivative of a function in a specific point
/// 
/// # Arguments
/// - `f`: Function to derivate
/// - `x`: Point to derivate
/// - `h`: A small value
/// 
/// # Examples
/// 
pub fn d2f(f: fn(f64) -> f64, x: f64, h: f64) -> f64 {
    return (f(x + h) - 2.0 * f(x) + f(x - h)) / h.powi(2);
}

/// Get a numerically calculated partial derivative of a function in a specific point
/// 
/// # Arguments
/// - `f`: Function to derivate
/// - `x`: Point to derivate
/// - `axis`: Partial derivative index
/// - `h`: A small value
/// 
/// # Examples
/// 
pub fn partial_df(f: fn(&Vec<f64>) -> f64, x: &Vec<f64>, axis: usize, h: f64) -> f64 {
    let mut xplus = copy(x);
    xplus[axis] += h;
    let mut xminus = copy(x);
    xminus[axis] -= h;
    return 0.5 * (f(&xplus) - f(&xminus)) / h;
}


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


/// Copy a list of values
pub fn copy(val: &Vec<f64>) -> Vec<f64> {
    let mut v = zeros(val.len());
    for i in 0..val.len() {
        v[i] = val[i];
    }

    return v;
}


/// Returns a list of zeros with specified size
pub fn zeros(len: usize) -> Vec<f64> {
    return vec![0.0; len];
}


/// Create a list of values
/// 
/// # Arguments
/// - `values`: Values of the list
/// 
/// # Examples
/// 
pub fn list(values: &[f64]) -> Vec<f64> {
    let mut v = zeros(values.len());
    for i in 0..values.len() {
        v[i] = values[i];
    }

    return v;
}


/// Vector
pub struct Vector {
    x: Vec<f64>
}

impl Vector {
    /// Create a vector with specified components
    /// 
    /// # Arguments
    /// - `x`: Components of a created vector
    /// 
    /// # Examples
    /// 
    pub fn new(x: &[f64]) -> Vector {
        let mut component_list = zeros(x.len());
        for i in 0..x.len() {
            component_list[i] = x[i];
        }

        return Vector { x: component_list };
    }

    /// Turn vector to a string, format `"[x1, x2, x3, ...]"`
    pub fn to_string(&self) -> String {
        let mut s = "[".to_owned();
        for i in 0..self.x.len() {
            s.push_str(&self.x[i].to_string());
            if i < self.x.len() - 1 {
                s.push_str(", ");
            }
        }

        s.push_str("]");
        return s;
    }
}