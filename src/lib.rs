// add
// mult
// scalar
// transpose
// dot product

extern crate num;

use num::Num;
use num::Zero;

#[derive(Debug)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

// trait MatrixOps<T> {
//     fn add(&self, m: &Matrix<T>) -> Matrix<T>;
//     // fn mult(&self, m: &Matrix<T>) -> Matrix<T>;
//     // fn dot(&self, m: &Matrix<T>) -> Matrix<T>;
//     // fn scalar(&self, scalar: T) -> Matrix<T>;
//     // fn transpose(&self) -> Matrix<T>;
//     fn index(&self, i: usize, j: usize) -> T;
//     // fn push(&mut self, value: T, i: usize, j: usize); 
// }

impl<T: Num + Clone> Matrix<T> {
    pub fn new(rows: usize, cols: usize) -> Matrix<T> {
        Matrix { 
            rows: rows, 
            cols: cols, 
            data: vec![T::zero(); cols * rows],
        }
    }

    fn new_with_data(rows: usize, cols: usize, data: Vec<T>) -> Matrix<T>{
        Matrix {
            rows: rows,
            cols: cols,
            data: data,
        }
    }

    pub fn add(&self, m: &Matrix<T>) -> Matrix<T> {
        let mut vec: Vec<T> = Vec::new(); 
        for (a, b) in self.data.iter().zip(m.data.iter()) {
            vec.push(a.clone() + b.clone());
        }
        Matrix::new_with_data(self.rows, self.cols, vec)
    }

    pub fn mult(&self, m: &Matrix<T>) -> Matrix<T> {
        let mut vec: Vec<T> = Vec::new(); 
        for (a, b) in self.data.iter().zip(m.data.iter()) {
            vec.push(a.clone() * b.clone());
        }
        Matrix::new_with_data(self.rows, self.cols, vec)
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn index(&self, row: usize, col: usize) -> T {
        self.data[(row * col) - 1].clone()
    }

    pub fn push(&mut self, value: T, row: usize, col: usize) {
        self.data[(row * col) - 1] = value;
    }
}


#[test]
fn it_works() {
}
