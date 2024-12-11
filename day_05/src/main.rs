use std::{collections::HashMap, fs::read_to_string};

fn main() {
    println!("Welcome to Day 5!");

    let (rule_map, page_updates) = read_input_file("./src/data/small_sample.txt");

    for update in page_updates {
        let mut valid: bool = false;
        for page in update {
            if rule_map.contains_key(&page) {
                let value = rule_map.get(&page);
                let key = page;

                let idx_2 = if let Some(val) = value {
                    update.iter().position(|v| v == val); // We borrow `update` here
                };
            
                // `update` is still available here because we only borrowed it with `.iter()`
                let idx_1 = update.iter().position(|v| v == &key);

                
                match (idx_1, idx_2) {
                    (Some(i1), Some(i2)) => {
                        if i1 < i2 {
                            println!("idx_1 is less than idx_2: {} < {}", i1, i2);
                        } else if i1 > i2 {
                            println!("idx_1 is greater than idx_2: {} > {}", i1, i2);
                        } else {
                            println!("idx_1 is equal to idx_2: {} == {}", i1, i2);
                        }
                    },
                    _ => {
                        println!("At least one of the indices was not found (None).");
                    }
                }
            }
        }
        println!("{}", valid)
    }
}

fn read_input_file(file_path: &str) -> (HashMap<u8, u8>, Vec<Vec<u8>>) {
    let file_content = read_to_string(file_path).expect("Failed to open file...");

    let mut rule_map: HashMap<u8, u8> = HashMap::new();
    let mut page_updates: Vec<Vec<u8>> = Vec::new();

    for line in file_content.lines() {
        if line.contains("|") {
            let pages: Vec<u8> = line.split("|").map(|page| page.parse::<u8>().unwrap()).collect();

            if pages.len() == 2 {
                rule_map.insert(pages[0], pages[1]);
                // rule_map.insert(pages[1], pages[0]);
            }
            
        } else if line.contains(",") {
            let updates: Vec<u8> = line.split(",").map(|update| update.parse().unwrap()).collect();
            page_updates.push(updates);
        }
    }
    return (rule_map, page_updates)
}