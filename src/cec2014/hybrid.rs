use crate::cec2014::basic_functions::discus;
use crate::cec2014::multimodal::{shifted_rotated_ackley, shifted_rotated_escaffer6, shifted_rotated_expanded_griewank_rosenbrock, shifted_rotated_griewank, shifted_rotated_happy_cat, shifted_rotated_hgbat, shifted_rotated_katsuura, shifted_rotated_rastrigin, shifted_rotated_rosenbrock, shifted_rotated_schwefel, shifted_rotated_weierstrass};
use crate::cec2014::unimodal::{rotated_bent_cigar, rotated_high_conditioned_elliptic};
use crate::cec2014::Cec14Helper;

/// f17: Hybrid Function 1
pub fn hf01(values: &[f64], o: &[f64], m: &[f64], s: &[usize], s_flag: bool, r_flag: bool, sh_flag: bool) -> f64 {
    let p_ratio = [0.3, 0.3, 0.4];
    let p = Cec14Helper::calculate_slices(&p_ratio, values.len());

    let mut x = values.to_vec();
    Cec14Helper::shift_sub(&mut x, o, s_flag);
    x = Cec14Helper::rotate(&x, m, r_flag);
    let x = Cec14Helper::shuffle(&x, s, sh_flag);
    println!("Shuffled:\n{:?}", x);

    let mut result: f64 = shifted_rotated_schwefel(&x[..p[0]], o, m, false, false);
    result += shifted_rotated_rastrigin(&x[p[0]..p[1]], o, m, false, false);
    result += rotated_high_conditioned_elliptic(&x[p[1]..], o, m, false, false);

    result
}

/// f18: Hybrid Function 2
pub fn hf02(values: &[f64], o: &[f64], m: &[f64], s: &[usize], s_flag: bool, r_flag: bool, sh_flag: bool) -> f64 {
    let p_ratio = [0.3, 0.3, 0.4];
    let p = Cec14Helper::calculate_slices(&p_ratio, values.len());

    let mut x = values.to_vec();
    Cec14Helper::shift_sub(&mut x, o, s_flag);
    x = Cec14Helper::rotate(&x, m, r_flag);
    let x = Cec14Helper::shuffle(&x, s, sh_flag);

    let mut result: f64 = rotated_bent_cigar(&x[..p[0]], o, m, false, false);
    result += shifted_rotated_hgbat(&x[p[0]..p[1]], o, m, false, false);
    result += shifted_rotated_rastrigin(&x[p[1]..], o, m, false, false);

    result
}


/// f19: Hybrid Function 3
pub fn hf03(values: &[f64], o: &[f64], m: &[f64], s: &[usize], s_flag: bool, r_flag: bool, sh_flag: bool) -> f64 {
    let p_ratio = [0.2, 0.2, 0.3, 0.3];
    let p = Cec14Helper::calculate_slices(&p_ratio, values.len());

    let mut x = values.to_vec();
    Cec14Helper::shift_sub(&mut x, o, s_flag);
    x = Cec14Helper::rotate(&x, m, r_flag);
    let x = Cec14Helper::shuffle(&x, s, sh_flag);

    let mut result: f64 = shifted_rotated_griewank(&x[..p[0]], o, m, false, false);
    result += shifted_rotated_weierstrass(&x[p[0]..p[1]], o, m, false, false);
    result += shifted_rotated_rosenbrock(&x[p[1]..p[2]], o, m, false, false);
    result += shifted_rotated_escaffer6(&x[p[2]..], o, m, false, false);

    result
}

/// f20: Hybrid Function 4
pub fn hf04(values: &[f64], o: &[f64], m: &[f64], s: &[usize], s_flag: bool, r_flag: bool, sh_flag: bool) -> f64 {
    let p_ratio = [0.2, 0.2, 0.3, 0.3];
    let p = Cec14Helper::calculate_slices(&p_ratio, values.len());

    let mut x = values.to_vec();
    Cec14Helper::shift_sub(&mut x, o, s_flag);
    x = Cec14Helper::rotate(&x, m, r_flag);
    let x = Cec14Helper::shuffle(&x, s, sh_flag);

    let mut result: f64 = shifted_rotated_hgbat(&x[..p[0]], o, m, false, false);
    result += discus(&x[p[0]..p[1]]);
    result += shifted_rotated_expanded_griewank_rosenbrock(&x[p[1]..p[2]], o, m, false, false);
    result += shifted_rotated_rastrigin(&x[p[2]..], o, m, false, false);

    result
}

/// f21: Hybrid Function 5
pub fn hf05(values: &[f64], o: &[f64], m: &[f64], s: &[usize], s_flag: bool, r_flag: bool, sh_flag: bool) -> f64 {
    let p_ratio = [0.1, 0.2, 0.2, 0.2, 0.3];
    let p = Cec14Helper::calculate_slices(&p_ratio, values.len());

    let mut x = values.to_vec();
    Cec14Helper::shift_sub(&mut x, o, s_flag);
    x = Cec14Helper::rotate(&x, m, r_flag);
    let x = Cec14Helper::shuffle(&x, s, sh_flag);

    let mut result: f64 = shifted_rotated_escaffer6(&x[..p[0]], o, m, false, false);
    result += shifted_rotated_hgbat(&x[p[0]..p[1]], o, m, false, false);
    result += shifted_rotated_rosenbrock(&x[p[1]..p[2]], o, m, false, false);
    result += shifted_rotated_schwefel(&x[p[2]..p[3]], o, m, false, false);
    result += rotated_high_conditioned_elliptic(&x[p[3]..], o, m, false, false);

    result
}

/// f22: Hybrid Function 6
pub fn hf06(values: &[f64], o: &[f64], m: &[f64], s: &[usize], s_flag: bool, r_flag: bool, sh_flag: bool) -> f64 {
    let p_ratio = [0.1, 0.2, 0.2, 0.2, 0.3];
    let p = Cec14Helper::calculate_slices(&p_ratio, values.len());

    let mut x = values.to_vec();
    Cec14Helper::shift_sub(&mut x, o, s_flag);
    x = Cec14Helper::rotate(&x, m, r_flag);
    let x = Cec14Helper::shuffle(&x, s, sh_flag);

    let mut result: f64 = shifted_rotated_katsuura(&x[..p[0]], o, m, false, false);
    result += shifted_rotated_happy_cat(&x[p[0]..p[1]], o, m, false, false);
    result += shifted_rotated_expanded_griewank_rosenbrock(&x[p[1]..p[2]], o, m, false, false);
    result += shifted_rotated_schwefel(&x[p[2]..p[3]], o, m, false, false);
    result += shifted_rotated_ackley(&x[p[3]..], o, m, false, false);

    result
}
