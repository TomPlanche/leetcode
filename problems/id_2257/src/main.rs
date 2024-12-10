///
/// # Count Unguarded Cells in the Grid (Medium) [Array, Matrix, Simulation]
/// LeetCode Problem 2257
///

/// Counts the number of unguarded cells in a grid with guards and walls
///
/// ## Arguments
/// * `m` - Number of rows in the grid (height)
/// * `n` - Number of columns in the grid (width)
/// * `guards` - 2D vector of guard positions
/// * `walls` - 2D vector of wall positions
///
/// ## Returns
/// * `i32` - Number of unguarded cells in the grid
///
/// ## Approach
/// 1. Create a grid to track cell status
/// 2. Mark wall and guard positions
/// 3. Propagate guard visibility in four directions
/// 4. Count unguarded cells
pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (m as usize, n as usize);

    // Grid status: 0 = unguarded, 1 = wall, 2 = guard, 3 = guarded
    let mut grid = vec![vec![0; n]; m];

    // Mark walls and guards
    for wall in walls {
        let (r, c) = (wall[0] as usize, wall[1] as usize);
        grid[r][c] = 1;
    }
    for guard in &guards {
        let (r, c) = (guard[0] as usize, guard[1] as usize);
        grid[r][c] = 2;
    }

    // Directions: North, East, South, West
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    // Mark cells guarded by guards in all four directions
    for guard in guards {
        let (gr, gc) = (guard[0] as usize, guard[1] as usize);

        for (dr, dc) in directions {
            let (mut r, mut c) = (gr as i32 + dr, gc as i32 + dc);

            while r >= 0 && r < m as i32 && c >= 0 && c < n as i32 {
                let (r_idx, c_idx) = (r as usize, c as usize);

                // Stop if wall or another guard encountered
                if grid[r_idx][c_idx] == 1 || grid[r_idx][c_idx] == 2 {
                    break;
                }

                // Mark cell as guarded
                grid[r_idx][c_idx] = 3;

                // Move to next cell in direction
                r += dr;
                c += dc;
            }
        }
    }

    // Count unguarded cells
    grid.iter().flatten().filter(|&&cell| cell == 0).count() as i32
}

fn main() {
    println!("LeetCode problem 2257: Count Unguarded Cells in the Grid");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_unguarded() {
        // Test Case 1: Multiple guards and walls
        assert_eq!(
            count_unguarded(
                4,
                6,
                vec![vec![0, 0], vec![1, 1], vec![2, 3]],
                vec![vec![0, 1], vec![2, 2], vec![1, 4]]
            ),
            7
        );

        // Test Case 2: Single guard with surrounding walls
        assert_eq!(
            count_unguarded(
                3,
                3,
                vec![vec![1, 1]],
                vec![vec![0, 1], vec![1, 0], vec![2, 1], vec![1, 2]]
            ),
            4
        );
    }
}
