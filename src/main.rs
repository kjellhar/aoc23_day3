use a23_d3::*;
pub mod a23_d3;

const FILE_NAME: &str = "input.txt";
//const FILE_NAME: &str = "testdata.txt";

fn main() {
    let lines: Vec<String> = read_lines(FILE_NAME);
    let matched_numbers: Vec<Number> = collect_numbers(&lines);
    let matched_symbols: Vec<Symbol> = collect_symbols(&lines);
    let sum: usize = sum_adjacent(&matched_numbers, &matched_symbols);
    println!("Part 1 Sum:  {}", sum);
    let sum: usize = sum_gears(&matched_symbols, &matched_numbers);
    println!("Part 2 Sum:  {}", sum);
}
