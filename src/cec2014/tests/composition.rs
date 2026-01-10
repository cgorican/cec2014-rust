use crate::cec2014::cec14_helper::Cec14Helper;
use crate::cec2014::composition::{cf01, cf02, cf03, cf04, cf05, cf06, cf07, cf08};
use crate::cec2014::tests::{TEST_VEC_10, TEST_VEC_20, TEST_VEC_30};

#[test]
fn test_cf01() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 23;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = cf01(&vec, &o, &m,  true) + fn_index as f64 * 100.0;
    assert_eq!(result, 2534.9167660742137);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = cf01(&vec, &o, &m, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 2608.2676369339256);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = cf01(&vec, &o, &m, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 3200.6530161269711);
}

#[test]
fn test_cf02() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 24;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = cf02(&vec, &o, &m,  true) + fn_index as f64 * 100.0;
    assert_eq!(result, 2603.508145243658);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = cf02(&vec, &o, &m, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 2609.8278295250111);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = cf02(&vec, &o, &m, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 2669.9172352749074);
}

#[test]
fn test_cf03() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 25;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = cf03(&vec, &o, &m,  true) + fn_index as f64 * 100.0;
    assert_eq!(result, 2700.2257891535792);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = cf03(&vec, &o, &m, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 2702.3944924483881);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = cf03(&vec, &o, &m, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 2713.943896567815);
}

#[test]
fn test_cf04() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 26;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = cf04(&vec, &o, &m,  true) + fn_index as f64 * 100.0;
    assert_eq!(result, 2800.0434579625266);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = cf04(&vec, &o, &m, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 2800.3942958585058);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = cf04(&vec, &o, &m, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 2810.9347994344917);
}

#[test]
fn test_cf05() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 27;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = cf05(&vec, &o, &m,  true) + fn_index as f64 * 100.0;
    assert_eq!(result, 3084.2866797111542);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = cf05(&vec, &o, &m, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 4452.7102243306663);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = cf05(&vec, &o, &m, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 23828.696225849166);
}

#[test]
fn test_cf06() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 28;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = cf06(&vec, &o, &m,  true) + fn_index as f64 * 100.0;
    assert_eq!(result, 3185.6123767976596);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = cf06(&vec, &o, &m, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 4353.4863815620165);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = cf06(&vec, &o, &m, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 19897.073071218449);
}

#[test]
fn test_cf07() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 29;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = cf07(&vec, &o, &m, &s,  true) + fn_index as f64 * 100.0;
    assert_eq!(result, 29189798.075958397);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = cf07(&vec, &o, &m, &s, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 80200643.162139624);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = cf07(&vec, &o, &m, &s, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 790017239.77496493);
}

#[test]
fn test_cf08() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 30;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = cf08(&vec, &o, &m, &s,  true) + fn_index as f64 * 100.0;
    assert_eq!(result, 5476248.1643524133);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = cf08(&vec, &o, &m, &s, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 13378731.715415224);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = cf08(&vec, &o, &m, &s, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 53227246.555207826);
}