use crate::matrix::matrix::Matrix;
use crate::matrix::matrix::__Matrix;

struct NN {
    weights: Vec<Matrix<f64>>,
    biases: Vec<Matrix<f64>>,
    apps: Vec<Matrix<f64>>,
}

impl NN {
    fn new(layers: &[usize]) -> Self {
        let depth = layers.len();

        let mut weights = Vec::with_capacity(depth - 1);
        let mut biases = Vec::with_capacity(depth - 1);
        let mut apps = Vec::with_capacity(depth);

        for (level, size) in layers.iter().enumerate() {
            if level == 0 {
                apps[0] = Matrix::new(1, *size);
            } else {
                weights[level] = Matrix::new(apps[level - 1].len_col(), *size);
                biases[level - 1] = Matrix::new(1, *size);
                apps[level - 1] = Matrix::new(1, *size);
            }
        }

        return NN {
            weights,
            biases,
            apps,
        };
    }

    fn len(&self) -> usize {
        self.weights.len()
    }

    fn process(&mut self) {
        for idx in 0..self.len() {
            let apps = self.apps[idx].clone();
            self.apps[idx+1].dot(&apps,&self.weights[idx]);
            self.apps[idx+1].sum(&self.biases[idx]);
            self.apps[idx+1].sigmoid()
        }
    }

    fn set(&mut self,input:&[f64]) {
        assert!(input.len()==self.apps[0].len_col());
        for (idx, r) in self.apps[0].row_mut(0).iter_mut().enumerate() {
            *r = input[idx];
        }
    }

    fn output(&self) -> &[f64] {
        self.apps[self.len()].row(0)
    }

    fn __diff(&self, expect: &[f64]) -> f64{
        assert!(self.output().len()==expect.len());
        let mut diff = 0.0;
        for (idx,output) in self.output().iter().enumerate() {
            diff += (output-expect[idx]).powi(2);
        }
        diff
    }

    fn diff(&mut self,inputs:&Vec<&[f64]>, expects: &Vec<&[f64]>) -> f64 {
        assert!(inputs.len()==expects.len());
        let mut diff = 0.0;
        for round in 0..inputs.len() {
            self.set(inputs[round]);
            diff += self.__diff(expects[round])
        }
        diff
    }

    fn learn(&mut self, inputs:&Vec<&[f64]>, expects: &Vec<&[f64]>,epsilon:&f64,rate:&f64) {
        
        let cost_original = self.diff(inputs, expects);

        let mut delta = Self::new(self.apps.iter().fold(Vec::new(),|mut layers,apps| {layers.push(apps.len_col()); layers}).as_slice());

        for level in 0..self.len() {
            for row in 0..self.weights[level].len_row() {
                for col in 0..self.weights[level].len_col() {
                    let saved = self.weights[level].at(row, col);
                    *self.weights[level].at_mut(row, col) += *epsilon;
                    let cost_renewed = self.diff(inputs, expects);
                    *delta.weights[level].at_mut(row,col) = (cost_renewed -cost_original) * rate;
                    *self.weights[level].at_mut(row, col) = saved;
                }
            }

        for col in 0..self.biases[level].len_col() {
                    let saved = self.biases[level].at(0, col);
                    *self.biases[level].at_mut(0, col) += *epsilon;
                    let cost_renewed = self.diff(inputs, expects);
                    *delta.biases[level].at_mut(0,col) = (cost_renewed -cost_original) * rate;
                    *self.biases[level].at_mut(0, col) = saved;
                }

        }

        for level in 0..self.len() {
            self.weights[level].sum(&delta.weights[level]);
            self.biases[level].sum(&delta.biases[level]);
        }

    }
}
