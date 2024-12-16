use std::{collections::HashMap, fs::read_to_string};

use regex::Regex;

fn main() {

  println!("Welcome to Day 13!");

  let data_maps = read_file();

  let mut total  = 0;

  for map in data_maps {

    let result  = find_min_button_presses(
        map["a_x"], 
        map["a_y"], 
        map["b_x"],
        map["b_y"],
        map["prize_x"], 
        map["prize_y"]
    );
    
    total += result;
  }

  println!("Total {}", total);
}

/// utility to read the file, match the regex and extract the values to store in map
/// 
/// Returns:
///     Vector of Maps which stores all the values read from the file
/// 
/// Note: For each case, map "a_x" "b_x" "a_y" "b_y" "prize_x" "prize_y" stored
/// Length of the vector will be equal to the number of cases found in the file
/// 
fn read_file() -> Vec<HashMap<String, i128>> {
    let contents  = read_to_string("./src/data.txt")
    .expect("Failed to read data file");

    let btn_a_regex = regex::Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let btn_b_regex = regex::Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = regex::Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut data : Vec<HashMap<String, i128>> = Vec::new();

    let mut map = HashMap::new();
    
    for line in contents.lines() {

        if btn_a_regex.is_match(line) {
            regex_data_extract(&btn_a_regex, line, &mut map, &("a_x", "a_y"));
        } else if btn_b_regex.is_match(line) {
            regex_data_extract(&btn_b_regex, line, &mut map, &("b_x", "b_y"));
        } else if prize_regex.is_match(line) {
            regex_data_extract(&prize_regex, line, &mut map, &("prize_x", "prize_y"));
        } else {
            data.push(map.clone());
            map.clear();
        }
    }

    data.push(map.clone());
    data
}

/// Extracts the values from string
/// Arguments:
///     re: matched regex
///     line: line read from the file
///     map: to store the values
///     keys: map keys
fn regex_data_extract(re: &Regex, line: &str, map: &mut HashMap<String, i128>, keys: &(&str, &str)) {
    re.captures_iter(line).for_each(|caps| {
        match &caps[1].parse::<i128>() {
            Ok(parsed_val) => map.insert(keys.0.to_string(), *parsed_val),
            Err(_e) => {
                eprintln!("Failed to parse value for key '{}'", &caps[1]);
                std::process::exit(1);
            },
        };
        match &caps[2].parse::<i128>() {
            Ok(parsed_val) => map.insert(keys.1.to_string(), *parsed_val),
            Err(_e) => {
                eprintln!("Failed to parse value for key '{}'", &caps[2]);
                std::process::exit(1);
            },
        };
    }); 
}

/// Finds the minimum number of times the Button A (x) and Button B (y) should be pressed to move the claw
/// by adding Button A X axis move and Button B X axis move
/// Also, by adding Button B Y axis move and Button B Y axis move
/// Formula used:
///     x * a_x + y * b_x = location of prize along X axis
/// Arguments: 
///     a_x: Button A X axis move
///     a_y: Button A Y axis move
///     b_x: Button B X axis move
///     b_y: Button B Y axis move
///     prize_x: X coordinate of the prize
///     prize_x: Y coordinate of the prize
/// 
/// Returns: i128
///     0 - when button presses less than 100 cannot be found
///     else, cost - tokens to reach the prize
/// 
/// Note: Here tokens represent minimum button presses * cost of pressing the token
/// Cost is 3 for Button A
/// Cost is 1 for Button B
/// 
fn find_min_button_presses(a_x: i128, a_y: i128, b_x: i128, b_y: i128, prize_x_coord: i128, prize_y_coord: i128) -> i128 {
    let mut btn_a_min_press: i128 = 1;
    let mut btn_b_min_press: i128 = 1;

    const BTN_A_TOKENS: i128 = 3;
    const BTN_B_TOKENS: i128 = 1;

    let mut solved = false;

    while btn_a_min_press <= 100 {

        btn_b_min_press = (prize_x_coord - (btn_a_min_press * a_x)) / b_x;

        let res_y = (btn_a_min_press * a_y) + (btn_b_min_press * b_y);

        if res_y == prize_y_coord {
            solved = true;
            break;
        }
    
        btn_a_min_press += 1;
        
        
        if btn_b_min_press <= 0 {
            break;
        }
    }

    
    if solved {
        return (btn_a_min_press * BTN_A_TOKENS) + (btn_b_min_press * BTN_B_TOKENS);
    }

    return 0;
}