use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

#[derive(Debug, Clone)]
struct Number {
    pub value: usize,
    pub is_valid: bool,
    pub on_line: usize,
    pub start_on_col: usize,
    pub end_on_col: usize,
}

impl Number {
    fn new(value: usize, on_line: usize, start_on_col: usize, end_on_col: usize) -> Number {
        Number {
            value,
            is_valid: false,
            on_line,
            start_on_col,
            end_on_col,
        }
    }

//    pub fn is_adjacent(&mut self, symb: &Symbol) {
//        todo!();
//    }
}

#[derive(Debug, Clone)]
struct Symbol {
    pub symbol: char,
    pub on_line: usize,
    pub on_col: usize,
}

impl Symbol {
    fn new(symbol: char, on_line: usize, on_col: usize) -> Symbol {
        Symbol {
            symbol,
            on_line,
            on_col,
        }
    }
}

//const FILE_NAME: &str = "input.txt";
const FILE_NAME: &str = "testdata.txt";

fn main() {

    let mut matched_numbers: Vec<Number> = Vec::new();
    let mut matched_symbols: Vec<Symbol> = Vec::new();
    let reg_match_number: Regex = Regex::new(r"(\d+)").unwrap();
    let reg_match_symbol: Regex = Regex::new(r"([^0-9.{1}])").unwrap();

    // Read each line of file into a vector
    let lines: Vec<String> = read_lines(FILE_NAME); 

    // Collect all numbers and symbols
    for (line_number, line) in lines.iter().enumerate() {     
        matched_numbers.extend(reg_match_number.find_iter(&line)
            .map(|m| Number::new(m.as_str().parse().unwrap(), 
                                             line_number, 
                                             m.start(), 
                                             m.end()))
            .collect::<Vec<Number>>().clone());
        
        matched_symbols.extend(reg_match_symbol
            .find_iter(&line)
            .map(|m| Symbol::new(m.as_str().chars().next().unwrap(),
                                                line_number,
                                                m.start()))
            .collect::<Vec<Symbol>>().clone());      
    }

    println!("{:?}", matched_numbers);
    println!("{:?}", matched_symbols);
    


    
}



fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    reader.lines()
        .filter_map(|result| result.ok())
        .collect()
}
