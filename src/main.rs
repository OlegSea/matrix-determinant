mod io;

type Matrix = Vec<Vec<f32>>;

fn define_matrix() -> Matrix {
    // Get the size of the matrix from the user
    let matrix_size = io::get_number(Some("Matrix size: "));
    println!("Matrix rows (in a format like \"1 2 3 4 5\"");
    let mut matrix = Vec::new();
    for _ in 0..matrix_size {
        // Get a row from the user
        matrix.push({
            let row: Vec<f32> = io::get_line()
                .split(" ")
                .map(|n| io::parse_f32(n.to_string()))
                .into_iter()
                .collect();
            row
        })
    }
    matrix
}

fn calculate_determinant_ref(matrix: &Matrix) -> f32 {
    // Matrix must be square
    assert!(matrix.len() == matrix[0].len());
    // Matrix must be at least 1x1
    assert!(matrix.len() >= 1);

    match matrix.len() {
        1 => matrix[0][0],
        _ => {
            // Iterate over the first row of the matrix
            matrix[0]
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    // Calculate the cofactor of the current element
                    (-1.0f32).powf((index % 2) as f32)
                        * value
                        * calculate_determinant_ref(
                            // Calculate the minor of the current element
                            &matrix[1..]
                                .to_vec()
                                .into_iter()
                                .map(|row| {
                                    row.iter()
                                        .enumerate()
                                        .filter(|(column, &_)| column != &index)
                                        .map(|(_, &value)| value)
                                        .collect()
                                })
                                .map(|vec: Vec<f32>| vec)
                                .collect::<Matrix>(),
                        )
                })
                .sum()
        }
    }
}

fn main() {
    let matrix = define_matrix();
    let determinant = calculate_determinant_ref(&matrix);
    println!("{}", determinant);
}
