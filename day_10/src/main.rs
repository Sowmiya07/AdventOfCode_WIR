use std::{collections::HashSet, fs::read_to_string, vec};

fn main() {
    println!("Welcome to Day 10!");

    // let map: Vec<Vec<u8>> = vec![vec![0, 1, 2, 3], vec![1, 2, 3, 4], vec![8, 7, 6, 5], vec![9, 8, 7, 6]];

    let mut map: Vec<Vec<u8>> = Vec::new();

    let content = read_to_string("./src/sample.txt").expect("Couldn't open the file");

    for line in content.lines() {
        let row: Vec<u8> = line.chars()
        .map(|el| el.to_digit(10).unwrap_or(10) as u8)
        .collect();
        map.push(row);
    }

    let rows = map.len();
    let cols = map[0].len();

    let mut sum = 0;

    for i in 0..rows {
        for j in 0..cols {
            if map[i][j] == 0 {
                let mut nodes: HashSet<(usize, usize)> = HashSet::new();
                recursive_sum(i, j, rows, cols, &map, 0, &mut nodes);
                sum += nodes.len();
            }
        }
    }

    println!("sum {}", sum);

}

fn recursive_sum(i: usize, j: usize, rows: usize, cols: usize, map: &Vec<Vec<u8>>, last: u8, nodes: &mut HashSet<(usize, usize)>) {
    if map[i][j] == 9 && !nodes.contains(&(i, j)){
        nodes.insert((i, j));
    }

    // let mut max_sum = sum;

    if j + 1 < cols && map[i][j+1] == last + 1 {
        recursive_sum(i, j+1, rows, cols, map, last + 1, nodes);
    } 
    
    if i >= 1 && map[i - 1][j] == last + 1 {
       recursive_sum(i - 1, j, rows, cols, map, last + 1, nodes);
    } 
    
    if j >= 1 && map[i][j - 1] == last + 1 {
       recursive_sum(i, j - 1, rows, cols, map, last + 1, nodes);
    } 
    
    if i + 1 < rows && map[i + 1][j] == last + 1 {
       recursive_sum(i + 1, j, rows, cols, map, last + 1, nodes);
    }
}
