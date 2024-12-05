mod search;
mod test;

use std::fs::read_to_string;

fn main() {
    println!("Welcome to Day 4!");

    let result = find_xmas_in_all_directions("./src/data/small_sample.txt");

    println!("Total XMAS in all directions for small sample: {:?}", result);

    let result = find_xmas_in_all_directions("./src/data/large_sample.txt");

    println!("Total XMAS in all directions for large sample: {:?}", result);
}

/// Read the test data file
/// Find the total number of XMAS found in the file in all directions
/// 
/// Returns:
/// A i32 representing the total number of XMAS found
fn find_xmas_in_all_directions(file_path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let matrix = read_test_data_into_matrix(&file_path)?;

    let mut sum  = 0;

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            sum += if search::search_east(row, col, &matrix)  {1} else {0};
            sum += if search::search_west(row, col, &matrix)  {1} else {0};
            sum += if search::search_north(row, col, &matrix)  {1} else {0};
            sum += if search::search_south(row, col, &matrix)  {1} else {0};
            sum += if search::search_north_east(row, col, &matrix)  {1} else {0};
            sum += if search::search_north_west(row, col, &matrix)  {1} else {0};
            sum += if search::search_south_east(row, col, &matrix)  {1} else {0};
            sum += if search::search_south_west(row, col, &matrix)  {1} else {0};
        }
    }

    Ok(sum)
}

/// Read the test data file
/// Load the test data into a matrix
/// 
/// Returns:
/// A Nested Vector Vec<Vec<char>> - matrix representation of the test data
fn read_test_data_into_matrix(file_path: &str) -> Result<Vec<Vec<char>>, Box<dyn std::error::Error>> {
    let content = read_to_string(file_path)?;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in content.lines() {
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    }
    Ok(matrix)
}