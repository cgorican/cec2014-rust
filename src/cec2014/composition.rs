use crate::cec2014::hybrid::{hf01, hf02, hf03, hf04, hf05, hf06};
use crate::cec2014::multimodal::{shifted_rotated_escaffer6, shifted_rotated_expanded_griewank_rosenbrock, shifted_rotated_griewank, shifted_rotated_happy_cat, shifted_rotated_hgbat, shifted_rotated_rastrigin, shifted_rotated_rosenbrock, shifted_rotated_schwefel, shifted_rotated_weierstrass};
use crate::cec2014::unimodal::{rotated_bent_cigar, rotated_discus, rotated_high_conditioned_elliptic};

const INF: f64 = 1e99;

fn cf_cal(values: &[f64], fit: &mut [f64], o: &[f64], delta: &[f64], bias: &[f64], cf_num: usize) -> f64 {
    let mut w: Vec<f64> = vec![0.0; cf_num];
    let mut w_max: f64 = 0.0;

    let n_f64 = values.len() as f64;
    for i in 0..cf_num {
        fit[i] += bias[i];

        let offset = i * values.len();
        let dist_sq = values.iter()
            .zip(&o[offset..offset + values.len()])
            .map(|(v, o_j)| {
                let diff = v - o_j;
                diff * diff
            })
            .sum::<f64>();

        w[i] = if dist_sq == 0.0 {
            INF
        } else {
            (1.0 / dist_sq).powf(0.5) * (-dist_sq / (2.0 * n_f64 * delta[i].powi(2))).exp()
        };

        if w[i] > w_max {
            w_max = w[i];
        }
    }

    let mut w_sum: f64 = w.iter().sum::<f64>();

    if w_max == 0.0 {
        w.iter_mut()
            .for_each(|wi| *wi = 1.0);
        w_sum = cf_num as f64;
    }

    let result: f64 = if w_sum > 0.0 {
        let inv = 1.0 / w_sum;
        w.iter()
            .zip(fit.iter())
            .map(|(w_i, fit_i)| w_i * fit_i * inv)
            .sum()
    } else {
        0.0
    };

    result
}

/// f23: Composition Function 1
pub fn cf01(values: &[f64], o: &[f64], m: &[f64], r_flag: bool) -> f64 {
    let cf_num = 5;
    let mut fit = [0.0; 5];
    let delta = [10.0, 20.0, 30.0, 40.0, 50.0];
    let bias = [0.0, 100.0, 200.0, 300.0, 400.0];
    let lambda = [1.0, 1e-6, 1e-26, 1e-6, 1e-6];

    let n = values.len();
    let n2 = n * n;

    for i in 0..cf_num {
        let os = &o[i * n .. (i + 1) * n];
        let ms = &m[i * n2 .. (i + 1) * n2];

        fit[i] = match i {
            0 => shifted_rotated_rosenbrock(values, os, ms, true, r_flag), // lambda[0] is 1.0
            1 => rotated_high_conditioned_elliptic(values, os, ms, true, r_flag) * lambda[1],
            2 => rotated_bent_cigar(values, os, ms, true, r_flag) * lambda[2],
            3 => rotated_discus(values, os, ms, true, r_flag) * lambda[3],
            4 => rotated_high_conditioned_elliptic(values, os, ms, true, false) * lambda[4],
            _ => panic!("Invalid function index")
        };
    }

    cf_cal(values, &mut fit, o, &delta, &bias, cf_num)
}

/// f24: Composition Function 2
pub fn cf02(values: &[f64], o: &[f64], m: &[f64], r_flag: bool) -> f64 {
    let cf_num = 3;
    let mut fit = [0.0; 3];
    let delta = [20.0; 3];
    let bias = [0.0, 100.0, 200.0];
    // let lambda = [1.0; 3];

    for i in 0..cf_num {
        let os = &o[i * values.len() .. (i + 1) * values.len()];
        let ms = &m[i * values.len() * values.len() .. (i + 1) * values.len() * values.len()];

        fit[i] = match i {
            0 => shifted_rotated_schwefel(values, os, ms, true, false),
            1 => shifted_rotated_rastrigin(values, os, ms, true, r_flag),
            2 => shifted_rotated_hgbat(values, os, ms, true, r_flag),
            _ => panic!("Invalid function index")
        };
    }

    cf_cal(values, &mut fit, o, &delta, &bias, cf_num)
}

/// f25: Composition Function 3
pub fn cf03(values: &[f64], o: &[f64], m: &[f64], r_flag: bool) -> f64 {
    let cf_num = 3;
    let mut fit = [0.0; 3];
    let delta = [10.0, 30.0, 50.0];
    let bias = [0.0, 100.0, 200.0];
    let lambda = [0.25, 1.0, 1e-7];

    for i in 0..cf_num {
        let os = &o[i * values.len() .. (i + 1) * values.len()];
        let ms = &m[i * values.len() * values.len() .. (i + 1) * values.len() * values.len()];

        fit[i] = match i {
            0 => shifted_rotated_schwefel(values, os, ms, true, r_flag) * lambda[0],
            1 => shifted_rotated_rastrigin(values, os, ms, true, r_flag), // lambda[1] is 1.0
            2 => rotated_high_conditioned_elliptic(values, os, ms, true, r_flag) * lambda[2],
            _ => panic!("Invalid function index")
        };
    }

    cf_cal(values, &mut fit, o, &delta, &bias, cf_num)
}

/// f26: Composition Function 4
pub fn cf04(values: &[f64], o: &[f64], m: &[f64], r_flag: bool) -> f64 {
    let cf_num = 5;
    let mut fit = [0.0; 5];
    let delta = [10.0; 5];
    let bias = [0.0, 100.0, 200.0, 300.0, 400.0];
    let lambda = [0.25, 1.0, 1e-7, 2.5, 10.0];

    for i in 0..cf_num {
        let os = &o[i * values.len() .. (i + 1) * values.len()];
        let ms = &m[i * values.len() * values.len() .. (i + 1) * values.len() * values.len()];

        fit[i] = match i {
            0 => shifted_rotated_schwefel(values, os, ms, true, r_flag) * lambda[0],
            1 => shifted_rotated_happy_cat(values, os, ms, true, r_flag), // lambda[1] is 1.0
            2 => rotated_high_conditioned_elliptic(values, os, ms, true, r_flag) * lambda[2],
            3 => shifted_rotated_weierstrass(values, os, ms, true, r_flag) * lambda[3],
            4 => shifted_rotated_griewank(values, os, ms, true, r_flag) * lambda[4],
            _ => panic!("Invalid function index")
        };
    }

    cf_cal(values, &mut fit, o, &delta, &bias, cf_num)
}

/// f27: Composition Function 5
pub fn cf05(values: &[f64], o: &[f64], m: &[f64], r_flag: bool) -> f64 {
    let cf_num = 5;
    let mut fit = [0.0; 5];
    let delta = [10.0, 10.0, 10.0, 20.0, 20.0];
    let bias = [0.0, 100.0, 200.0, 300.0, 400.0];
    let lambda = [10.0, 10.0, 2.5, 25.0, 1e-6];

    for i in 0..cf_num {
        let os = &o[i * values.len() .. (i + 1) * values.len()];
        let ms = &m[i * values.len() * values.len() .. (i + 1) * values.len() * values.len()];

        fit[i] = match i {
            0 => shifted_rotated_hgbat(values, os, ms, true, r_flag) * lambda[0],
            1 => shifted_rotated_rastrigin(values, os, ms, true, r_flag) * lambda[1],
            2 => shifted_rotated_schwefel(values, os, ms, true, r_flag) * lambda[2],
            3 => shifted_rotated_weierstrass(values, os, ms, true, r_flag) * lambda[3],
            4 => rotated_high_conditioned_elliptic(values, os, ms, true, r_flag) * lambda[4],
            _ => panic!("Invalid function index")
        };
    }

    cf_cal(values, &mut fit, o, &delta, &bias, cf_num)
}

/// f28: Composition Function 6
pub fn cf06(values: &[f64], o: &[f64], m: &[f64], r_flag: bool) -> f64 {
    let cf_num = 5;
    let mut fit = [0.0; 5];
    let delta = [10.0, 20.0, 30.0, 40.0, 50.0];
    let bias = [0.0, 100.0, 200.0, 300.0, 400.0];
    let lambda = [2.5, 10.0, 2.5, 5e-4, 1e-6];

    for i in 0..cf_num {
        let os = &o[i * values.len() .. (i + 1) * values.len()];
        let ms = &m[i * values.len() * values.len() .. (i + 1) * values.len() * values.len()];

        fit[i] = match i {
            0 => shifted_rotated_expanded_griewank_rosenbrock(values, os, ms, true, r_flag) * lambda[0],
            1 => shifted_rotated_happy_cat(values, os, ms, true, r_flag) * lambda[1],
            2 => shifted_rotated_schwefel(values, os, ms, true, r_flag) * lambda[2],
            3 => shifted_rotated_escaffer6(values, os, ms, true, r_flag) * lambda[3],
            4 => rotated_high_conditioned_elliptic(values, os, ms, true, r_flag) * lambda[4],
            _ => panic!("Invalid function index")
        };
    }

    cf_cal(values, &mut fit, o, &delta, &bias, cf_num)
}

/// f29: Composition Function 7
pub fn cf07(values: &[f64], o: &[f64], m: &[f64], s: &[usize], r_flag: bool) -> f64 {
    let cf_num = 3;
    let mut fit = [0.0; 3];
    let delta = [10.0, 30.0, 50.0];
    let bias = [0.0, 100.0, 200.0];
    // let lambda = [1.0; 3];

    for i in 0..cf_num {
        let os = &o[i * values.len() .. (i + 1) * values.len()];
        let ms = &m[i * values.len() * values.len() .. (i + 1) * values.len() * values.len()];
        let ss = &s[i * values.len() .. (i + 1) * values.len()];

        fit[i] = match i {
            0 => hf01(values, os, ms, ss, true, r_flag, true),
            1 => hf02(values, os, ms, ss, true, r_flag, true),
            2 => hf03(values, os, ms, ss, true, r_flag, true),
            _ => panic!("Invalid function index")
        };
    }

    cf_cal(values, &mut fit, o, &delta, &bias, cf_num)
}

/// f30: Composition Function 8
pub fn cf08(values: &[f64], o: &[f64], m: &[f64], s: &[usize], r_flag: bool) -> f64 {
    let cf_num = 3;
    let mut fit = [0.0; 3];
    let delta = [10.0, 30.0, 50.0];
    let bias = [0.0, 100.0, 200.0];
    // let lambda = [1.0; 3];

    for i in 0..cf_num {
        let os = &o[i * values.len() .. (i + 1) * values.len()];
        let ms = &m[i * values.len() * values.len() .. (i + 1) * values.len() * values.len()];
        let ss = &s[i * values.len() .. (i + 1) * values.len()];

        fit[i] = match i {
            0 => hf04(values, os, ms, ss, true, r_flag, true),
            1 => hf05(values, os, ms, ss, true, r_flag, true),
            2 => hf06(values, os, ms, ss, true, r_flag, true),
            _ => panic!("Invalid function index")
        };
    }

    cf_cal(values, &mut fit, o, &delta, &bias, cf_num)
}
