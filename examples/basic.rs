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

    let sum = matrix.add(&mata);
    let product = matrix.mult(&mata);
    println!("{:?}", sum);
    println!("{:?}", product);

    let mut mat = Matrix::new(2, 4);
    mat.push(1, 1, 1);
    mat.push(2, 1, 2);
    mat.push(3, 1, 3);
    mat.push(4, 1, 4);
    mat.push(5, 2, 1);
    mat.push(6, 2, 2);
    mat.push(7, 2, 3);
    mat.push(8, 2, 4);
    let transpose_a = mat.transpose();
    println!("{:?}", mat.index(2, 1));
    println!("{:?}", transpose_a);
    println!("{:?}", transpose_a.index(2, 1));

    let mut other = Matrix::new(4, 2);
    other.push(1, 1, 1);
    other.push(2, 2, 1);
    other.push(3, 3, 1);
    other.push(4, 4, 1);
    other.push(5, 1, 2);
    other.push(6, 2, 2);
    other.push(7, 3, 2);
    other.push(8, 4, 2);
    println!("mat > {:?}", mat);
    println!("other > {:?}", other);
    let result = mat.dot(&other);
    println!("{:?}", result);
    println!("{:?}", result.index(2, 2));
}