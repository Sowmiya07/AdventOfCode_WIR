use std::fs::read_to_string;
use regex::Regex;

fn main() {
    println!("Welcome to Day 3!");
    
    let part_1_result = mull_it_over_part_1();
    println!("Result of Part 1 {}", part_1_result);
    
    let part_2_result  = mull_it_over_part_2();
    println!("Result of Part 1 {}", part_2_result);
}

/// Finds the sum of the all the factors in the corrupted memory
/// 
/// # Returns
/// A `u64` sum of all the factors in the corrupted memory
/// 
fn mull_it_over_part_1() -> u64 {

    let corrupted_memory  = read_to_string("./src/part_1.txt")
                                        .expect("Failed to read the input file!!!");

    return multiplication(corrupted_memory.as_str());
}
 
/// Finds the sum of the all the factors in the corrupted memory in splits
/// 
/// # Returns
/// A `u64` sum of all the factors in the corrupted memory
/// 
fn mull_it_over_part_2() -> u64 {
    let corrupted_memory  = read_to_string("./src/part_2.txt")
                                        .expect("Failed to read the input file!!!");

    let splits_by_dont: Vec<&str> = corrupted_memory
                                        .split("don't()")
                                        .collect();

    let mut aggregated_sum: u64 = multiplication(splits_by_dont[0]);
    for index in 1..splits_by_dont.len() {
        let split = splits_by_dont[index];
        if split.contains("do()") {
            let splits_by_do: Vec<&str> = split.splitn(2, "do()").collect();
            if splits_by_do.len() > 1 {
                aggregated_sum += multiplication(splits_by_do[1]);
            }
        }
    }
    return aggregated_sum;
}


///1. Match the input str with the regex pattern
///2. find all valid factors mul(x,y)
///3. multiply the factors and add to the sum
/// 
/// # Arguments
/// * `corrupted_memory` - A str representing whole/part of the corrupted memory.
///
/// # Returns
/// A `u64` sum of all the factors
/// 
fn multiplication(corrupted_memory: &str) -> u64 {
    let mul_pattern = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut factors = vec![];
    for capture in mul_pattern.captures_iter(corrupted_memory) {
        let (_, [multiplicand, multiplier]) = capture.extract();
        factors.push((
            multiplicand.parse::<u32>().unwrap(), 
            multiplier.parse::<u32>().unwrap()
        ));
    }
    
    let mut sum: u64 = 0;
    for factor in factors.iter() {
       let result = (factor.0 * factor.1) as u64;
       sum += result;
    }

    return sum;
}


#[cfg(test)]
mod tests {
    use crate::multiplication;

    #[test]
    fn test_multiplication() {
        let corrupted_memory = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(multiplication(corrupted_memory), 161);
    }
}