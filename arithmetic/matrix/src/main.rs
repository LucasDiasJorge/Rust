extern crate nalgebra as na;

use na::{Matrix2, Matrix3};

fn main() {
    // Create a 2x2 matrix
    let matrix_a = Matrix2::new(1.0, 2.0,
                                3.0, 4.0);

    // Create another 2x2 matrix
    let matrix_b = Matrix2::new(5.0, 6.0,
                                7.0, 8.0);

    // Matrix addition
    let matrix_sum = matrix_a + matrix_b;
    println!("Matrix sum:\n{}", matrix_sum);

    // Matrix multiplication
    let matrix_product = matrix_a * matrix_b;
    println!("Matrix product:\n{}", matrix_product);

    // Matrix transposition
    let matrix_transposed = matrix_a.transpose();
    println!("Transposed matrix:\n{}", matrix_transposed);

    // Create a 3x3 identity matrix
    let identity_matrix = Matrix3::identity();
    println!("Identity matrix:\n{}", identity_matrix);
}
