use regression::{
    analytic::{self, Analytic},
    model::LinearRegression,
};

fn main() {
    let linear_regression = Analytic::new(vec![(0.0, 0.0), (1.0, 1.0)], 0.0, 0.0);
    let (m, n) = linear_regression.step();
    println!("m: {m}, n: {n}");
}
