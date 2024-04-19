use core::num;
use std::collections::linked_list;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

#[derive(Debug, Clone)]
struct Number {
    pub value: u32,
    pub is_valid: bool,
    pub on_line: u32,
    pub start_on_col: u32,
    pub end_on_col: u32,
    pub adjacent_symbols: String,
}

impl Number {
    fn new(value: u32, on_line: u32, start_on_col: u32, end_on_col: u32) -> Number {
        Number {
            value,
            is_valid: false,
            on_line,
            start_on_col,
            end_on_col,
            adjacent_symbols: String::new(),
        }
    }
}

//const FILE_NAME: &str = "input.txt";
const FILE_NAME: &str = "testdata.txt";

fn main() {

    let mut matched_numbers: Vec<Number> = Vec::new();
    let reg_match_number: Regex = Regex::new(r"(\d+)").unwrap();
    let reg_match_symbol: Regex = Regex::new(r"([^0-9.])").unwrap();

    // Read each line of file into a vector
    let lines: Vec<String> = read_lines(FILE_NAME); 

    let mut sum  = 0;

    for (line_number, line) in lines.iter().enumerate() {
        let found_numbers: regex::Matches<'_, '_> = reg_match_number.find_iter(&line);
        for number in found_numbers {
            let value: u32 = number.as_str().parse().unwrap();
            let start_col: u32 = number.start() as u32;
            let end_col: u32 = (number.end() as u32)-1;

            let mut new_number: Number = Number::new(value, line_number as u32, start_col, end_col);

            // Find adjacent fields
            let mut adjacent_string = String::new();
            let line_iter = line_number.saturating_sub(1)..(line_number+1).min(lines.len()-1);
            let char_iter = (start_col.saturating_sub(1) as usize)..((end_col as usize)+1).min(&line.len()-1);
            for l in line_iter {
                adjacent_string.push_str(lines[l].to_string()[char_iter.clone()].to_string().as_str());
            }
            
            let found_symbols: regex::Matches<'_, '_> = reg_match_symbol.find_iter(&line);
            for symbol in found_symbols {
                let symbol_char: char = symbol.as_str().chars().next().unwrap();
                new_number.adjacent_symbols.push(symbol_char);
            }

            if new_number.adjacent_symbols.is_empty() { 
                new_number.is_valid = false
            } else {
                new_number.is_valid = true;
                sum += new_number.value;
            }        
            matched_numbers.push(new_number.clone());
        }
    }
    
    println!("{}", sum);
}



fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    reader.lines()
        .filter_map(|result| result.ok())
        .collect()
}
