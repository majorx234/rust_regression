use crate::model::LinearRegression;

struct Analytic {
    m: f64,
    n: f64,
    points: Vec<(f64, f64)>,
    learn_rate: f64,
}

impl LinearRegression for Analytic {
    fn new(points: Vec<(f64, f64)>, init_m: f64, init_n: f64) -> Analytic {
        Self {
            m: 0.0,
            n: 0.0,
            points: Vec::new(),
            learn_rate: 0.0,
        }
    }
    fn step(self: Self) -> (f64, f64) {
        (0.0, 0.0)
    }
}
