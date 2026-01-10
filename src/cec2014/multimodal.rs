use crate::cec2014::basic_functions::{ackley, escaffer6, griewank, happy_cat, hgbat, katsuura, modified_schwefel, rastrigin, weierstrass};
use crate::cec2014::Cec14Helper;

/// Shifted and Rotated Rosenbrock’s Function
pub fn shifted_rotated_rosenbrock(values: &[f64], o: &[f64], m: &[f64], s_flag: bool, r_flag: bool) -> f64 {
    let mut x = values.to_vec();

    Cec14Helper::shift_sub(&mut x, o, s_flag);
    Cec14Helper::scale(&mut x, 2.048 / 100.0);
    let mut x = Cec14Helper::rotate(&x, m, r_flag);

    let mut result: f64 = 0.0;
    let mut p1: f64;
    let mut p2: f64;
    x[0] += 1.0; // shift to origin
    for i in 0..x.len().saturating_sub(1) {
        // shift to origin
        x[i+1] += 1.0;
        // rosenbrock part
        p1 = x[i] * x[i] - x[i+1];
        p2 = x[i] - 1.0;
        result += 100.0 * p1 * p1 + p2 * p2;
    }

    result
}

/// Shifted and Rotated Ackley’s Function
pub fn shifted_rotated_ackley(values: &[f64], o: &[f64], m: &[f64], s_flag: bool, r_flag: bool) -> f64 {
    let mut x = values.to_vec();

    Cec14Helper::shift_sub(&mut x, o, s_flag);
    let x = Cec14Helper::rotate(&x, m, r_flag);

    ackley(&x)
}

/// Shifted and Rotated Weierstrass Function
pub fn shifted_rotated_weierstrass(values: &[f64], o: &[f64], m: &[f64], s_flag: bool, r_flag: bool) -> f64 {
    let mut x = values.to_vec();

    Cec14Helper::shift_sub(&mut x, o, s_flag);
    Cec14Helper::scale(&mut x, 0.5 / 100.0);
    let x = Cec14Helper::rotate(&x, m, r_flag);

    weierstrass(&x)
}

/// Shifted and Rotated Griewank’s Function
pub fn shifted_rotated_griewank(values: &[f64], o: &[f64], m: &[f64], s_flag: bool, r_flag: bool) -> f64 {
    let mut x = values.to_vec();

    Cec14Helper::shift_sub(&mut x, o, s_flag);
    // 600.0 / 100.0 = 6.0
    Cec14Helper::scale(&mut x, 6.0);
    let x = Cec14Helper::rotate(&x, m, r_flag);

    griewank(&x)
}

/// Shifted Rastrigin’s Function
pub fn shifted_rastrigin(values: &[f64], o: &[f64], s_flag: bool) -> f64 {
    let mut x = values.to_vec();

    Cec14Helper::shift_sub(&mut x, o, s_flag);
    Cec14Helper::scale(&mut x, 5.12 / 100.0);

    rastrigin(&x)
}

/// Shifted and Rotated Rastrigin’s Function
pub fn shifted_rotated_rastrigin(values: &[f64], o: &[f64], m: &[f64], s_flag: bool, r_flag: bool) -> f64 {
    let mut x = values.to_vec();

    Cec14Helper::shift_sub(&mut x, o, s_flag);
    Cec14Helper::scale(&mut x, 5.12 / 100.0);
    let x = Cec14Helper::rotate(&x, m, r_flag);

    rastrigin(&x)
}

/// Shifted Schwefel’s Function
pub fn shifted_schwefel(values: &[f64], o: &[f64], s_flag: bool) -> f64 {
    let mut x = values.to_vec();

    Cec14Helper::shift_sub(&mut x, o, s_flag);
    // 1000.0 / 100.0 = 10.0
    Cec14Helper::scale(&mut x, 10.0);

    modified_schwefel(&x)
}

/// Shifted and Rotated Schwefel’s Function
pub fn shifted_rotated_schwefel(values: &[f64], o: &[f64], m: &[f64], s_flag: bool, r_flag: bool) -> f64 {
    let mut x = values.to_vec();

    Cec14Helper::shift_sub(&mut x, o, s_flag);
    // 1000.0 / 100.0 = 10.0
    Cec14Helper::scale(&mut x, 10.0);
    let x = Cec14Helper::rotate(&x, m, r_flag);

    modified_schwefel(&x)
}

/// Shifted and Rotated Katsuura Function
pub fn shifted_rotated_katsuura(values: &[f64], o: &[f64], m: &[f64], s_flag: bool, r_flag: bool) -> f64 {
    let mut x = values.to_vec();

    Cec14Helper::shift_sub(&mut x, o, s_flag);
    Cec14Helper::scale(&mut x, 5.0 / 100.0);
    let x = Cec14Helper::rotate(&x, m, r_flag);

    katsuura(&x)
}

/// Shifted and Rotated HappyCat Function
pub fn shifted_rotated_happy_cat(values: &[f64], o: &[f64], m: &[f64], s_flag: bool, r_flag: bool) -> f64 {
    let mut x = values.to_vec();

    Cec14Helper::shift_sub(&mut x, o, s_flag);
    Cec14Helper::scale(&mut x, 5.0 / 100.0);
    let x = Cec14Helper::rotate(&x, m, r_flag);

    happy_cat(&x)
}

/// Shifted and Rotated HGBat Function
pub fn shifted_rotated_hgbat(values: &[f64], o: &[f64], m: &[f64], s_flag: bool, r_flag: bool) -> f64 {
    let mut x = values.to_vec();

    Cec14Helper::shift_sub(&mut x, o, s_flag);
    Cec14Helper::scale(&mut x, 5.0 / 100.0);
    let x = Cec14Helper::rotate(&x, m, r_flag);

    hgbat(&x)
}

/// Shifted and Rotated Expanded Griewank’s plus Rosenbrock’s Function
pub fn shifted_rotated_expanded_griewank_rosenbrock(values: &[f64], o: &[f64], m: &[f64], s_flag: bool, r_flag: bool) -> f64 {
    let mut x = values.to_vec();

    Cec14Helper::shift_sub(&mut x, o, s_flag);
    Cec14Helper::scale(&mut x, 5.0 / 100.0);
    let mut x = Cec14Helper::rotate(&x, m, r_flag);

    let mut result = 0.0;
    let n = x.len();

    let mut p1: f64;
    let mut p2: f64;
    let mut p_rosen: f64;
    x[0] += 1.0; // shift to origin
    for i in 0..n.saturating_sub(1) {
        // shift to origin
        x[i+1] += 1.0;
        p1 = x[i] * x[i] - x[i+1];
        p2 = x[i] - 1.0;

        p_rosen = 100.0 * p1 * p1 + p2 * p2;
        result += (p_rosen * p_rosen) / 4000.0 - p_rosen.cos() + 1.0;
    }
    p1 = x[n-1] * x[n-1] - x[0];
    p2 = x[n-1] - 1.0;

    p_rosen = 100.0 * p1 * p1 + p2 * p2;
    result += (p_rosen * p_rosen) / 4000.0 - p_rosen.cos() + 1.0;

    result
}

/// Shifted and Rotated Expanded Scaffer’s F6 Function
pub fn shifted_rotated_escaffer6(values: &[f64], o: &[f64], m: &[f64], s_flag: bool, r_flag: bool) -> f64 {
    let mut x = values.to_vec();

    Cec14Helper::shift_sub(&mut x, o, s_flag);
    x = Cec14Helper::rotate(&x, m, r_flag);

    escaffer6(&x)
}