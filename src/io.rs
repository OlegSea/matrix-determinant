use std::io;

pub fn get_line() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read the line!");
    user_input
}

pub fn parse_i32(input_line: String) -> i32 {
    // Trim the whitespace from the string
    let trimmed = input_line.trim();
    // Convert the trimmed string to a number
    let parsed = trimmed.parse();
    // If the string was not a number, panic
    let number = parsed.expect("Not a number!");
    // Return the parsed number
    number
}

// parse_f32 converts a string input to a float32
// and returns the value
pub fn parse_f32(input_line: String) -> f32 {
    input_line.trim().parse().expect("Not a number!")
}

pub fn get_number(msg: Option<&str>) -> i32 {
    // If there is a message, print it
    match msg {
        Some(msg) => println!("{}", msg),
        None => (),
    };

    // Get a line of input from the user
    let input_line = get_line();

    // Parse the input line as an integer
    parse_i32(input_line)
}
