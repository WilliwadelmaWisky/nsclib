pub mod differentiate;
pub mod integrate;
pub mod vector;
pub mod matrix;
pub mod transform;
pub mod uncertainty;
pub mod constant;

fn main() {
    let point = vector::new(&[1.0, 2.0, 3.0]);

    let res1 = differentiate::partial_df(|x| x[0].powi(2) * x[1] * x[2].powi(4), &point, 0, 0.001);
    println!("{res1}");

    let res2 = differentiate::partial_df(|x| x[0].powi(2) * x[1] * x[2].powi(4), &point, 1, 0.001);
    println!("{res2}");

    let res3 = differentiate::partial_df(|x| x[0].powi(2) * x[1] * x[2].powi(4), &point, 2, 0.001);
    println!("{res3}");
}
