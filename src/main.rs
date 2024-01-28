use crate::differentiate::derivative;
use crate::integrate::integral;
use crate::vector::{linspace, sum};

pub mod differentiate;
pub mod integrate;
pub mod vector;
pub mod matrix;
pub mod uncertainty;

fn main() {
    let x = linspace(0.0, 1.3, 20);
    for i in x.iter() {
        println!("{i}");
    }

    let v = vector::new(&[5.0, 5.4, 3.0]);
    for i in v.iter() {
        println!("{i}");
    }

    let m = matrix::ones(2, 2);
    for x in 0..2 {
        for y in 0..2 {
            let row = &m[y];
            let elem = row[x].to_string();
            println!("Matrix element: {y}_{x} is {elem}");
        }
    }
    

    let df = derivative(|x| x.powi(2), 2.0, 0.0001);
    println!("Derivative: {df}");

    let intf = integral(|x| x.powi(2), 0.0, 2.0, 0.0001);
    println!("Integral: {intf}");

    let sum = sum(&x);
    println!("Sum is: {sum}");
}
