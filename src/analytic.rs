use crate::model::LinearRegression;

pub struct Analytic {
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

fn points2quadric(points: Vec<(f64, f64)>) -> (f64, f64, f64, f64, f64, f64, f64) {
    let mut a11: f64 = 0.0;
    let mut a12: f64 = 0.0;
    let mut a21: f64 = 0.0;
    let mut a22: f64 = 0.0;
    let mut b1: f64 = 0.0;
    let mut b2: f64 = 0.0;
    let mut c: f64 = 0.0;

    for (x, y) in points {
        a11 += x * x;
        a12 += x;
        a21 += x;
        a22 += 1.0;
        b1 += -2.0 * x * y;
        b2 += -2.0 * y;
        c += y * y;
    }
    // TODO implement calculation
    (a11, a12, a21, a22, b1, b2, c)
}

fn derive_dm(
    a11: f64,
    a12: f64,
    a21: f64,
    a22: f64,
    b1: f64,
    b2: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    return (2.0 * a11, a12, a21, 0.0, b1, 0.0);
}

fn derive_dn(
    a11: f64,
    a12: f64,
    a21: f64,
    a22: f64,
    b1: f64,
    b2: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    return (0.0, a12, a21, 2.0 * a22, 0.0, b2);
}

fn derive_dmdm(
    a11: f64,
    a12: f64,
    a21: f64,
    a22: f64,
    b1: f64,
    b2: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    return (a11, 0.0, 0.0, 0.0, 0.0, 0.0);
}

fn derive_dndn(
    a11: f64,
    a12: f64,
    a21: f64,
    a22: f64,
    b1: f64,
    b2: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    return (0.0, 0.0, 0.0, a22, 0.0, 0.0);
}

fn derive_dmdn(
    a11: f64,
    a12: f64,
    a21: f64,
    a22: f64,
    b1: f64,
    b2: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    return (0.0, a12, a21, 0.0, 0.0, 0.0);
}

fn determinat2x2matrix(a11: f64, a12: f64, a21: f64, a22: f64) -> f64 {
    a11 * a22 - a12 * a21
}

fn solve2x2matrix(a11: f64, a12: f64, a21: f64, a22: f64, b1: f64, b2: f64) -> (f64, f64) {
    let root1: f64 = (b1 - a12 * b2 / a21) / ((a21 / a22) + a11);
    let root2: f64 = (b2 - a21 * b1 / a12) / ((a12 / a11) + a22);

    (root1, root2)
}

pub fn calc_regression_fct(points: Vec<(f64, f64)>) -> (f64, f64) {
    let (a11, a12, a21, a22, b1, b2, c) = points2quadric(points);

    let (a11dm, a12dm, a21dm, a22dm, b1dm, b2dm) = derive_dm(a11, a12, a21, a22, b1, b2);
    let (a11dn, a12dn, a21dn, a22dn, b1dn, b2dn) = derive_dn(a11, a12, a21, a22, b1, b2);
    let (_, _, _, a22dndn, _, _) = derive_dndn(a11dn, a12dn, a21dn, a22dn, b1dn, b2dn);
    let (_, a12dmdn, a21dmdn, _, _, _) = derive_dmdn(a11dm, a12dm, a21dm, a22dm, b1dm, b2dm);
    let (a11dmdm, _, _, _, _, _) = derive_dmdm(a11dm, a12dm, a21dm, a22dm, b1dm, b2dm);
    // set gradient = 0
    let (root1, root2) = solve2x2matrix(
        a11dm,
        a12dm + a21dm,
        a22dn,
        a12dn + a21dn,
        -1.0 * b1dm,
        -1.0 * b2dn,
    );
    (root1, root2)
}
