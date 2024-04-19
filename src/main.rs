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
    pub adjacent_symbols: Box<Vec<Symbol>>,
}

impl Number {
    fn new(value: u32, on_line: u32, start_on_col: u32, end_on_col: u32) -> Number {
        Number {
            value,
            is_valid: false,
            on_line,
            start_on_col,
            end_on_col,
            adjacent_symbols: Box::new(Vec::new()),
        }
    }
}

#[derive(Debug, Clone)]
struct Symbol {
    pub symbol: char,
    pub on_line: usize,
    pub on_col: usize,
    pub adjacent_number: Box<Vec<Number>>,
}

impl Symbol {
    fn new(symbol: char, on_line: usize, on_col: usize) -> Symbol {
        Symbol {
            symbol,
            on_line,
            on_col,
            adjacent_number: Box::new(Vec::new()),
        }
    }
}

//const FILE_NAME: &str = "input.txt";
const FILE_NAME: &str = "testdata.txt";

fn main() {

    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    let reg_match_number: Regex = Regex::new(r"(\d+)").unwrap();

    // Read each line of file into a vector
    let lines: Vec<String> = read_lines(FILE_NAME); 

    let mut sum  = 0;

    for (line_number, line) in lines.iter().enumerate() {
        let found_numbers: regex::Matches<'_, '_> = 
            reg_match_number.find_iter(&line);
        for number in found_numbers {
            let value: u32 = number.as_str().parse().unwrap();
            let start_col: u32 = number.start() as u32;
            let end_col: u32 = (number.end() as u32)-1;

            let new_number: Number = Number::new(
                value, 
                line_number as u32, 
                start_col, 
                end_col);

            numbers.push(new_number.clone());
        }

        for (c_ind, c) in line.chars().enumerate() {
            if !c.is_numeric() && c != '.' {
                let new_symbol: Symbol = Symbol::new(
                    c, 
                    line_number as usize, 
                    c_ind,
                );
                symbols.push(new_symbol.clone());               
            }
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
