// Tests for the public wrapper `Cec14::eval`, which the DDE-ARA code calls.
// The other test files call the raw functions and add `fn_index * 100` by hand,
// so they never exercised this wrapper.
use crate::cec2014::cec14::Cec14;
use crate::cec2014::cec14_function::Cec14Function;
use crate::cec2014::cec14_helper::Cec14Helper;
use crate::cec2014::tests::TEST_VEC_10;

fn function(index: i32) -> Cec14Function {
    Cec14Function::try_from(index).expect("valid CEC14 function index")
}

/// At the shift vector (the optimum) every function must return an error of ~0.
/// Checked for D=10 and D=50, since the benchmark runs D=50.
#[test]
fn eval_is_zero_at_the_optimum() {
    let helper = Cec14Helper::default();
    let mut failures: Vec<String> = Vec::new();

    for dim in [10usize, 50] {
        for index in 1..=30i32 {
            let cec = Cec14::new(function(index), dim);
            // composition functions store one shift row per component, the first is the global optimum
            let o = helper.load_shift_vector(index as usize, dim);
            let value = cec.eval(&o[..dim]);
            if !value.is_finite() || value.abs() > 1e-6 {
                failures.push(format!("f{index} D={dim}: eval(o) = {value:e}"));
            }
        }
    }

    assert!(failures.is_empty(), "eval(o) should be ~0:\n{}", failures.join("\n"));
}

/// `Cec14::eval` must return exactly what the enum dispatch returns, with no extra scaling.
/// (The previous implementation multiplied by `index * 100`, which is invisible at the optimum
/// because 0 * anything = 0, so this test uses a non-optimal point.)
#[test]
fn eval_matches_raw_function_away_from_the_optimum() {
    let helper = Cec14Helper::default();
    let dim = TEST_VEC_10.len();
    let mut failures: Vec<String> = Vec::new();

    for index in 1..=30i32 {
        let f = function(index);
        let o = helper.load_shift_vector(index as usize, dim);
        let m = helper.load_rotation_matrix(index as usize, dim);
        let s = helper.load_shuffle_vector(index as usize, dim);

        let raw = f.eval(&TEST_VEC_10, &o, &m, &s);
        let wrapped = Cec14::new(f, dim).eval(&TEST_VEC_10);

        let tol = 1e-9 * raw.abs().max(1.0);
        if (raw - wrapped).abs() > tol {
            failures.push(format!("f{index}: raw = {raw:e}, Cec14::eval = {wrapped:e}"));
        }
    }

    assert!(failures.is_empty(), "Cec14::eval differs from the raw function:\n{}", failures.join("\n"));
}

/// Anchor to the C reference values already used in the unimodal/composition tests:
/// reference value = error + f* (= index * 100).
#[test]
fn eval_plus_bias_matches_reference_values() {
    let f1 = Cec14::new(function(1), 10).eval(&TEST_VEC_10) + 100.0;
    assert!((f1 - 4702928009.420022).abs() < 1e-6 * 4702928009.420022, "f1 = {f1}");

    let f23 = Cec14::new(function(23), 10).eval(&TEST_VEC_10) + 2300.0;
    assert!((f23 - 2534.9167660742137).abs() < 1e-6 * 2534.9167660742137, "f23 = {f23}");
}
