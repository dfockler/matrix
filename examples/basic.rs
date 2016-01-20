extern crate matrix;

use matrix::*;

fn main() {
    let mut matrix = Matrix::new(2, 2);
    matrix.push(13.5f32, 1, 1);
    matrix.push(43f32, 2, 2);

    let mut mata = Matrix::new(2, 2);
    mata.push(23.5f32, 1, 1);
    mata.push(4.3f32, 2, 2);

    println!("{:?}", matrix);
    println!("{:?}", mata);

    let result = matrix.add(&mata);
    println!("{:?}", result);
}