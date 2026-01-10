use crate::cec2014::cec14_helper::Cec14Helper;
use crate::cec2014::hybrid::{hf01, hf02, hf03, hf04, hf05, hf06};
use crate::cec2014::tests::{TEST_VEC_10, TEST_VEC_20, TEST_VEC_30};

#[test]
fn test_hf01() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 17;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = hf01(&vec, &o, &m, &s, true,  true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 32129128.035634216);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = hf01(&vec, &o, &m, &s, true,  true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 68500413.551766694);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = hf01(&vec, &o, &m, &s, true,  true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 1352550509.3263743);
}

#[test]
fn test_hf02() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 18;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = hf02(&vec, &o, &m, &s, true,  true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 208929221.07173982);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = hf02(&vec, &o, &m, &s, true,  true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 1580890580.2670112);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = hf02(&vec, &o, &m, &s, true,  true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 16379326100.921312);
}

#[test]
fn test_hf03() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 19;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = hf03(&vec, &o, &m, &s, true,  true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 3109.6406028826796);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = hf03(&vec, &o, &m, &s, true,  true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 7530.1506346233627);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = hf03(&vec, &o, &m, &s, true,  true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 2827.4231075855005);
}

#[test]
fn test_hf04() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 20;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = hf04(&vec, &o, &m, &s, true,  true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 805083222.97022259);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = hf04(&vec, &o, &m, &s, true,  true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 785004371.39867687);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = hf04(&vec, &o, &m, &s, true,  true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 2582491969.4259782);
}

#[test]
fn test_hf05() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 21;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = hf05(&vec, &o, &m, &s, true,  true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 2779210676.6228242);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = hf05(&vec, &o, &m, &s, true,  true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 811992415.60897255);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = hf05(&vec, &o, &m, &s, true,  true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 2422469924.9205689);
}

#[test]
fn test_hf06() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 22;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = hf06(&vec, &o, &m, &s, true,  true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 11523.661917876705);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = hf06(&vec, &o, &m, &s, true,  true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 106309.46943398094);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());
    let s = helper.load_shuffle_vector(fn_index, vec.len());

    let result = hf06(&vec, &o, &m, &s, true,  true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 5997042.1588237528);
}