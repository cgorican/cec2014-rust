use crate::cec2014::cec14_helper::Cec14Helper;
use crate::cec2014::multimodal::{shifted_rastrigin, shifted_rotated_ackley, shifted_rotated_escaffer6, shifted_rotated_expanded_griewank_rosenbrock, shifted_rotated_griewank, shifted_rotated_happy_cat, shifted_rotated_hgbat, shifted_rotated_katsuura, shifted_rotated_rastrigin, shifted_rotated_rosenbrock, shifted_rotated_schwefel, shifted_rotated_weierstrass, shifted_schwefel};
use crate::cec2014::tests::{TEST_VEC_10, TEST_VEC_20, TEST_VEC_30};

#[test]
fn test_shifted_rotated_rosenbrock() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 4;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_rosenbrock(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 12071.751197410289);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_rosenbrock(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 15999.526025495716);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_rosenbrock(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 28835.322735929374);
}

#[test]
fn test_shifted_rotated_ackley() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 5;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_ackley(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 521.7519755184717);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_ackley(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 521.5154474107057);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_ackley(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 521.8324744712833);
}

#[test]
fn test_shifted_rotated_weierstrass() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 6;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_weierstrass(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 614.9521900260443);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_weierstrass(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 632.9017469039933);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_weierstrass(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 651.6727321160001);
}

#[test]
fn test_shifted_rotated_griewank() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 7;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_griewank(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 1119.9426336080664);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_griewank(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 1146.9012907449785);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_griewank(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 1702.853180524241);
}

#[test]
fn test_shifted_rastrigin() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 8;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());

    let result = shifted_rastrigin(&vec, &o,  true) + fn_index as f64 * 100.0;
    assert_eq!(result, 986.7207379740212);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());

    let result = shifted_rastrigin(&vec, &o,  true) + fn_index as f64 * 100.0;
    assert_eq!(result, 1160.1628597015);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());

    let result = shifted_rastrigin(&vec, &o,  true) + fn_index as f64 * 100.0;
    assert_eq!(result, 1245.6331962694717);
}

#[test]
fn test_shifted_rotated_rastrigin() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 9;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_rastrigin(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 1008.7654750814751);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_rastrigin(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 1202.780373120963);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_rastrigin(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 1460.5951824370698);
}

#[test]
fn test_shifted_schwefel() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 10;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());

    let result = shifted_schwefel(&vec, &o, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 3466.351035023338);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());

    let result = shifted_schwefel(&vec, &o, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 7332.303615503373);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());

    let result = shifted_schwefel(&vec, &o, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 11179.924079575298);
}

#[test]
fn test_shifted_rotated_schwefel() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 11;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_schwefel(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 3939.054847192011);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_schwefel(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 7762.122271033737);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_schwefel(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 13690.549676015);
}

#[test]
fn test_shifted_rotated_katsuura() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 12;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_katsuura(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 1210.4255629033125);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_katsuura(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 1224.6900328607383);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_katsuura(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 1213.5697085543784);
}

#[test]
fn test_shifted_rotated_happy_cat() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 13;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_happy_cat(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 1308.0461045225156);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_happy_cat(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 1306.1996393528418);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_happy_cat(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 1311.1228423318164);
}

#[test]
fn test_shifted_rotated_hgbat() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 14;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_hgbat(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 1467.2624071080654);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_hgbat(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 1562.500214300778);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_hgbat(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 1772.8176204508932);
}

#[test]
fn test_shifted_rotated_expanded_griewank_plus_rosenbrock() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 15;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_expanded_griewank_rosenbrock(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 121892.58692162948);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_expanded_griewank_rosenbrock(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 177827.13331267799);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_expanded_griewank_rosenbrock(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 553998.51919292146);
}

#[test]
fn test_shifted_rotated_escaffer6() {
    let helper = Cec14Helper::default();
    let fn_index: usize = 16;

    let vec = TEST_VEC_10;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_escaffer6(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 1604.9905811879412);

    let vec = TEST_VEC_20;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_escaffer6(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 1610.1285115463768);

    let vec = TEST_VEC_30;
    let o = helper.load_shift_vector(fn_index, vec.len());
    let m = helper.load_rotation_matrix(fn_index, vec.len());

    let result = shifted_rotated_escaffer6(&vec, &o, &m, true, true) + fn_index as f64 * 100.0;
    assert_eq!(result, 1614.9177804929627);
}
