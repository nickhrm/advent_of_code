use std::fs;

fn main() {
    let file_path = "src/input";

    println!("Hello, world!");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = contents.split("\n");

    for line in lines {
        println!("{}", line);
    }
}

fn parseLine(input: &str) -> i32 {
    let mut found_first_digit = false;
    let mut first_digit = 0;
    let mut last_digit = 0;
    for c in input.chars() {
        match c.to_digit(10) {
            Some(digit) => {
                if (!found_first_digit) {
                    first_digit = digit;
                } else {
                    last_digit = digit;
                }
            }
        }
    }
    return 
}
