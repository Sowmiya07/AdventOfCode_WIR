use std::fs::read_to_string;

fn main() {
    println!("Welcome to Day 1!");

    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();

    let content = read_to_string("./src/Day_1.txt")
        .expect("File read failed!");

    for line in content.lines() {
        let pair: Vec<&str> = line.split_whitespace().collect();
        if let (Ok(num1), Ok(num2)) = (pair[0].parse::<i32>(), pair[1].parse::<i32>()) {
            list_1.push(num1);
            list_2.push(num2);
        }

    }

    list_1.sort();
    list_2.sort();

    let mut sum = 0;
    for(m,n) in list_1.iter().zip(list_2.iter()) {
        let diff = (m - n).abs();

        sum = sum + diff;
    }

    print!("{}", sum)

}
