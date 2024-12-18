use std::{collections::HashMap, fs::read_to_string};

fn main() {
    println!("Welcome to Day 7!");

    let inputs = read_input();

    let mut sum = 0;

    for mut input in inputs {
        let mut current_combination: Vec<u64> = Vec::new();

        let result = input.remove(0);

        if let Some(true) = operator_combination_recursion(result, 
                                        &input, 
                                        &mut current_combination, 
                                        input.len() - 1) {
            println!("Solution found");
            sum += result;
        } else {
            println!("Solution not found");
        }
    }

    println!("Result {}", sum);
}

fn find_calibrations(result: u64, operands: &Vec<u64>, op_combo: &mut Vec<u64>) -> bool {

    let mut total = operands[0];

    for (idx, opr) in op_combo.iter().enumerate() {
        if *opr == 0 {
            total += operands[idx + 1];
        } else {
            total *= operands[idx + 1];
        }
    }

    // println!("Result {}", result);

    total == result
}


fn operator_combination_recursion(result: u64, operands: &Vec<u64>, combination: &mut Vec<u64>, length: usize) -> Option<bool> {

    if combination.len() == length {
        let solved = find_calibrations(result, &operands, combination);
        if solved {
            return Some(true);
        } else {
            return Some(false);
        }
    }

    for i in 0..2 {
        combination.insert(0, i);
        if let Some(true) = operator_combination_recursion(result, operands, combination, length) {
            return Some(true);
        }
        combination.remove(0);
    }

    None
}

fn read_input() -> Vec<Vec<u64>> {
    let contents = read_to_string("./src/data.txt").expect("File cannot be read");

    let mut input: Vec<Vec<u64>> = Vec::new();
    
    for line in contents.lines() {
        if let Some((result, numbers)) = line.split_once(":") {

            let operands: Vec<u64> = numbers.trim().split(" ")
                                        .map(|n| n.parse::<u64>().unwrap())
                                        .collect();

            let result = result.parse::<u64>().unwrap();

            let mut record = vec![result];

            for op in operands {
                record.push(op);
            }

            input.push(record);
        }
    }
    input
}