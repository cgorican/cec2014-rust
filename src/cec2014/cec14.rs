use crate::cec2014::cec14_helper::Cec14Helper;
use crate::cec2014::composition::{cf01, cf02, cf03, cf04, cf05, cf06, cf07, cf08};
use crate::cec2014::hybrid::{hf01, hf02, hf03, hf04, hf05, hf06};
use crate::cec2014::multimodal::{shifted_rastrigin, shifted_rotated_ackley, shifted_rotated_escaffer6, shifted_rotated_expanded_griewank_rosenbrock, shifted_rotated_griewank, shifted_rotated_happy_cat, shifted_rotated_hgbat, shifted_rotated_katsuura, shifted_rotated_rastrigin, shifted_rotated_rosenbrock, shifted_rotated_schwefel, shifted_rotated_weierstrass, shifted_schwefel};
use crate::cec2014::unimodal::{rotated_bent_cigar, rotated_discus, rotated_high_conditioned_elliptic};

pub struct Cec14 {
    fn_index: usize,
    dim: usize,
    o: Vec<f64>,
    m: Vec<f64>,
    s: Vec<usize>
}

#[allow(unused)]
impl Cec14 {
    pub fn new(fn_index: usize, dim: usize) -> Self {
        let mut cec = Cec14 {
            fn_index,
            dim,
            o: Vec::new(),
            m: Vec::new(),
            s: Vec::new(),
        };

        cec.load_input_data(&Cec14Helper::default());

        cec
    }

    fn load_input_data(&mut self, helper: &Cec14Helper) {
        if !(self.dim==2||self.dim==10||self.dim==20||self.dim==30||self.dim==50||self.dim==100)
        {
            panic!("Test functions are only defined for D=2,10,20,30,50,100.");
        }
        if self.dim==2&&((self.fn_index>=17&&self.fn_index<=22)||(self.fn_index>=29&&self.fn_index<=30))
        {
            panic!("Error: hf01,hf02,hf03,hf04,hf05,hf06,cf07,cf08 are NOT defined for D=2.");
        }

        self.o = helper.load_shift_vector(self.fn_index, self.dim);
        self.m = helper.load_rotation_matrix(self.fn_index, self.dim);
        self.s = helper.load_shuffle_vector(self.fn_index, self.dim);
    }

    #[allow(dead_code)]
    pub fn set_dim(&mut self, dim: usize) {
        self.dim = dim;
        self.load_input_data(&Cec14Helper::default());
    }

    #[allow(dead_code)]
    pub fn set_fn_index(&mut self, fn_index: usize) {
        self.fn_index = fn_index;
        self.load_input_data(&Cec14Helper::default());
    }

    pub fn eval(&self, values: &[f64]) -> f64 {
        if values.len() != self.dim {
            panic!("Input dimension does not match the initialized dimension!");
        }

        let score = match self.fn_index {
            1 => rotated_high_conditioned_elliptic(values, &self.o, &self.m, true, true),
            2 => rotated_bent_cigar(values, &self.o, &self.m, true, true),
            3 => rotated_discus(values, &self.o, &self.m, true, true),
            4 => shifted_rotated_rosenbrock(values, &self.o, &self.m, true, true),
            5 => shifted_rotated_ackley(values, &self.o, &self.m, true, true),
            6 => shifted_rotated_weierstrass(values, &self.o, &self.m, true, true),
            7 => shifted_rotated_griewank(values, &self.o, &self.m, true, true),
            8 => shifted_rastrigin(values, &self.o, true),
            9 => shifted_rotated_rastrigin(values, &self.o, &self.m, true, true),
            10 => shifted_schwefel(values, &self.o, true),
            11 => shifted_rotated_schwefel(values, &self.o, &self.m, true, true),
            12 => shifted_rotated_katsuura(values, &self.o, &self.m, true, true),
            13 => shifted_rotated_happy_cat(values, &self.o, &self.m, true, true),
            14 => shifted_rotated_hgbat(values, &self.o, &self.m, true, true),
            15 => shifted_rotated_expanded_griewank_rosenbrock(values, &self.o, &self.m, true, true),
            16 => shifted_rotated_escaffer6(values, &self.o, &self.m, true, true),
            17 => hf01(values, &self.o, &self.m, &self.s, true, true, true),
            18 => hf02(values, &self.o, &self.m, &self.s, true, true, true),
            19 => hf03(values, &self.o, &self.m, &self.s, true, true, true),
            20 => hf04(values, &self.o, &self.m, &self.s, true, true, true),
            21 => hf05(values, &self.o, &self.m, &self.s, true, true, true),
            22 => hf06(values, &self.o, &self.m, &self.s, true, true, true),
            23 => cf01(values, &self.o, &self.m, true),
            24 => cf02(values, &self.o, &self.m, true),
            25 => cf03(values, &self.o, &self.m, true),
            26 => cf04(values, &self.o, &self.m, true),
            27 => cf05(values, &self.o, &self.m, true),
            28 => cf06(values, &self.o, &self.m, true),
            29 => cf07(values, &self.o, &self.m, &self.s, true),
            30 => cf08(values, &self.o, &self.m, &self.s, true),
            _ => panic!("There are only 30 test functions in this test suite!"),
        };

        score * self.fn_index as f64 * 100.0
    }
}
