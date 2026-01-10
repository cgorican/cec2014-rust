use crate::cec2014::basic_functions::{high_conditioned_elliptic, bent_cigar, discus};
use crate::cec2014::Cec14Helper;

/// Rotated High Conditioned Elliptic Function
pub fn rotated_high_conditioned_elliptic(values: &[f64], o: &[f64], m: &[f64], s_flag: bool, r_flag: bool) -> f64 {
    let mut x = values.to_vec();
    Cec14Helper::shift_sub(&mut x, o, s_flag);
    let x = Cec14Helper::rotate(&x, m, r_flag);

    high_conditioned_elliptic(&x)
}

/// Rotated Bent Cigar Function
pub fn rotated_bent_cigar(values: &[f64], o: &[f64], m: &[f64], s_flag: bool, r_flag: bool) -> f64 {
    let mut x = values.to_vec();
    Cec14Helper::shift_sub(&mut x, o, s_flag);
    let x = Cec14Helper::rotate(&x, m, r_flag);

    bent_cigar(&x)
}


/// Rotated Discus Function
pub fn rotated_discus(values: &[f64], o: &[f64], m: &[f64], s_flag: bool, r_flag: bool) -> f64 {
    let mut x = values.to_vec();
    Cec14Helper::shift_sub(&mut x, o, s_flag);
    let x = Cec14Helper::rotate(&x, m, r_flag);

    discus(&x)
}
