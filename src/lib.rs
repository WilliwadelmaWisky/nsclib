pub mod optimize;
pub mod linalg;
pub mod constant;
pub mod uncertainty;

/// Get a numerically calculated derivative of a function in a specific point
/// 
/// # Arguments
/// - `f`: Function to derivate
/// - `x`: Point to derivate
/// 
/// # Examples
/// 
pub fn df(f: fn(f64) -> f64, x: f64) -> f64 {
    let h = 0.000_001;
    return 0.5 * (f(x + h) - f(x - h)) / h;
}

/// Get a numerically calculated second derivative of a function in a specific point
/// 
/// # Arguments
/// - `f`: Function to derivate
/// - `x`: Point to derivate
/// 
/// # Examples
/// 
pub fn d2f(f: fn(f64) -> f64, x: f64) -> f64 {
    let h = 0.000_001;
    return (f(x + h) - 2.0 * f(x) + f(x - h)) / h.powi(2);
}

/// Get a numerically calculated partial derivative of a function in a specific point
/// 
/// # Arguments
/// - `f`: Function to derivate
/// - `x`: Point to derivate
/// - `axis`: Partial derivative index
/// 
/// # Examples
/// 
pub fn partial_df(f: fn(&Vector) -> f64, x: &Vector, axis: usize) -> f64 {
    let h = 0.000_001;
    let mut xplus = x.clone();
    xplus.set(axis, xplus.get(axis) + h);
    let mut xminus = x.clone();
    xminus.set(axis, xminus.get(axis) - h);
    return 0.5 * (f(&xplus) - f(&xminus)) / h;
}

/// Get a numerically calculated gradient of a function in a specific point
/// 
/// # Arguments
/// - `f`: Function to derivate
/// - `x`: Point to derivate
/// 
/// # Examples
/// 
pub fn gradient(f: fn(&Vector) -> f64, x: &Vector) -> Vector {
    let arg_count = x.length();
    let mut grad = Vector::zeros(arg_count);

    for i in 0..arg_count {
        grad.set(i, partial_df(f, x, i));
    }

    return grad;
}

/// Get a divergence of a function in a specific point
/// 
/// # Arguments
/// - `f`: Function
/// - `x`: Point
pub fn divergence(f: fn(&Vector) -> f64, x: &Vector) -> f64 {
    let grad = gradient(f, x);
    return grad.sum();
}


/// Calculate an integral of a function
/// 
/// # Arguments
/// - `f`: Function
/// - `x0`: Start point
/// - `x1`: End point
/// 
/// # Examples
/// 
pub fn integral(f: fn(f64) -> f64, x0: f64, x1: f64) -> f64 {
    let h = 0.000_001;
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
pub fn array(values: &[f64]) -> Vec<f64> {
    let mut v = zeros(values.len());
    for i in 0..values.len() {
        v[i] = values[i];
    }

    return v;
}


/// Vector
#[derive(Clone)]
pub struct Vector {
    components: Vec<f64>
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

        return Vector { components: component_list };
    }

    /// Create a 2d-vector
    /// 
    /// # Arguments
    /// - `x`: x-component of the vector
    /// - `y`: y-component of the vector
    /// 
    /// # Examples
    /// 
    pub fn new_2d(x: f64, y: f64) -> Vector {
        return Vector::new(&[x, y]);
    }

    /// Create a 3d-vector
    /// 
    /// # Arguments
    /// - `x`: x-component of the vector
    /// - `y`: y-component of the vector
    /// - `z`: z-component of the vector
    /// 
    /// # Examples
    /// 
    pub fn new_3d(x: f64, y: f64, z: f64) -> Vector {
        return Vector::new(&[x, y, z]);
    }

    /// Create a vector of zeros with specified length (0, 0, ..., 0)
    /// 
    /// # Arguments
    /// - `length`: Length of a vector
    pub fn zeros(length: usize) -> Vector {
        let component_list = vec![0.0; length];
        return Vector { components: component_list };
    }

    /// Create a vector of ones with specified length (1, 1, ..., 1)
    /// 
    /// # Arguments
    /// - `length`: Length of a vector
    pub fn ones(length: usize) -> Vector {
        let component_list = vec![1.0; length];
        return Vector { components: component_list };
    }

    /// Get a number of components in a vector
    pub fn length(&self) -> usize {
        return self.components.len();
    }

    pub fn get(&self, index: usize) -> f64 {
        return self.components[index];
    }

    pub fn set(&mut self, index: usize, val: f64) {
        self.components[index] = val;
    }

    /// Turn vector to a string, format `"[x1, x2, x3, ...]"`
    pub fn to_string(&self) -> String {
        let mut s = "[".to_owned();
        for i in 0..self.components.len() {
            s.push_str(&self.components[i].to_string());
            if i < self.components.len() - 1 {
                s.push_str(", ");
            }
        }

        s.push(']');
        return s;
    }

    /// Calculate a sum of all components in the vector
    pub fn sum(&self) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.length() {
            sum += self.get(i);
        }

        return sum;
    }
}

#[derive(Clone)]
pub struct Matrix {
    rows: Vec<Vector>
}

impl Matrix {
    /// Create a matrix of zeros with specified size {{0, 0, ..., 0}, ..., {0, 0, ..., 0}}
    /// 
    /// # Arguments
    /// - `rows`: Number of rows
    /// - `cols`: Number of columns
    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        let row = Vector::zeros(cols);
        let row_list = vec![row; rows];
        return Matrix { rows: row_list };
    }

    pub fn to_string(&self) -> String {
        let row_count = self.rows.len();
        let mut s = "[".to_owned();

        for i in 0..row_count {
            s.push_str(&self.rows[i].to_string());
            if i < row_count - 1 {
                s.push_str(",\n ");
            }
        }

        s.push(']');
        return s;
    }
}
