// add
// mult
// scalar
// transpose
// dot product

extern crate num;

use num::Num;
use num::Zero;

/// The struct used by the Matrix type is simple. It uses the number of rows,
/// the number of columns and a data field that contains the data.
#[derive(Debug)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

/// The Matrix type allows for simple operations on an underlying Vec, as if
/// the data was arranged in a 2D array. None of the functions in the library
/// modify the Matrix, with the exception of `push`
///
/// The Matrix type is generic over Numeric types that can be cloned
impl<T: Num + Clone> Matrix<T> {
    /// Returns a new Matrix with the given number of rows and columns set.
    /// All elements are set to zero
    pub fn new(rows: usize, cols: usize) -> Matrix<T> {
        Matrix { 
            rows: rows,
            cols: cols,
            data: vec![T::zero(); cols * rows],
        }
    }

    /// Returns a new Matrix with the number of rows and columns set.
    /// Uses the provided data to set the data in the Matrix
    ///
    /// TODO: Make this public and check data is the correct size
    /// given the rows/cols
    fn new_with_data(rows: usize, cols: usize, data: Vec<T>) -> Matrix<T>{
        Matrix {
            rows: rows,
            cols: cols,
            data: data,
        }
    }

    /// Performs an element-wise addition onto the Matrix
    ///
    /// TODO: Add error handling to ensure size compatibility
    pub fn add(&self, m: &Matrix<T>) -> Matrix<T> {
        let mut vec: Vec<T> = Vec::new(); 
        for (a, b) in self.data.iter().zip(m.data.iter()) {
            vec.push(a.clone() + b.clone());
        }
        Matrix::new_with_data(self.rows, self.cols, vec)
    }

    /// Performs an element-wise multiplication onto the Matrix
    ///
    /// TODO: Add error handling to ensure size compatibility
    pub fn mult(&self, m: &Matrix<T>) -> Matrix<T> {
        let mut vec: Vec<T> = Vec::new(); 
        for (a, b) in self.data.iter().zip(m.data.iter()) {
            vec.push(a.clone() * b.clone());
        }
        Matrix::new_with_data(self.rows, self.cols, vec)
    }

    /// Transposes the Matrix and returns the resulting Matrix
    pub fn transpose(&self) -> Matrix<T> {
        let mut vec: Vec<T> = Vec::new();
        for i in 0..self.cols {
            for j in 0..self.rows {
                vec.push(self.data[(j * self.cols + i)].clone());
            }
        }
        Matrix::new_with_data(self.cols, self.rows, vec)
    }

    /// Performs the dot product and returns the resulting Matrix
    /// TODO: Add error handling to ensure size compatibility
    pub fn dot(&self, m: &Matrix<T>) -> Matrix<T> {
        let mut vec: Vec<T> = Vec::new();
        for i in 0..self.rows {
            for j in 0..m.cols {
                let mut sum = T::zero();
                for k in 0..self.cols {
                    sum = sum + self.data[i * self.cols + k].clone() * m.data[k * m.cols + j].clone();
                }
                vec.push(sum);
            }
        }
        Matrix::new_with_data(self.rows, m.cols, vec)
    }

    /// Returns the dimensions of the Matrix in a tuple
    /// in (rows, columns) format
    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    /// Returns the specific value at an index given by
    /// a row and column
    pub fn index(&self, row: usize, col: usize) -> T {
        self.data[(row-1) * self.cols + (col-1)].clone()
    }

    /// Pushes a value into the specified location in the Matrix
    pub fn push(&mut self, value: T, row: usize, col: usize) {
        self.data[ (row-1) * self.cols + (col-1) ] = value;
    }
}


#[test]
fn it_works() {
}
