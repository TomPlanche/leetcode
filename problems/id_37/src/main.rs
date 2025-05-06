//!
//! # Sudoku Solver (Hard) [Array, Hash Table, Backtracking, Matrix]
//! Leetcode Problem 37
//!

///
/// # Sudoku struct
/// - `board` - a mutable reference to a 9x9 board.
/// - `squares` - a 9x10 array to store the number of occurrences of each digit in each 3x3 square.
/// - `rows` - a 9x10 array to store the number of occurrences of each digit in each row.
/// - `cols` - a 9x10 array to store the number of occurrences of each digit in each column.
struct Sudoku<'a> {
    board: &'a mut Vec<Vec<char>>,
    squares: [[usize; 10]; 9],
    rows: [[usize; 10]; 9],
    cols: [[usize; 10]; 9],
}

/// This struct represents a Sudoku puzzle solver.
///
/// # Arguments
/// * `board`: A mutable reference to a 9x9 board represented as a vector of vectors of characters.
/// * `squares`: A 9x10 array to store the number of occurrences of each digit (1-9) in each 3x3 square.
/// * `rows`: A 9x10 array to store the number of occurrences of each digit (1-9) in each row.
/// * `cols`: A 9x10 array to store the number of occurrences of each digit (1-9) in each column.
///
/// The extra element in each array (making them 10 instead of 9) is to allow for 1-based indexing of digits.
///
/// # Lifetime
/// The lifetime 'a is used to ensure that the reference to the board remains valid for the entire lifetime of the Sudoku struct.
///
/// # Implementation
/// The implementation of this struct includes methods for:
/// - Creating a new Sudoku instance
/// - Solving the Sudoku puzzle
/// - Checking if a digit placement is valid
/// - Placing a digit on the board
/// - Reversing a digit placement (for backtracking)
///
/// The solving algorithm uses a backtracking approach to explore all possible solutions.
impl<'a> Sudoku<'a> {
    /// Creates a new Sudoku instance from a given board.
    ///
    /// ## Arguments
    /// * `board` - A mutable reference to a 9x9 vector of characters representing the Sudoku board.
    ///
    /// ## Returns
    /// Returns a new Sudoku struct initialized with the given board and empty counting arrays.
    ///
    /// ## Details
    /// This function performs the following steps:
    /// 1. Creates a new Sudoku struct with the given board and zeroed-out counting arrays.
    /// 2. Iterates through each cell of the board.
    /// 3. For non-empty cells (cells not containing '.'), it calls the `place` method to update the counting arrays.
    /// 4. Returns the fully initialized Sudoku struct.
    ///
    /// ## Note
    /// The `place` method is responsible for updating the `squares`, `rows`, and `cols` arrays
    /// to reflect the presence of digits on the board.
    pub fn new(board: &'a mut Vec<Vec<char>>) -> Self {
        let mut sudoku = Self {
            board,
            squares: [[0usize; 10]; 9],
            rows: [[0usize; 10]; 9],
            cols: [[0usize; 10]; 9],
        };

        for row in 0..9 {
            for col in 0..9 {
                if sudoku.board[row][col] != '.' {
                    sudoku.place(row, col, sudoku.board[row][col]);
                }
            }
        }
        return sudoku;
    }

    /// Solves the Sudoku puzzle using a backtracking algorithm.
    ///
    /// ## Arguments
    /// * `i` - The current index in the board (0 to 80, representing cells from left to right, top to bottom).
    ///
    /// ## Returns
    /// Returns a boolean indicating whether a solution was found.
    ///
    /// ## Details
    /// This recursive function performs the following steps:
    /// 1. If the index is 81 or greater, the entire board has been filled, so return true.
    /// 2. Calculate the current row and column from the index.
    /// 3. If the current cell is not empty ('.'), move to the next cell.
    /// 4. For empty cells, try digits 1 through 9:
    ///    a. Check if the digit is valid in the current position.
    ///    b. If valid, place the digit and recursively solve the next cell.
    ///    c. If the recursive call returns true, a solution is found, so return true.
    ///    d. If not, backtrack by reversing the placement and try the next digit.
    /// 5. If no digit works, return false to trigger backtracking.
    ///
    /// ## Note
    /// This method uses the `is_valid`, `place`, and `reverse` helper methods to manage
    /// the board state and check move validity.
    pub fn solve(&mut self, i: usize) -> bool {
        if i >= 81 {
            // entire board has been filled
            return true;
        }
        let (row, col) = (i / 9, i % 9);

        // if the cell is not empty, move to the next cell
        if self.board[row][col] != '.' {
            return self.solve(i + 1);
        }

        // iterate all possible candidates.
        for digit in '1'..='9' {
            if self.is_valid(row, col, digit) {
                // try this partial candidate solution
                self.place(row, col, digit);
                // given the candidate, explore further.
                if self.solve(i + 1) {
                    return true;
                }
                // backtrack
                self.reverse(row, col);
            }
        }
        return false;
    }

    /// Checks if placing a digit in a specific position is valid.
    ///
    /// ## Arguments
    /// * `row` - The row index (0-8) of the position to check.
    /// * `col` - The column index (0-8) of the position to check.
    /// * `digit` - The digit (character '1' to '9') to check for validity.
    ///
    /// ## Returns
    /// Returns a boolean indicating whether the digit can be placed at the specified position.
    ///
    /// ## Details
    /// This function performs the following checks:
    /// 1. Converts the character digit to its numeric value (1-9).
    /// 2. Checks if the digit is already present in the 3x3 square containing the cell.
    /// 3. Checks if the digit is already present in the same row.
    /// 4. Checks if the digit is already present in the same column.
    ///
    /// ## Note
    /// The function uses the `squares`, `rows`, and `cols` arrays to perform quick checks
    /// without needing to iterate through the entire board.
    ///
    /// ## Complexity
    /// Time complexity: O(1) - constant time operations
    /// Space complexity: O(1) - no additional space used
    pub fn is_valid(&self, row: usize, col: usize, digit: char) -> bool {
        let digit = digit as usize - '0' as usize;

        return self.squares[row / 3 * 3 + col / 3][digit] == 0
            && self.rows[row][digit] == 0
            && self.cols[col][digit] == 0;
    }

    /// Places a digit on the Sudoku board and updates the counting arrays.
    ///
    /// ## Arguments
    /// * `row` - The row index (0-8) where the digit will be placed.
    /// * `col` - The column index (0-8) where the digit will be placed.
    /// * `digit` - The digit (character '1' to '9') to be placed.
    ///
    /// ## Details
    /// This function performs the following steps:
    /// 1. Places the digit character on the board at the specified position.
    /// 2. Converts the character digit to its numeric value (1-9).
    /// 3. Increments the count for this digit in the corresponding 3x3 square.
    /// 4. Increments the count for this digit in the corresponding row.
    /// 5. Increments the count for this digit in the corresponding column.
    ///
    /// ## Note
    /// - This method assumes that the placement is valid and does not perform any validity checks.
    /// - The `squares`, `rows`, and `cols` arrays are updated to reflect the new digit placement.
    ///
    /// ## Complexity
    /// Time complexity: O(1) - constant time operations
    /// Space complexity: O(1) - no additional space used
    pub fn place(&mut self, row: usize, col: usize, digit: char) {
        self.board[row][col] = digit;

        let digit = digit as usize - '0' as usize;

        self.squares[row / 3 * 3 + col / 3][digit] += 1;
        self.rows[row][digit] += 1;
        self.cols[col][digit] += 1;
    }

    /// Reverses a digit placement on the Sudoku board and updates the counting arrays.
    ///
    /// ## Arguments
    /// * `row` - The row index (0-8) of the digit to be removed.
    /// * `col` - The column index (0-8) of the digit to be removed.
    ///
    /// ## Details
    /// This function performs the following steps:
    /// 1. Retrieves the digit at the specified position and converts it to its numeric value (1-9).
    /// 2. Decrements the count for this digit in the corresponding 3x3 square.
    /// 3. Decrements the count for this digit in the corresponding row.
    /// 4. Decrements the count for this digit in the corresponding column.
    /// 5. Replaces the digit on the board with a '.' to indicate an empty cell.
    ///
    /// ## Note
    /// - This method is typically used during backtracking to undo a previous placement.
    /// - The `squares`, `rows`, and `cols` arrays are updated to reflect the removal of the digit.
    ///
    /// ## Complexity
    /// Time complexity: O(1) - constant time operations
    /// Space complexity: O(1) - no additional space used

    pub fn reverse(&mut self, row: usize, col: usize) {
        let digit = self.board[row][col] as usize - '0' as usize;

        self.squares[row / 3 * 3 + col / 3][digit] -= 1;
        self.rows[row][digit] -= 1;
        self.cols[col][digit] -= 1;

        self.board[row][col] = '.';
    }
}

pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let mut sudoku = Sudoku::new(board);

    sudoku.solve(0);
}

fn main() {
    let mut board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    solve_sudoku(&mut board);

    for row in board {
        println!("{:?}", row);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        solve_sudoku(&mut board);

        let expected = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];

        assert_eq!(board, expected);
    }
}
