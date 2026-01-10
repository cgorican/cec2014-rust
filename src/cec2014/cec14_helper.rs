use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Cec14Helper {
    path: String,
}

impl Default for Cec14Helper {
    fn default() -> Cec14Helper {
        Cec14Helper::new(String::from("./input_data/"))
    }
}

impl Cec14Helper {
    pub fn new(mut path: String) -> Self {
        path = path.replace("\\", "/");

        if !path.ends_with('/') {
            path.push('/');
        }

        Self {
            path,
        }
    }

    pub fn rotate(values: &[f64], m: &[f64], flag: bool) -> Vec<f64> {
        if !flag {
            return values.to_vec();
        }

        if values.len().pow(2) != m.len() {
            panic!("Invalid rotational matrix");
        }

        let mut result: Vec<f64> = Vec::with_capacity(values.len());

        for i in 0..values.len() {
            let mut sum = 0.0;
            for j in 0..values.len() {
                // 1xN * NxN => 1xN
                sum += values[j] * m[i * values.len() + j];
            }
            result.push(sum);
        }

        result
    }

    #[allow(dead_code)]
    pub fn shift_add(values: &mut [f64], o: &[f64], flag: bool) {
        if !flag {
            return;
        }
        let range = std::cmp::min(values.len(), o.len());

        for i in 0..range {
            values[i] += o[i];
        }
    }

    pub fn shift_sub(values: &mut [f64], o: &[f64], flag: bool) {
        if !flag {
            return;
        }
        let range = std::cmp::min(values.len(), o.len());

        for i in 0..range {
            values[i] -= o[i];
        }
    }

    pub fn scale(values: &mut [f64], factor: f64) {
        for i in 0..values.len() {
            values[i] *= factor;
        }
    }

    fn open_file(&self, filename: &String) -> BufReader<File> {
        let filepath = Path::new(&self.path).join(filename);

        if !filepath.exists() {
            panic!("File does not exist: {}", filepath.to_str().unwrap());
        }

        let file = File::open(&filepath)
            .expect(&format!("Failed to open the file: {}", filepath.to_str().unwrap()));

        BufReader::new(file)
    }

    pub fn load_rotation_matrix(&self, fn_index: usize, dim: usize) -> Vec<f64> {
        let filename = format!("M_{}_D{}.txt", fn_index, dim);
        let reader = self.open_file(&filename);

        let cf_num = if dim > 2 {
            10
        } else {
            8
        };

        let capacity = if fn_index < 23 {
            dim * dim
        } else {
            (dim * dim) * cf_num
        };

        let mut matrix: Vec<f64> = Vec::with_capacity(capacity);

        // handle file
        for line in reader.lines() {
            let line = line.expect("Failed to read a line!");
            for token in line.split_whitespace().filter(|t| !t.is_empty()) {
                let x = token.parse::<f64>()
                    .expect("Failed to parse f64");
                matrix.push(x);
            }
        }

        if matrix.len() != capacity {
            if fn_index < 23 {
                panic!("Rotation matrix dimension mismatch! Expected {}x{}, got {}x{}",
                       dim,
                       dim,
                       matrix.len(),
                       matrix.len());
            } else {
                let dim_half = dim / 2;
                let dim1 = ((matrix.len() / cf_num) as f64).sqrt();
                panic!("Rotation matrix dimension mismatch! Expected {}x{}x{}, got {}x{}x{}",
                       dim_half,
                       dim_half,
                       cf_num,
                       dim1,
                       dim1,
                       cf_num);
            }
        }

        matrix
    }

    pub fn load_shift_vector(&self, fn_index: usize, dim: usize) -> Vec<f64> {
        let filename = format!("shift_data_{}.txt", fn_index);
        let reader = self.open_file(&filename);

        let cf_num = 10usize;
        let capacity = if fn_index < 23 {
            dim
        } else {
            dim * cf_num
        };

        let mut shift_vec: Vec<f64> = Vec::with_capacity(capacity);
        for line in reader.lines() {
            let line = line.expect("Failed to read a line!");
            for x_str in line.split_whitespace().filter(|t| !t.is_empty()) {
                let x = x_str.parse::<f64>().expect("Failed to parse f64");
                shift_vec.push(x);
                if fn_index > 22 && shift_vec.len() % dim == 0 || shift_vec.len() == capacity {
                    break;
                }
            }
            if shift_vec.len() == capacity {
                break;
            }
        }

        if shift_vec.len() != capacity {
            panic!("Shift vector dimension mismatch! Expected {} values, got {}",
                   capacity,
                   shift_vec.len());
        }

        shift_vec
    }

    pub fn load_shuffle_vector(&self, fn_index: usize, dim: usize) -> Vec<usize> {
        let filename = format!("shuffle_data_{}_D{}.txt", fn_index, dim);
        let reader = self.open_file(&filename);

        let cf_num = 10usize;
        let capacity = if fn_index < 23 {
            dim
        } else {
            dim * cf_num
        };

        let mut shuffle_vec: Vec<usize> = Vec::with_capacity(capacity);
        for line in reader.lines() {
            let line = line.expect("Failed to read a line!");
            for x_str in line.split_whitespace().filter(|t| !t.is_empty()) {
                let x = x_str.parse::<usize>().expect("Failed to parse usize");
                shuffle_vec.push(x-1); // 0-based index
                if shuffle_vec.len() == capacity {
                    break;
                }
            }
            if shuffle_vec.len() == capacity {
                break;
            }
        }

        if shuffle_vec.len() != capacity {
            panic!("Shuffle vector dimension mismatch! Expected {} values, got {}",
                   capacity,
                   shuffle_vec.len());
        }

        shuffle_vec
    }

    pub fn calculate_slices(p: &[f64], len: usize) -> Vec<usize> {
        let mut p_count: Vec<usize> = Vec::with_capacity(p.len() - 1);
        if p.is_empty() {
            return p_count;
        }
        let len_f64 = len as f64;

        let mut sum_index: usize = 0;
        for p_i in p.iter().take(p.len().saturating_sub(1)) {
            sum_index += f64::ceil(p_i * len_f64) as usize;
            p_count.push(sum_index);
        }

        p_count
    }

    pub fn shuffle(values: &[f64], s: &[usize], flag: bool) -> Vec<f64> {
        if !flag {
            return values.to_vec();
        }
        // s should be 0 index based
        s.iter()
            .map(|&s_i| values[s_i])
            .collect::<Vec<f64>>()
    }
}