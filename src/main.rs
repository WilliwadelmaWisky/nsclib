pub mod differentiate;
pub mod integrate;
pub mod vector;
pub mod matrix;
pub mod transform;
pub mod uncertainty;

fn main() {
    let a = vector::new(&[1.0, 2.0, 3.0]);
    let b = vector::newi(&[2, -1, 1]);
    
    let dot = vector::dot_product(&a, &b);
    println!("Dot: {dot}");

    let cross = vector::cross_product(&a, &b);
    println!("Cross: [{}, {}, {}]", cross[0], cross[1], cross[2]);

    let m = matrix::ones(2, 3);
    let v = transform::transform(&m, &a);
    println!("Transform: [{}, {}]", v[0], v[1]);
}
