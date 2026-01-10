use std::f64::consts::{E, PI};

/// f1: High Conditioned Elliptic Function
pub fn high_conditioned_elliptic(values: &[f64]) -> f64 {
    let mut result = 0.0;

    let n_f64  = values.len() as f64;
    for i in 0..values.len() {
        result += 10f64.powf(6.0 * (i as f64) / (n_f64 - 1.0)) * values[i] * values[i];
    }

    result
}

/// f2: Bent Cigar Function
pub fn bent_cigar(values: &[f64]) -> f64 {
    if values.is_empty() {
        panic!("Bent cigar cannot be empty");
    }

    let mut result = values[0] * values[0];
    {
        let factor = 10f64.powi(6);
        for i in 1..values.len() {
            result += factor * values[i] * values[i];
        }
    }

    result
}

/// f3: Discus Function
pub fn discus(values: &[f64]) -> f64 {
    if values.is_empty() {
        panic!("Discus cannot be empty");
    }

    let mut result = 10f64.powi(6) * values[0] * values[0];

    for i in 1..values.len() {
        result += values[i] * values[i];
    }

    result
}

/// f4: Rosenbrock’s Function
#[allow(unused)]
pub fn rosenbrock(values: &[f64]) -> f64 {
    let mut result = 0.0;

    for i in 0..values.len().saturating_sub(1) {
        let p1 = values[i] * values[i] - values[i+1];
        let p2 = values[i] - 1.0;
        result += 100.0 * p1 * p1 + p2 * p2;
    }

    result
}

/// f5: Ackley’s Function
pub fn ackley(values: &[f64]) -> f64 {
    if values.is_empty() {
        panic!("Ackley cannot be empty");
    }

    let mut sum1 = 0.0;
    let mut sum2 = 0.0;
    for i in 0..values.len() {
        sum1 += values[i] * values[i];
        sum2 += (2.0 * PI * values[i]).cos();
    }

    let n_f64 = values.len() as f64;
    sum1 = -0.2 * (sum1 / n_f64).sqrt();
    sum2 = sum2 / n_f64;

    -20.0 * sum1.exp() - sum2.exp() + 20.0 + E
}

/// f6: Weierstrass Function
pub fn weierstrass(values: &[f64]) -> f64 {
    let mut result = 0.0;

    let a: f64 = 0.5;
    let b: f64 = 3.0;
    let k_max: i32 = 20;

    let mut sum1: f64;
    let mut sum2: f64 = 0.0;

    for i in 0..values.len() {
        sum1 = 0.0;
        for j in 0..=k_max {
            sum1 += a.powi(j) * (2.0 * PI * b.powi(j) * (values[i] + 0.5)).cos();
            if i == 0 {
                sum2 += a.powi(j) * (2.0 * PI * b.powi(j) * 0.5).cos();
            }
        }
        result += sum1;
    }
    result -= (values.len() as f64) * sum2;

    result
}

/// f7: Griewank’s Function
pub fn griewank(values: &[f64]) -> f64 {
    let mut sum = 0.0;
    let mut prod = 1.0;

    for i in 0..values.len() {
        sum += values[i] * values[i] / 4000.0;
        prod *= (values[i] / (i as f64 + 1.0).sqrt()).cos()
    }

    1.0 + sum - prod
}

/// f8: Rastrigin’s Function
pub fn rastrigin(values: &[f64]) -> f64 {
    let mut result = 0.0;

    for i in 0..values.len() {
        result += values[i] * values[i] - 10.0 * (2.0 * PI * values[i]).cos() + 10.0;
    }

    result
}

/// f9: Modified Schwefel’s Function
pub fn modified_schwefel(values: &[f64]) -> f64 {
    if values.is_empty() {
        panic!("Schwefel cannot be empty");
    }
    let mut result = 0.0;
    let n_f64 = values.len() as f64;

    for i in 0..values.len() {
        let z_i = values[i] + 4.209687462275036e+002;

        if z_i > 500.0 {
            let p1 = 500.0 - (z_i % 500.0);
            let p2 = p1.abs().sqrt().sin();

            let mut p3 = (z_i - 500.0) / 100.0;
            p3 = p3 * p3 / n_f64;

            result += p1 * p2 - p3
        } else if z_i < -500.0 {
            let p1 = (z_i.abs() % 500.0) - 500.0;
            let p2 = p1.abs().sqrt().sin();

            let mut p3 = (z_i + 500.0) / 100.0;
            p3 = p3 * p3 / n_f64;

            result += p1 * p2 - p3;
        } else {
            result += z_i * z_i.abs().sqrt().sin();
        }
    }

    // 418.9829 * n_f64 - result
    4.189_828_872_724_338e2 * n_f64 - result
}

/// f10: Katsuura Function
pub fn katsuura(values: &[f64]) -> f64 {
    if values.is_empty() {
        panic!("Katsuura cannot be empty");
    }
    let mut result = 1.0;
    let n_f64 = values.len() as f64;

    let exp = 10.0 / n_f64.powf(1.2);

    let mut two_j: f64;
    let mut sum: f64;
    for i in 0..values.len() {
        sum = 0.0;
        for j in 1..=32 {
            two_j = 2f64.powi(j);
            sum += (two_j * values[i] - f64::round(two_j * values[i])).abs() / two_j;
        }
        result *= (1.0 + (i as f64 + 1.0) * sum).powf(exp);
    }

    let factor = 10.0 / (n_f64 * n_f64);

    factor * result - factor
}

/// f11: HappyCat Function
pub fn happy_cat(values: &[f64]) -> f64 {
    /*HappyCat, provided by Hans-Georg Beyer (HGB)*/ /*original global optimum: [-1,-1,...,-1]*/
    if values.is_empty() {
        panic!("HappyCat cannot be empty");
    }
    let n_f64 = values.len() as f64;

    let mut sum_x = 0f64;
    let mut x2 = 0f64;
    let mut shifted_x: f64;
    for i in 0..values.len() {
        shifted_x = values[i] - 1.0; // shift to origin
        x2 += shifted_x * shifted_x;
        sum_x += shifted_x;
    }

    // 2 * 1 / 8 = 0.25
    let result = (x2 - n_f64).abs().powf(0.25);

    result + (0.5 * x2 + sum_x) / n_f64 + 0.5
}

/// f12: HGBat Function
pub fn hgbat(values: &[f64]) -> f64 {
    /*HGBat, provided by Hans-Georg Beyer (HGB)*/ /*original global optimum: [-1,-1,...-1]*/
    if values.is_empty() {
        panic!("HGBat cannot be empty");
    }
    let n_f64 = values.len() as f64;

    let mut sum_x = 0f64;
    let mut sum_x2 = 0f64;
    let mut shifted_x: f64;
    for i in 0..values.len() {
        shifted_x = values[i] - 1.0; // shift to origin
        sum_x2 += shifted_x * shifted_x;
        sum_x += shifted_x;
    }

    let p1 = (sum_x2 * sum_x2 - sum_x * sum_x).abs().sqrt();
    let p2 = (0.5 * sum_x2 + sum_x) / n_f64;

    p1 + p2 + 0.5
}

/// f13: Expanded Griewank’s plus Rosenbrock’s Function
#[allow(unused)]
pub fn expanded_griewank_rosenbrock(values: &[f64]) -> f64 {
    if values.len() < 2 {
        panic!("Expanded Griewank and Rosenbrok cannot be less than 2");
    }
    let mut result = 0.0;
    let n = values.len();

    let mut p1: f64;
    let mut p2: f64;
    let mut p_rosen: f64;
    for i in 0..n.saturating_sub(1) {
        p1 = values[i] * values[i] - values[i+1];
        p2 = values[i] - 1.0;

        p_rosen = 100.0 * p1 * p1 + p2 * p2;
        result += (p_rosen * p_rosen) / 4000.0 - p_rosen.cos() + 1.0;
    }
    p1 = values[n-1] * values[n-1] - values[0];
    p2 = values[n-1] - 1.0;

    p_rosen = 100.0 * p1 * p1 + p2 * p2;
    result += (p_rosen * p_rosen) / 4000.0 - p_rosen.cos() + 1.0;

    result
}

/// f14: Expanded Scaffer’s F6 Function
pub fn escaffer6(values: &[f64]) -> f64 {
    let mut result = 0.0;
    let n = values.len();

    let mut p1: f64;
    let mut p2: f64;
    for i in 0..n.saturating_sub(1) {
        p1 = values[i] * values[i] + values[i+1] * values[i+1];
        p2 = 1.0 + 0.001 * p1;

        p1 = p1.sqrt().sin();
        p1 = p1 * p1;

        result += 0.5 + (p1 - 0.5) / (p2 * p2);
    }
    p1 = values[n-1] * values[n-1] + values[0] * values[0];
    p2 = 1.0 + 0.001 * p1;

    p1 = p1.sqrt().sin();
    p1 = p1 * p1;

    result += 0.5 + (p1 - 0.5) / (p2 * p2);

    result
}
