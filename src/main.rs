use crate::differentiate::derivative;
use crate::integrate::integral;
use crate::vector::{linspace, sum};

pub mod differentiate;
pub mod integrate;
pub mod vector;
pub mod uncertainty;

fn main() {
    let x = linspace(0.0, 1.3, 20);
    for i in x.iter() {
        println!("{i}");
    }

    let df = derivative(|x| x.powi(2), 2.0, 0.0001);
    println!("Derivative: {df}");

    let intf = integral(|x| x.powi(2), 0.0, 2.0, 0.0001);
    println!("Integral: {intf}");

    let sum = sum(&x);
    println!("Sum is: {sum}");
}
