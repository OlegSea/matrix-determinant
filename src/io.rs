use std::io;

pub fn get_line() -> String {
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read the line!");
    input_line
}

pub fn parse_i32(input_line: String) -> i32 {
    input_line.trim().parse().expect("Not a number!")
}

pub fn parse_f32(input_line: String) -> f32 {
    input_line.trim().parse().expect("Not a number!")
}

pub fn get_number(msg: Option<&str>) -> i32 {
    match msg {
        Some(msg) => println!("{}", msg),
        None => (),
    };

    let input_line = get_line();
    parse_i32(input_line)
}
