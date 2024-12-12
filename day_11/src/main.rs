use std::{ collections::HashMap, process::exit, time::Instant};

fn main() {
    println!("Welcome to Day 11!");

    // Part 1
    let mut part_1_stone_arrangement: Vec<u64> = vec![125, 17];
    let part_1_blinks = 6;

    let now = Instant::now();
    let mut current_blink = 0;
    
    while current_blink < part_1_blinks {
        part_1_stone_arrangement = blink(&part_1_stone_arrangement);
        current_blink += 1;
    }
    println!("Part 1 Sum {}", part_1_stone_arrangement.len());
    println!("Part 1 took {:?}", now.elapsed());

    
    // Part 2#

    let part_2_stone_arrangement: Vec<u64> = vec![4189, 413, 82070, 61, 655813, 7478611, 0, 8];
    let part_2_blinks = 75;

    let now = Instant::now();
    let part_2_sum: u64 = part_2_improved(part_2_stone_arrangement, part_2_blinks);

    println!("Part 2 Sum {}", part_2_sum);
    println!("Part 2 Took {:?}", now.elapsed());
    
}

fn part_2_improved(nums: Vec<u64>, blinks: u8) -> u64{
    let mut count_map = HashMap::new();

    for num in nums {
        *count_map.entry(num).or_insert(0) += 1;
    }

    for i in 0..blinks {
        let mut next_count_map = HashMap::new();
        for (key, value) in count_map.iter() {
            let next_stones = find_next_stones(*key);
            for stone in next_stones {
                *next_count_map.entry(stone).or_insert(0) += *value;
            }
        }
        count_map = next_count_map;
    }
    return count_map.values().sum();
}

fn find_next_stones(num: u64) -> Vec<u64> {
    if num == 0 {
        return vec![1];
    } else if check_for_even_no_of_digits(num) {
        let split = split_even_digits(num);
        return vec![split.0, split.1];
    } else {
        return vec![num * 2024];
    }
}

fn check_for_even_no_of_digits(num: u64) -> bool {
    return num.to_string().len() % 2 == 0;
}

fn split_even_digits(num: u64) -> (u64, u64){
    let num_str = num.to_string();
    
    let (first, second) = num_str.split_at(num.to_string().len() / 2);

    match (first.parse::<u64>(), second.parse::<u64>()) {
        (Ok(first), Ok(second)) => {
            return (first, second);
        }
        _ => {
            println!("Error splitting and parsing even digit number {}", num);
            exit(0)
        }
    }
}

fn blink(stone_arrangement:  &Vec<u64>) -> Vec<u64>{
    let mut index = 0;

    let mut new_arrangement: Vec<u64> = Vec::with_capacity(stone_arrangement.len() * 2);

    while index < stone_arrangement.len() {

        if stone_arrangement[index] == 0 {
            new_arrangement.push(1);
        } else if check_for_even_no_of_digits(stone_arrangement[index]) {
            let (first_part, second_part) = split_even_digits(stone_arrangement[index]);
            new_arrangement.push(first_part);
            new_arrangement.push(second_part);
        } else {
            let val = stone_arrangement[index] * 2024;
            new_arrangement.push(val);
        }
        index += 1;
    }

    new_arrangement
}