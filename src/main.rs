mod io;

type Matrix = Vec<Vec<f32>>;

fn define_matrix() -> Matrix {
    let matrix_size = io::get_number(Some("Matrix size: "));
    println!("Matrix rows (in a format like \"1 2 3 4 5\"");
    let mut matrix = Vec::new();
    for _ in 0..matrix_size {
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
    match matrix.len() {
        1 => matrix[0][0],
        _ => matrix[0]
            .iter()
            .enumerate()
            .map(|(index, value)| {
                (-1.0f32).powf((index % 2) as f32)
                    * value
                    * calculate_determinant_ref(
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
            .sum(),
    }
}

fn main() {
    let matrix = define_matrix();
    let determinant = calculate_determinant_ref(&matrix);
    println!("{}", determinant);
}
