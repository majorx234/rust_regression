pub trait LinearRegression {
    fn new(points: Vec<(f64, f64)>, init_m: f64, init_n: f64) -> impl LinearRegression;
    fn step(self: Self) -> (f64, f64);
}
