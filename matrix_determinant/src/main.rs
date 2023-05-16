use crate::solution::determinant;

pub mod solution;
pub mod tests;

fn main() {
    println!(
        "{}",
        determinant(&[vec![2, 5, 3], vec![1, -2, -1], vec![1, 3, 4]])
    );
}
