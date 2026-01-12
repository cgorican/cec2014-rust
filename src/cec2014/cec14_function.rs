use crate::cec2014::multimodal::{shifted_rastrigin, shifted_rotated_ackley, shifted_rotated_escaffer6, shifted_rotated_expanded_griewank_rosenbrock, shifted_rotated_griewank, shifted_rotated_happy_cat, shifted_rotated_hgbat, shifted_rotated_katsuura, shifted_rotated_rastrigin, shifted_rotated_rosenbrock, shifted_rotated_schwefel, shifted_rotated_weierstrass, shifted_schwefel};
use crate::cec2014::unimodal::{rotated_bent_cigar, rotated_discus, rotated_high_conditioned_elliptic};
use crate::{cf01, cf02, cf03, cf04, cf05, cf06, cf07, cf08, hf01, hf02, hf03, hf04, hf05, hf06};

#[derive(Clone, Debug)]
pub enum Cec14Function {
    RotatedHighConditionedElliptic = 1,
    RotatedBentCigar = 2,
    RotatedDiscus = 3,
    ShiftedRotatedRosenbrock = 4,
    ShiftedRotatedAckley = 5,
    ShiftedRotatedWeierstrass = 6,
    ShiftedRotatedGriewank = 7,
    ShiftedRastrigin = 8,
    ShiftedRotatedRastrigin = 9,
    ShiftedSchwefel = 10,
    ShiftedRotatedSchwefel = 11,
    ShiftedRotatedKatsuura = 12,
    ShiftedRotatedHappyCat = 13,
    ShiftedRotatedHgbat = 14,
    ShiftedRotatedExpandedGriewankRosenbrock = 15,
    ShiftedRotatedEscaffer6 = 16,
    Hf01 = 17,
    Hf02 = 18,
    Hf03 = 19,
    Hf04 = 20,
    Hf05 = 21,
    Hf06 = 22,
    Cf01 = 23,
    Cf02 = 24,
    Cf03 = 25,
    Cf04 = 26,
    Cf05 = 27,
    Cf06 = 28,
    Cf07 = 29,
    Cf08 = 30,
}

impl Cec14Function {
    pub fn eval(&self, values: &[f64], o: &[f64], m: &[f64], s: &[usize]) -> f64 {
        match self {
            Cec14Function::RotatedHighConditionedElliptic => rotated_high_conditioned_elliptic(values, &o, &m, true, true),
            Cec14Function::RotatedBentCigar => rotated_bent_cigar(values, &o, &m, true, true),
            Cec14Function::RotatedDiscus => rotated_discus(values, &o, &m, true, true),
            Cec14Function::ShiftedRotatedRosenbrock => shifted_rotated_rosenbrock(values, &o, &m, true, true),
            Cec14Function::ShiftedRotatedAckley => shifted_rotated_ackley(values, &o, &m, true, true),
            Cec14Function::ShiftedRotatedWeierstrass => shifted_rotated_weierstrass(values, &o, &m, true, true),
            Cec14Function::ShiftedRotatedGriewank => shifted_rotated_griewank(values, &o, &m, true, true),
            Cec14Function::ShiftedRastrigin => shifted_rastrigin(values, &o, true),
            Cec14Function::ShiftedRotatedRastrigin => shifted_rotated_rastrigin(values, &o, &m, true, true),
            Cec14Function::ShiftedSchwefel => shifted_schwefel(values, &o, true),
            Cec14Function::ShiftedRotatedSchwefel => shifted_rotated_schwefel(values, &o, &m, true, true),
            Cec14Function::ShiftedRotatedKatsuura => shifted_rotated_katsuura(values, &o, &m, true, true),
            Cec14Function::ShiftedRotatedHappyCat => shifted_rotated_happy_cat(values, &o, &m, true, true),
            Cec14Function::ShiftedRotatedHgbat => shifted_rotated_hgbat(values, &o, &m, true, true),
            Cec14Function::ShiftedRotatedExpandedGriewankRosenbrock => shifted_rotated_expanded_griewank_rosenbrock(values, &o, &m, true, true),
            Cec14Function::ShiftedRotatedEscaffer6 => shifted_rotated_escaffer6(values, &o, &m, true, true),
            Cec14Function::Hf01 => hf01(values, &o, &m, &s, true, true, true),
            Cec14Function::Hf02 => hf02(values, &o, &m, &s, true, true, true),
            Cec14Function::Hf03 => hf03(values, &o, &m, &s, true, true, true),
            Cec14Function::Hf04 => hf04(values, &o, &m, &s, true, true, true),
            Cec14Function::Hf05 => hf05(values, &o, &m, &s, true, true, true),
            Cec14Function::Hf06 => hf06(values, &o, &m, &s, true, true, true),
            Cec14Function::Cf01 => cf01(values, &o, &m, true),
            Cec14Function::Cf02 => cf02(values, &o, &m, true),
            Cec14Function::Cf03 => cf03(values, &o, &m, true),
            Cec14Function::Cf04 => cf04(values, &o, &m, true),
            Cec14Function::Cf05 => cf05(values, &o, &m, true),
            Cec14Function::Cf06 => cf06(values, &o, &m, true),
            Cec14Function::Cf07 => cf07(values, &o, &m, &s, true),
            Cec14Function::Cf08 => cf08(values, &o, &m, &s, true),
        }
    }

    pub fn index(&self) -> usize {
        self.clone() as usize
    }
}