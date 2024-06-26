use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
pub(crate) struct Number {
    pub value: usize,
    pub on_line: usize,
    pub start_on_col: usize,
    pub end_on_col: usize,
}

impl Number {
    pub(crate) fn new(
        value: usize,
        on_line: usize,
        start_on_col: usize,
        end_on_col: usize,
    ) -> Number {
        Number {
            value,
            on_line,
            start_on_col,
            end_on_col,
        }
    }

    pub fn is_adjacent(&self, symb: &Symbol) -> bool {
        if symb.on_line < self.on_line.saturating_sub(1) || symb.on_line > self.on_line + 1 {
            return false;
        }
        if symb.on_col < self.start_on_col.saturating_sub(1) || symb.on_col > self.end_on_col + 1 {
            return false;
        }
        true
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Symbol {
    pub symbol: char,
    pub on_line: usize,
    pub on_col: usize,
}

impl Symbol {
    pub(crate) fn new(symbol: char, on_line: usize, on_col: usize) -> Symbol {
        Symbol {
            symbol,
            on_line,
            on_col,
        }
    }
}

pub(crate) fn collect_numbers(lines: &[String]) -> Vec<Number> {
    let reg_match_number: Regex = Regex::new(r"(\d+)").unwrap();

    lines
        .iter()
        .enumerate()
        .flat_map(|(line_number, line)| {
            reg_match_number
                .find_iter(line)
                .map(move |m| {
                    Number::new(
                        m.as_str().parse().unwrap(),
                        line_number,
                        m.start(),
                        m.end() - 1,
                    )
                })
                .collect::<Vec<Number>>()
        })
        .collect()
}

pub(crate) fn collect_symbols(lines: &[String]) -> Vec<Symbol> {
    let reg_match_symbol: Regex = Regex::new(r"([^0-9.])").unwrap();

    lines
        .iter()
        .enumerate()
        .flat_map(|(line_number, line)| {
            reg_match_symbol
                .find_iter(line)
                .map(move |m| {
                    Symbol::new(m.as_str().chars().next().unwrap(), line_number, m.start())
                })
                .collect::<Vec<Symbol>>()
        })
        .collect()
}

pub(crate) fn sum_adjacent(matched_numbers: &[Number], matched_symbols: &[Symbol]) -> usize {
    matched_numbers
        .iter()
        .filter_map(|n| {
            if matched_symbols.iter().any(|s| n.is_adjacent(s)) {
                Some(n.value)
            } else {
                None
            }
        })
        .sum::<usize>()
}

pub(crate) fn sum_gears(matched_symbols: &[Symbol], matched_numbers: &[Number]) -> usize {
    matched_symbols
        .iter()
        .filter_map(|star| {
            if star.symbol != '*' {
                return None;
            }

            let adjacent = matched_numbers
                .iter()
                .filter(|m| m.is_adjacent(star))
                .collect::<Vec<_>>();

            if adjacent.len() != 2 {
                return None;
            }
            Some(adjacent[0].value * adjacent[1].value)
        })
        .sum()
}

pub(crate) fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    reader.lines().map_while(Result::ok).collect()
}
