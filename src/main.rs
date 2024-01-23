fn main() {
    let x = linspace(0.0, 1.3, 20);
    for i in x.iter() {
        println!("{i}");
    }

    let sum = sum(x);
    println!("Sum is: {sum}");
}

/// Returns a list of zeros with specified size
fn zeros(len: usize) -> Vec<f64> {
    return vec![0.0; len];
}

/// Returns a list of ones with specified size
fn ones(len: usize) -> Vec<f64> {
    return vec![0.0; len];
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
fn linspace(start: f64, end: f64, num: usize) -> Vec<f64> {
    let mut vector: Vec<f64> = zeros(num);
    let h: f64 = (end - start) / (num - 1) as f64;
    for i in 0..num {
        vector[i] = start + i as f64 * h;
    }

    return vector;
}


/// Get a numerically calculated derivative of a function in a specific point
/// 
/// # Arguments
/// - `f`: Function to derivate
/// - `x`: Point to derivate
/// - `h`: A small value
/// 
/// # Examples
/// 
fn derivative(f: fn(f64) -> f64, x: f64, h: f64) -> f64 {
    return 0.5 * (f(x + h) - f(x - h)) / h;
}

fn integral(f: fn(f64) -> f64, x0: f64, x1: f64) -> f64 {
    return 0.0;
}

fn sum(v: Vec<f64>) -> f64 {
    let mut result: f64 = 0.0;
    for i in v.iter() {
        result += i;
    }

    return result;
}

fn magnitude(v: Vec<f64>) -> f64 {
    return 0.0;
}
