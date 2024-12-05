#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        search::search_east, 
        search::search_north, 
        search::search_north_east, 
        search::search_south, 
        search::search_south_east, 
        search::search_south_west, 
        search::search_west, 
        search::search_north_west
    };

    #[test]
    fn test_search_east() {
        assert_eq!(search_east(0, 0, &vec![vec!['X', 'M', 'A', 'S', '*']]), true);
        assert_eq!(search_east(0, 0, &vec![vec!['X', 'M', 'A']]), false);
        assert_eq!(search_east(0, 0, &vec![vec!['X', 'm', 'A', 'S', '*']]), false);
    }

    #[test]
    fn test_search_west() {
        assert_eq!(search_west(0, 0, &vec![vec!['X', 'S', 'A', 'M', '*']]), false);
        assert_eq!(search_west(0, 4, &vec![vec!['*', 'S', 'A', 'M', 'X']]), true);
        assert_eq!(search_west(0, 2, &vec![vec!['A', 'M', 'X']]), false);
        assert_eq!(search_west(0, 4, &vec![vec!['*', 'S', 'a', 'M', 'X']]), false);
    }

    #[test]
    fn test_search_south() {
        let matrix = vec![vec!['X', '*', '*', '*', '*'],vec!['M', '*', '*', '*', '*'],vec!['A', '*', '*', '*', '*'],vec!['S', '*', '*', '*', '*']];
        assert_eq!(search_south(0, 0, &matrix), true);
        let matrix = vec![vec!['*', '*', '*', '*', 'X'],vec!['*', '*', '*', '*', 'M'],vec!['*', '*', '*', '*', 'A'],vec!['*', '*', '*', '*', 'S']];
        assert_eq!(search_south(0, 4, &matrix), true);
        let matrix = vec![vec!['X', '*', '*', '*', '*'],vec!['m', '*', '*', '*', '*'],vec!['A', '*', '*', '*', '*'],vec!['S', '*', '*', '*', '*']];
        assert_eq!(search_south(0, 0, &matrix), false);
        let matrix = vec![vec!['X', 'S', 'A', 'M', 'X'],vec!['M', 'S', 'A', 'M', 'X'],vec!['A', 'S', 'A', 'M', 'X']];
        assert_eq!(search_south(0, 0, &matrix), false);
    }

    #[test]
    fn test_search_north() {
        let matrix = vec![vec!['*', '*', '*', '*', 'S'],vec!['*', '*', '*', '*', 'A'],vec!['*', '*', '*', '*', 'M'],vec!['*', '*', '*', '*', 'X']];
        assert_eq!(search_north(3, 4, &matrix), true);
        let matrix = vec![vec!['*', '*', '*', '*', 'S'],vec!['*', '*', '*', '*', 'A'],vec!['*', '*', '*', '*', 'm'],vec!['*', '*', '*', '*', 'X']];
        assert_eq!(search_north(3, 4, &matrix), false);
        let matrix = vec![vec!['X', 'S', 'A', 'M', 'X'],vec!['M', 'S', 'A', 'M', 'X'],vec!['A', 'S', 'A', 'M', 'X']];
        assert_eq!(search_north(0, 2, &matrix), false);
    }

    #[test]
    fn test_search_south_east() {
        let matrix = vec![vec!['X', '*', '*', '*', '*'],vec!['*', 'M', '*', '*', '*'],vec!['*', '*', 'A', '*', '*'],vec!['*', '*', '*', 'S', '*']];
        assert_eq!(search_south_east(0, 0, &matrix), true);
        let matrix = vec![vec!['X', '*', '*', '*', '*'],vec!['*', 'm', '*', '*', '*'],vec!['*', '*', 'A', '*', '*'],vec!['*', '*', '*', 'S', '*']];
        assert_eq!(search_south_east(0, 0, &matrix), false);
    }

    #[test]
    fn test_search_south_west() {
        let matrix = vec![vec!['*', '*', '*', '*', 'X'],vec!['*', '*', '*', 'M', '*'],vec!['*', '*', 'A', '*', '*'],vec!['*', 'S', '*', '*', '*']];
        assert_eq!(search_south_west(0, 4, &matrix), true);
        let matrix = vec![vec!['*', '*', '*', '*', 'X'],vec!['*', '*', '*', 'm', '*'],vec!['*', '*', 'A', '*', '*'],vec!['*', 'S', '*', '*', '*']];
        assert_eq!(search_south_west(0, 4, &matrix), false);
    }

    #[test]
    fn test_search_north_east() {
        let matrix = vec![vec!['*', '*', '*', 'S', '*'],vec!['*', '*', 'A', '*', '*'],vec!['*', 'M', '*', '*', '*'],vec!['X', '*', '*', '*', '*']];
        assert_eq!(search_north_east(3, 0, &matrix), true);
        let matrix = vec![vec!['*', '*', '*', 'S', '*'],vec!['*', '*', 'A', '*', '*'],vec!['*', 'm', '*', '*', '*'],vec!['X', '*', '*', '*', '*']];
        assert_eq!(search_north_east(3, 0, &matrix), false);
    }

    #[test]
    fn test_search_north_west() {
        let matrix = vec![vec!['*', 'S', '*', '*', '*'],vec!['*', '*', 'A', '*', '*'],vec!['*', '*', '*', 'M', '*'],vec!['*', '*', '*', '*', 'X']];
        assert_eq!(search_north_west(3, 4, &matrix), true);
        let matrix = vec![vec!['*', 'S', '*', '*', '*'],vec!['*', '*', 'A', '*', '*'],vec!['*', '*', '*', 'm', '*'],vec!['*', '*', '*', '*', 'X']];
        assert_eq!(search_north_west(3, 4, &matrix), false);
    }
}