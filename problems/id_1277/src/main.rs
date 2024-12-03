///
/// # Count Square Submatrices with All Ones (Medium) [Array, Dynamic Programming, Matrix]
/// Leetcode Problem 1277
///

///
/// # count_squares
///
/// Given a m * n matrix of ones and zeros, return how many square submatrices have all ones.
///
/// ## Arguments
///
/// * `matrix` - `Vec<Vec<i32>>`: A m * n matrix of ones and zeros.
///
/// ## Returns
///
/// * `i32`: The number of square submatrices have all ones.
pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
    // Check if matrix is empty or has no columns
    // Return 0 if either condition is true since no squares can exist
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }

    // Get dimensions of the matrix
    let m = matrix.len(); // Number of rows
    let n = matrix[0].len(); // Number of columns

    // Create a dynamic programming array with same dimensions as input matrix
    // This will store the size of largest square possible at each position
    let mut dyn_prog = vec![vec![0; n]; m];

    // Initialize result counter to track total number of valid squares
    let mut result = 0;

    // Initialize first column (j=0) of dynamic programming array
    // Copy values directly from input matrix as these can only form 1x1 squares
    for i in 0..m {
        dyn_prog[i][0] = matrix[i][0];
        result += dyn_prog[i][0]; // Add to result if cell is 1 (represents a 1x1 square)
    }

    // Initialize first row (i=0) of dynamic programming array
    // Start from j=1 since (0,0) was handled in previous loop
    for j in 1..n {
        dyn_prog[0][j] = matrix[0][j];
        result += dyn_prog[0][j]; // Add to result if cell is 1 (represents a 1x1 square)
    }

    // Process rest of the matrix using dynamic programming
    for i in 1..m {
        for j in 1..n {
            // Only process cells that contain 1
            if matrix[i][j] == 1 {
                // For each cell containing 1, check three adjacent cells:
                // 1. Cell above (i-1,j)
                // 2. Cell to left (i,j-1)
                // 3. Cell diagonally up-left (i-1,j-1)
                // Take minimum of these three values and add 1
                dyn_prog[i][j] = 1 + dyn_prog[i - 1][j]
                    .min(dyn_prog[i][j - 1])
                    .min(dyn_prog[i - 1][j - 1]);

                // Add to result - this value represents number of squares
                // that can end at current position (i,j)
                result += dyn_prog[i][j];
            }
            // If matrix[i][j] is 0, dyn_prog[i][j] remains 0 (initialized value)
        }
    }

    // Return total count of all valid squares found
    result
}

fn main() {
    println!("Hello, world!");
}
