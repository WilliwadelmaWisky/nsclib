pub mod differentiate;
pub mod integrate;
pub mod vector;
pub mod matrix;
pub mod uncertainty;

fn main() {
    let a = vector::new(&[1.0, 2.0, 3.0]);
    let b = vector::new(&[2.0, -1.0, 1.0]);
    
    let dot = vector::dot_product(&a, &b);
    println!("Dot: {dot}");

    let cross = vector::cross_product(&a, &b);
    println!("Cross: [{}, {}, {}]", cross[0], cross[1], cross[2]);
}
