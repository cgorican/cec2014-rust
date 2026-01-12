use crate::cec2014::multimodal::{shifted_rastrigin, shifted_rotated_ackley, shifted_rotated_escaffer6, shifted_rotated_expanded_griewank_rosenbrock, shifted_rotated_griewank, shifted_rotated_happy_cat, shifted_rotated_hgbat, shifted_rotated_katsuura, shifted_rotated_rastrigin, shifted_rotated_rosenbrock, shifted_rotated_schwefel, shifted_rotated_weierstrass, shifted_schwefel};
use crate::cec2014::unimodal::{rotated_bent_cigar, rotated_discus, rotated_high_conditioned_elliptic};
use crate::{cf01, cf02, cf03, cf04, cf05, cf06, cf07, cf08, hf01, hf02, hf03, hf04, hf05, hf06};

#[derive(Clone, Debug)]
pub enum Cec14Function {
    F1 = 1, // Rotated High Conditioned Elliptic
    F2 = 2, // Rotated Bent Cigar
    F3 = 3, // Rotated Discus
    F4 = 4, // Shifted Rotated Rosenbrock
    F5 = 5, // Shifted Rotated Ackley
    F6 = 6, // Shifted Rotated Weierstrass
    F7 = 7, // Shifted Rotated Griewank
    F8 = 8, // Shifted Rastrigin
    F9 = 9, // Shifted Rotated Rastrigin
    F10 = 10,  // Shifted Schwefel
    F11 = 11, // Shifted Rotated Schwefel
    F12 = 12, // Shifted Rotated Katsuura
    F13 = 13, // Shifted Rotated HappyCat
    F14 = 14, // Shifted Rotated Hgbat
    F15 = 15, // Shifted Rotated Expanded Griewank-Rosenbrock
    F16 = 16, // Shifted Rotated Escaffer6
    F17 = 17, // Hf01
    F18 = 18, // Hf02
    F19 = 19, // Hf03
    F20 = 20, // Hf04
    F21 = 21, // Hf05
    F22 = 22, // Hf06
    F23 = 23, // Cf01
    F24 = 24, // Cf02
    F25 = 25, // Cf03
    F26 = 26, // Cf04
    F27 = 27, // Cf05
    F28 = 28, // Cf06
    F29 = 29, // Cf07
    F30 = 30, // Cf08
}

impl Cec14Function {
    pub fn eval(&self, values: &[f64], o: &[f64], m: &[f64], s: &[usize]) -> f64 {
        match self {
            Cec14Function::F1 => rotated_high_conditioned_elliptic(values, &o, &m, true, true),
            Cec14Function::F2 => rotated_bent_cigar(values, &o, &m, true, true),
            Cec14Function::F3 => rotated_discus(values, &o, &m, true, true),
            Cec14Function::F4 => shifted_rotated_rosenbrock(values, &o, &m, true, true),
            Cec14Function::F5 => shifted_rotated_ackley(values, &o, &m, true, true),
            Cec14Function::F6 => shifted_rotated_weierstrass(values, &o, &m, true, true),
            Cec14Function::F7 => shifted_rotated_griewank(values, &o, &m, true, true),
            Cec14Function::F8 => shifted_rastrigin(values, &o, true),
            Cec14Function::F9 => shifted_rotated_rastrigin(values, &o, &m, true, true),
            Cec14Function::F10 => shifted_schwefel(values, &o, true),
            Cec14Function::F11 => shifted_rotated_schwefel(values, &o, &m, true, true),
            Cec14Function::F12 => shifted_rotated_katsuura(values, &o, &m, true, true),
            Cec14Function::F13 => shifted_rotated_happy_cat(values, &o, &m, true, true),
            Cec14Function::F14 => shifted_rotated_hgbat(values, &o, &m, true, true),
            Cec14Function::F15 => shifted_rotated_expanded_griewank_rosenbrock(values, &o, &m, true, true),
            Cec14Function::F16 => shifted_rotated_escaffer6(values, &o, &m, true, true),
            Cec14Function::F17 => hf01(values, &o, &m, &s, true, true, true),
            Cec14Function::F18 => hf02(values, &o, &m, &s, true, true, true),
            Cec14Function::F19 => hf03(values, &o, &m, &s, true, true, true),
            Cec14Function::F20 => hf04(values, &o, &m, &s, true, true, true),
            Cec14Function::F21 => hf05(values, &o, &m, &s, true, true, true),
            Cec14Function::F22 => hf06(values, &o, &m, &s, true, true, true),
            Cec14Function::F23 => cf01(values, &o, &m, true),
            Cec14Function::F24 => cf02(values, &o, &m, true),
            Cec14Function::F25 => cf03(values, &o, &m, true),
            Cec14Function::F26 => cf04(values, &o, &m, true),
            Cec14Function::F27 => cf05(values, &o, &m, true),
            Cec14Function::F28 => cf06(values, &o, &m, true),
            Cec14Function::F29 => cf07(values, &o, &m, &s, true),
            Cec14Function::F30 => cf08(values, &o, &m, &s, true),
        }
    }

    pub fn index(&self) -> usize {
        self.clone() as usize
    }
}
