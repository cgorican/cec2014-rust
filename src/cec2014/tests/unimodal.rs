use crate::cec2014::cec14_helper::Cec14Helper;
use crate::cec2014::unimodal::{rotated_high_conditioned_elliptic, rotated_bent_cigar, rotated_discus};
use crate::cec2014::tests::{TEST_VEC_10, TEST_VEC_20, TEST_VEC_30};

#[test]
fn test_rotated_high_conditioned_elliptic() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 1;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = rotated_high_conditioned_elliptic(&vec, &o, &m, true,  true) + fn_index as f64 * 100.0;
    assert_eq!(result, 4702928009.420022);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = rotated_high_conditioned_elliptic(&vec, &o, &m, true,  true) + fn_index as f64 * 100.0;
    assert_eq!(result, 4785456826.343216);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = rotated_high_conditioned_elliptic(&vec, &o, &m, true,  true) + fn_index as f64 * 100.0;
    assert_eq!(result, 2472944979.900850);
}

#[test]
fn test_rotated_bent_cigar() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 2;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = rotated_bent_cigar(&vec, &o, &m, true,  true) + fn_index as f64 * 100.0;

    assert_eq!(result, 16685331971.274801);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = rotated_bent_cigar(&vec, &o, &m, true,  true) + fn_index as f64 * 100.0;

    assert_eq!(result, 39767544516.665031);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = rotated_bent_cigar(&vec, &o, &m, true,  true) + fn_index as f64 * 100.0;

    assert_eq!(result, 104962377253.367065);
}

#[test]
fn test_rotated_discus() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 3;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = rotated_discus(&vec, &o, &m, true,  true) + fn_index as f64 * 100.0;
    assert_eq!(result, 12394973.845475068);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = rotated_discus(&vec, &o, &m, true,  true) + fn_index as f64 * 100.0;
    assert_eq!(result, 682488963.9419997);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = rotated_discus(&vec, &o, &m, true,  true) + fn_index as f64 * 100.0;
    assert_eq!(result, 131032568.29311569);
}
