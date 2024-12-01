use std::fs;

fn main() {
    let file_path = "src/input";

    println!("Hello, world!");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let mut sum: u32 = 0;

    for line in lines {
        let line_val = parse_line_extended(line);
        sum += line_val;
        println!("LINE: {line}. sum is {line_val}");
    }

    println!("sum is: {sum}");
}

const NUMBERS: [&str; 10] = [
    "one", "two", "three", "four", "five", "six", "seven", "eigth", "nine", "ten",
];

fn parse_line_extended(input: &str) -> u32 {
    let mut first_word: (u32, u32) = first_occurence(input, &NUMBERS);

    let mut rev_numbers: [&str; 10] = NUMBERS;

    for (i, number) in NUMBERS.iter().enumerate() {
        rev_numbers[i] = NUMBERS[i].chars().rev().collect().as_str();
        
    }

    let rev_input: String = input.chars().rev().collect();
    let last_word: (u32, u32) = first_occurence(&rev_input, &rev_numbers);

    let mut first_digit: u32 = 0;
    let mut first_digit_index: usize = 0;

    for (i, c) in input.chars().enumerate() {
        match c.to_digit(10) {
            Some(digit) => {
                first_digit = digit;
                first_digit_index = i;
                break;
            }
            None => {
                continue;
            }
        }
    }
    let rev_input = input.chars().rev();

    let mut last_digit: u32 = 0;
    let mut last_digit_index: usize = 0;

    for (i, c) in rev_input.enumerate() {
        match c.to_digit(10) {
            Some(digit) => {
                last_digit = digit;
                last_digit_index = i;
                break;
            }
            None => {
                continue;
            }
        }
    }

    let final_first = if first_digit_index > first_word.0.try_into().unwrap() {
        first_word.1
    } else {
        first_digit
    };

    let final_last = if last_digit_index < last_word.0.try_into().unwrap() {
        first_word.1
    } else {
        last_digit
    };

    return (final_first * 10) + final_last;
}

fn first_occurence(input: &str, numbers: &[&str]) -> (u32, u32) {
    let mut index: u32 = 0;
    let mut value: u32 = 0;
    for (i, number) in numbers.iter().enumerate() {
        match input.find(number) {
            Some(digit) => {
                let temp: usize = index.try_into().unwrap();
                if temp < i {
                    index = u32::try_from(digit).unwrap();
                    value = (i + 1).try_into().unwrap();
                } else {
                    continue;
                }
            }
            None => {
                continue;
            }
        }
    }
    return (index, value);
}

fn parse_line(input: &str) -> u32 {
    let mut first_digit: u32 = 0;
    let mut last_digit: u32 = 0;
    for c in input.chars() {
        match c.to_digit(10) {
            Some(digit) => {
                first_digit = digit;
                break;
            }
            None => {
                continue;
            }
        }
    }
    let rev_input = input.chars().rev();

    for c in rev_input {
        match c.to_digit(10) {
            Some(digit) => {
                last_digit = digit;
                break;
            }
            None => {
                continue;
            }
        }
    }

    return (first_digit * 10) + last_digit;
}
