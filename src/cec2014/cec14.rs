use crate::cec2014::cec14_function::Cec14Function;
use crate::cec2014::cec14_helper::Cec14Helper;

const AVAILABLE_DIM: [usize; 6] = [2,10,20,30,50,100];

pub struct Cec14 {
    problem: Cec14Function,
    dim: usize,
    o: Vec<f64>,
    m: Vec<f64>,
    s: Vec<usize>
}

impl Default for Cec14 {
    fn default() -> Self {
        Self {
            problem: Cec14Function::F1,
            dim: 10,
            o: Vec::new(),
            m: Vec::new(),
            s: Vec::new(),
        }
    }
}

#[allow(unused)]
impl Cec14 {
    pub fn new(problem: Cec14Function, dim: usize) -> Self {
        let mut cec = Cec14 {
            problem,
            dim,
            ..Self::default()
        };

        cec.load_input_data(&Cec14Helper::default());

        cec
    }

    fn load_input_data(&mut self, helper: &Cec14Helper) {
        if !AVAILABLE_DIM.contains(&self.dim)
        {
            panic!("Test functions are only defined for D=2,10,20,30,50,100.");
        }
        let problem_index = self.problem.index();
        if self.dim==2&&((problem_index >= 17 && problem_index <= 22) || (problem_index >= 29 && problem_index <= 30))
        {
            panic!("Error: hf01,hf02,hf03,hf04,hf05,hf06,cf07,cf08 are NOT defined for D=2.");
        }

        self.o = helper.load_shift_vector(problem_index, self.dim);
        self.m = helper.load_rotation_matrix(problem_index, self.dim);
        self.s = helper.load_shuffle_vector(problem_index, self.dim);
    }

    #[allow(dead_code)]
    pub fn set_dim(&mut self, dim: usize) {
        if !AVAILABLE_DIM.contains(&dim)
        {
            panic!("Test functions are only defined for D=2,10,20,30,50,100.");
        }
        self.dim = dim;
        self.load_input_data(&Cec14Helper::default());
    }

    #[allow(dead_code)]
    pub fn set_problem(&mut self, problem: Cec14Function) {
        self.problem = problem;
        self.load_input_data(&Cec14Helper::default());
    }

    pub fn eval(&self, values: &[f64]) -> f64 {
        if values.len() != self.dim {
            panic!("Input dimension does not match the initialized dimension!");
        }

        let score = self.problem.eval(values, &self.o, &self.m, &self.s);

        score * (self.problem.index() as f64) * 100.0
    }
}
