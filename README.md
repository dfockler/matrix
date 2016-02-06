# matrix
A small matrix math library in Rust, with support for generic [Num types](http://rust-num.github.io/num/num/index.html)

## Usage
```
use matrix::*;

//Initializes a new Matrix with Num::zero() in all elements
let mut matrix = Matrix::new(2, 2);

//Push an element into the matrix at the specified index
matrix.push(13.3f32, 1, 1);
matrix.push(43.f32, 2, 2);
```

## Todo
- Error Handling for Index Out-of-Bounds
- Error Handling for Matrix Size Incompatibility
- Better method for pushing values into a Matrix
- Method for generating a Matrix using a Vec
