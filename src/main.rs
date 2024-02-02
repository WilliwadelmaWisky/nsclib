pub mod differentiate;
pub mod integrate;
pub mod vector;
pub mod matrix;
pub mod transform;
pub mod uncertainty;
pub mod constant;

fn main() {
    let point = vector::new(&[1.0, 2.0, 3.0]);
    let err = vector::new(&[0.01, 0.2, 0.08]);
    let val = point[0].powi(2) * point[1].powi(3) * point[2].powi(5);
    let unc = uncertainty::calc_standard(|x| x[0].powi(2) * x[1].powi(3) * x[2].powi(5), &point, &err);
    println!("Uncertainty: {val}+-{unc}");
}
