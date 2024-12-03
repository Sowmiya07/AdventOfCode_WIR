use std::fs::read_to_string;

fn main() {
    println!("Welcome to Day 2!");

    let mut safe_count = 0;
    let mut safe_count_with_dampener = 0;

    let file_content = read_to_string("./src/input.txt")
        .expect("File read failed!");

    for report in file_content.lines() {
        let levels: Vec<u16> = report.split(" ")
                                .map(|x| x.parse::<u16>().unwrap())
                                .collect();
        
        if levels.len() < 2 {
            panic!("No levels found")
        }

        // Calculating the safe count without dampener
        let current_report_safety = check_report(&levels);
        safe_count += current_report_safety;

        // Calculating the safe count with dampener
        if current_report_safety == 1 {
            safe_count_with_dampener += 1;
        } else {
            for index in 0..levels.len() {
                let modified_levels = remove_ith_level(&levels, index);
                
                if check_report(&modified_levels) == 1 {
                    safe_count_with_dampener += 1;
                    break;
                }
            }
        }
        
    }

    println!("Total safe levels {}", safe_count);
    println!("Total safe levels with dampener {}", safe_count_with_dampener);
}

fn remove_ith_level(levels: &Vec<u16>, index: usize) -> Vec<u16> {
    let mut modified_levels = Vec::new();
    for (i, &item) in levels.iter().enumerate() {
        if i != index {
            modified_levels.push(item);
        }
    }
    modified_levels
}

fn check_report(levels: &Vec<u16>) -> u16 {
    let first_level = levels[0];
    let second_level = levels[1];

    if first_level > second_level {
        return check_decreasing_report(&levels);
    } else if first_level < second_level {
        return check_increasing_report(&levels);
    } else {
        return 0;
    }
}

fn check_decreasing_report(list: &Vec<u16>) -> u16{
    let mut result: u16 = 1;
    
    for m in 1..list.len() {
        if list[m] >= list[m-1] ||  not_in_range(list[m], list[m-1]) {
            result = 0;
            break;
        }
    }
    return result;
}

fn check_increasing_report(list: &Vec<u16>) -> u16{
    let mut result: u16 = 1;
    for m in 1..list.len() {
        if list[m] <= list[m-1] || not_in_range(list[m], list[m-1]){
            result = 0;
            break;
        }
    }
    return result;
}

fn not_in_range(m: u16, n: u16) -> bool{
    let range = 1..4;
    return !range.contains(&m.abs_diff(n));
}