pub fn determinant(matrix: &[Vec<i64>]) -> i64 {
    if matrix.len() == 1 {
        return matrix[0][0];
    }
    if matrix.len() == 2 {
        return matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
    }

    let mut det = 0;
    for n_matrix in 0..matrix.len() {
        let n_minor = matrix
            .iter()
            .skip(1)
            .map(|row| {
                row.iter()
                    .enumerate()
                    .filter(|&(i, _)| i != n_matrix)
                    .map(|(_, x)| *x)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        det += matrix[0][n_matrix] * determinant(&n_minor) * if n_matrix % 2 == 0 { 1 } else { -1 };
    }

    det
}
