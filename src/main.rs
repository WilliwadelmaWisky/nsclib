pub mod calc;
pub mod linalg;

fn main() {
    let point = calc::list(&[1.0, 2.0, 3.0]);
    let err = calc::list(&[0.01, 0.2, 0.08]);
    let val = point[0].powi(2) * point[1].powi(3) * point[2].powi(5);
    let unc = calc::uncertainty::calc_standard(|x| x[0].powi(2) * x[1].powi(3) * x[2].powi(5), &point, &err);
    println!("Uncertainty: {val}+-{unc}");

    let root1 = calc::newton(|x| x.powi(2) - 3.0, 1.0, 100);
    let root2 = calc::brent(|x| x.powi(2) - 3.0, 1.0, 2.0, 100);
    println!("Roots: {root1}, {root2}");
}
