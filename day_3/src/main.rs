use std::{char, fs};

mod models;
use models::Number;

fn main() {
    let file_path = "src/input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = contents.lines();

    let data: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let mut sum = 0;

    for (index, line) in data.iter().enumerate() {
        let numbers = find_numbers_in_line(line, index);
        for number in numbers {
            if check_adjacent_symbols(&data, &number) {
                sum += number.value.parse::<i32>().unwrap();
            }
        }
    }
    println!("Sum of numbers with adjacent symbols: {}", sum);
}

fn find_numbers_in_line(data: &Vec<char>, line: usize) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();

    let mut reading_number = false;

    for (i, c) in data.iter().enumerate() {
        if c.is_digit(10) {
            if !reading_number {
                reading_number = true;
                numbers.push(Number {
                    line: line as usize,
                    start_index: i as u32,
                    end_index: i as u32,
                    value: c.to_string(),
                });
            } else {
                numbers.last_mut().unwrap().end_index = i as u32;
                numbers.last_mut().unwrap().value.push(*c);
            }
        } else {
            if reading_number {
                reading_number = false;
            }
        }
    }

    return numbers;
}

//returns true, if there is a symbol other than . or number adjacent (also in line above or below) to the number.
fn check_adjacent_symbols(data: &Vec<Vec<char>>, number: &Number) -> bool {
    let mut adjacent_symbols: Vec<char> = Vec::new();

    for (index, _c) in number.value.chars().enumerate() {

        println!("checking neigbours of {} in line {}", _c, number.line);

        for y in -1..2 {
            for x in -1..2 {
                let y_index = number.line as i32 + y;
                let x_index = number.start_index as i32 + index as i32 + x;

                // Check bounds
                if y_index < 0 || x_index < 0 || y_index as usize >= data.len() || x_index as usize >= data[y_index as usize].len() {
                    continue;
                }

                // Skip the symbol itself
                if y == 0 && x == 0 {
                    continue;
                }

                println!(
                    "checking symbol {} at {},{}",
                    data[y_index as usize][x_index as usize], x_index, y_index
                );

                adjacent_symbols.push(data[y_index as usize][x_index as usize]);
            }
        }
    }

    let mut found_symbol = false;

    for &symbol in adjacent_symbols.iter() {
        println!(
            "Number {} in line {} has adjacent symbol {}",
            number.value, number.line, symbol
        );
        if symbol != '.' && !symbol.is_digit(10) {
            found_symbol = true;
        }
    }

    return found_symbol;
}