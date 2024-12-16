use std::{collections::HashMap, fs::read_to_string};

fn main() {
    println!("Welcome to Day 5!");

    let (rule_map, page_updates) = 
                read_input_file("./src/data/small_sample.txt");

    let mut sum_of_middle_elements: u64 = 0;
    for update in page_updates {
        let mut valid_update: bool = true;
        for (pg_idx, page) in update.iter().enumerate() {
            if rule_map.contains_key(&page) {
                let succeeding_pages = rule_map.get(&page);

                match succeeding_pages {
                    Some(s_pages) => {
                        for s_page in s_pages {
                            let position = update.iter().position(|pg| pg == s_page);
                            match position {
                                Some(pos) => {
                                    if pos < pg_idx {
                                        valid_update = false;
                                    }
                                },
                                None => {
                                    println!("Good the page {} not part of update", s_page);
                                }
                            }
                        }
                    },
                    None => {
                        print!("None found");
                    }
                }
            }
        }
        if valid_update {
            let mid  = update.get(update.len() / 2);
            match mid {
                Some(m) => {
                    sum_of_middle_elements += m;
                },
                None => {
                    println!("Couldn't find middle element")
                }
            }
        }
    }
    println!("Sum {}", sum_of_middle_elements);
}

fn read_input_file(file_path: &str) -> (HashMap<u64, Vec<u64>>, Vec<Vec<u64>>) {
    let file_content = read_to_string(file_path).expect("Failed to open file...");

    let mut rule_map: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut page_updates: Vec<Vec<u64>> = Vec::new();

    for line in file_content.lines() {
        if line.contains("|") {
            let pages: Vec<u64> = line.split("|").map(|page| page.parse::<u64>().unwrap()).collect();

            if pages.len() == 2 {
                rule_map.entry(pages[0])
                .or_insert_with(Vec::new)
                .push(pages[1]);
            }
            
        } else if line.contains(",") {
            let updates: Vec<u64> = line.split(",").map(|update| update.parse().unwrap()).collect();
            page_updates.push(updates);
        }
    }
    return (rule_map, page_updates)
}