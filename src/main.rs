use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;


#[derive(Debug, Clone)]
struct Number {
    pub value: usize,
    pub on_line: usize,
    pub start_on_col: usize,
    pub end_on_col: usize,
}

impl Number {
    fn new(value: usize, on_line: usize, start_on_col: usize, end_on_col: usize) -> Number {
        Number {
            value,
            on_line,
            start_on_col,
            end_on_col,
        }
    }

    pub fn is_adjacent(&self, symb: &Symbol) -> bool {
        if symb.on_line < self.on_line.saturating_sub(1) || symb.on_line > self.on_line + 1 {return false}
        if symb.on_col < self.start_on_col.saturating_sub(1) || symb.on_col > self.end_on_col + 1 {return false}
        return true
    }
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

const FILE_NAME: &str = "input.txt";
//const FILE_NAME: &str = "testdata.txt";

fn main() {
    let reg_match_number: Regex = Regex::new(r"(\d+)").unwrap();
    let reg_match_symbol: Regex = Regex::new(r"([^0-9.])").unwrap();

    // Read each line of file into a vector
    let lines: Vec<String> = read_lines(FILE_NAME); 

    // Collect all numbers and symbols
    let matched_numbers: Vec<Number> = lines.iter()
        .enumerate()
        .flat_map(|(line_number, line)| {
            reg_match_number.find_iter(line)
                .map(move |m| Number::new(m.as_str().parse().unwrap(), 
                                        line_number, 
                                        m.start(), 
                                        m.end()-1))
                .collect::<Vec<Number>>()
        })
        .collect();

        let matched_symbols: Vec<Symbol> = lines.iter()
            .enumerate()
            .flat_map(|(line_number, line)| {
                reg_match_symbol.find_iter(line)
                    .map(move |m| Symbol::new(m.as_str().chars().next().unwrap(),
                            line_number,
                                m.start()))
                    .collect::<Vec<Symbol>>()
            })
            .collect();


    let sum = matched_numbers
            .iter()
            .filter(|n|matched_symbols.iter().any(|s| n.is_adjacent(s)))
            .fold(0,|acc, n| acc + n.value );

    println!("Part 1 Sum:  {}", sum);


    // Extract all * symbols
    let star_symbols: Vec<Symbol> = matched_symbols
        .iter()
        .filter(|s| s.symbol == '*')
        .cloned()
        .collect::<Vec<Symbol>>();


    let sum = star_symbols
        .iter()
        .filter_map(|star| {
            let mut adjacent_numbers = matched_numbers
                .iter()
                .filter(|n| n.is_adjacent(star))
                .cloned();

            if let (Some(first), Some(second)) = (adjacent_numbers.next(), adjacent_numbers.next()) {
                if adjacent_numbers.next().is_none() {
                    return Some(first.value * second.value);
                }
            }

            None
        })
        .fold(0, |acc, x| acc + x);

    println!("Part 2 Sum:  {}", sum);

}



fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    reader.lines()
        .filter_map(|result| result.ok())
        .collect()
}
