/// checks if XMAS can be found horizontally-forwards
/// 
/// Arguments:
///    row & col: representing the index of the character in the matrix
///    matrix: representing the input matrix
///  
/// Returns:
///     bool: true representing a match, false representing no match
pub fn search_east(row: usize, col: usize, matrix: &Vec<Vec<char>>) -> bool {
    if col + 3 < matrix[row].len() {
        return matrix[row][col] == 'X' && 
            matrix[row][col+1] =='M' && 
            matrix[row][col+2] == 'A' && 
            matrix[row][col+3] == 'S';
    } else {
        return false;
    }
}

/// checks if XMAS can be found horizontally-backwards
/// 
/// Arguments:
///    row & col: representing the index of the character in the matrix
///    matrix: representing the input matrix
///  
/// Returns:
///     bool: true representing a match, false representing no match
pub fn search_west(row: usize, col: usize, matrix: &Vec<Vec<char>>) -> bool {
    if col >= 3 {
        return matrix[row][col] == 'X' && 
            matrix[row][col-1] =='M' && 
            matrix[row][col-2] == 'A' && 
            matrix[row][col-3] == 'S';
    } else {
        return false;
    }
}

/// checks if XMAS can be found vertically-downwards
/// 
/// Arguments:
///    row & col: representing the index of the character in the matrix
///    matrix: representing the input matrix
///  
/// Returns:
///     bool: true representing a match, false representing no match
pub fn search_south(row: usize, col: usize, matrix: &Vec<Vec<char>>) -> bool {
    if row + 3 < matrix.len() {
        return matrix[row][col] == 'X' && 
            matrix[row+1][col] =='M' && 
            matrix[row+2][col] == 'A' && 
            matrix[row+3][col] == 'S';
    } else {
        return false;
    }
}

/// checks if XMAS can be found vertically-upwards
/// 
/// Arguments:
///    row & col: representing the index of the character in the matrix
///    matrix: representing the input matrix
///  
/// Returns:
///     bool: true representing a match, false representing no match
pub fn search_north(row: usize, col: usize, matrix: &Vec<Vec<char>>) -> bool {
    if row >= 3 {
        return matrix[row][col] == 'X' && 
            matrix[row-1][col] =='M' && 
            matrix[row-2][col] == 'A' && 
            matrix[row-3][col] == 'S';
    } else {
        return false;
    }
}

/// checks if XMAS can be found diagonal-top-left-to-bottom-right
/// 
/// Arguments:
///    row & col: representing the index of the character in the matrix
///    matrix: representing the input matrix
///  
/// Returns:
///     bool: true representing a match, false representing no match
pub fn search_south_east(row: usize, col: usize, matrix: &Vec<Vec<char>>) -> bool {
    if row + 3 < matrix.len() && col + 3 < matrix[row].len() {
        return matrix[row][col] == 'X' && 
            matrix[row+1][col+1] =='M' && 
            matrix[row+2][col+2] == 'A' && 
            matrix[row+3][col+3] == 'S';
    } else {
        return false;
    }
}

/// checks if XMAS can be found diagonal-top-right-to-bottom-left
/// 
/// Arguments:
///    row & col: representing the index of the character in the matrix
///    matrix: representing the input matrix
///  
/// Returns:
///     bool: true representing a match, false representing no match
pub fn search_south_west(row: usize, col: usize, matrix: &Vec<Vec<char>>) -> bool {
    if row + 3 < matrix.len() && col >= 3 {
        return matrix[row][col] == 'X' && 
            matrix[row+1][col-1] =='M' && 
            matrix[row+2][col-2] == 'A' && 
            matrix[row+3][col-3] == 'S';
    } else {
        return false;
    }
}

/// checks if XMAS can be found diagonal-bottom-left-to-top-right
/// 
/// Arguments:
///    row & col: representing the index of the character in the matrix
///    matrix: representing the input matrix
///  
/// Returns:
///     bool: true representing a match, false representing no match
pub fn search_north_east(row: usize, col: usize, matrix: &Vec<Vec<char>>) -> bool {
    if row >= 3 && col + 3 < matrix[row].len() {
        return matrix[row][col] == 'X' && 
            matrix[row-1][col+1] =='M' && 
            matrix[row-2][col+2] == 'A' && 
            matrix[row-3][col+3] == 'S';
    } else {
        return false;
    }
}

/// checks if XMAS can be found diagonal-bottom-right-to-top-left
/// 
/// Arguments:
///    row & col: representing the index of the character in the matrix
///    matrix: representing the input matrix
///  
/// Returns:
///     bool: true representing a match, false representing no match
pub fn search_north_west(row: usize, col: usize, matrix: &Vec<Vec<char>>) -> bool {
    if row >= 3 && col >= 3 {
        return matrix[row][col] == 'X' && 
            matrix[row-1][col-1] =='M' && 
            matrix[row-2][col-2] == 'A' && 
            matrix[row-3][col-3] == 'S';
    } else {
        return false;
    }
}