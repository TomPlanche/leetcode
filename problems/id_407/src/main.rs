///
/// # Trapping Rain Water II (Hard) [Array, Breadth First Search, Heap (priority Queue), Matrix]
/// LeetCode Problem 407
///
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Represents a cell in the height map with its coordinates and height
#[derive(Copy, Clone, Eq, PartialEq)]
struct Cell {
    height: i32,
    row: usize,
    col: usize,
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        other.height.cmp(&self.height) // Reverse ordering for min-heap
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// # `trap_rain_water`
/// Traps rain water in a 2D elevation map and returns the total volume.
///
/// ## Arguments
/// * `height_map` - A 2D vector representing the elevation map where each cell contains the height
///
/// ## Returns
/// * `i32` - The total volume of water that can be trapped
///
/// ## Algorithm
/// Uses a priority queue based approach:
/// 1. Start from the border cells (which can't hold water)
/// 2. Process cells from lowest to highest height
/// 3. For each cell, check unvisited neighbors
/// 4. Calculate trapped water based on the minimum height of surrounding walls
pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
    if height_map.is_empty() || height_map[0].is_empty() {
        return 0;
    }

    let rows = height_map.len();
    let cols = height_map[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut heap = BinaryHeap::new();

    // Add border cells to heap
    for i in 0..rows {
        heap.push(Cell {
            height: height_map[i][0],
            row: i,
            col: 0,
        });
        heap.push(Cell {
            height: height_map[i][cols - 1],
            row: i,
            col: cols - 1,
        });
        visited[i][0] = true;
        visited[i][cols - 1] = true;
    }
    for j in 1..cols - 1 {
        heap.push(Cell {
            height: height_map[0][j],
            row: 0,
            col: j,
        });
        heap.push(Cell {
            height: height_map[rows - 1][j],
            row: rows - 1,
            col: j,
        });
        visited[0][j] = true;
        visited[rows - 1][j] = true;
    }

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut water = 0;

    // Process cells from lowest to highest
    while let Some(cell) = heap.pop() {
        for (dx, dy) in directions.iter() {
            let new_row = cell.row as i32 + dx;
            let new_col = cell.col as i32 + dy;

            if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if !visited[new_row][new_col] {
                    visited[new_row][new_col] = true;

                    // Calculate water trapped in this cell
                    if cell.height > height_map[new_row][new_col] {
                        water += cell.height - height_map[new_row][new_col];
                    }

                    // Add cell to heap with maximum of its height and water level
                    heap.push(Cell {
                        height: height_map[new_row][new_col].max(cell.height),
                        row: new_row,
                        col: new_col,
                    });
                }
            }
        }
    }

    water
}

fn main() {
    println!("LeetCode problem 407: Trapping Rain Water II");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap_rain_water() {
        assert_eq!(
            trap_rain_water(vec![
                vec![1, 4, 3, 1, 3, 2],
                vec![3, 2, 1, 3, 2, 4],
                vec![2, 3, 3, 2, 3, 1]
            ]),
            4
        );

        assert_eq!(
            trap_rain_water(vec![
                vec![3, 3, 3, 3, 3],
                vec![3, 2, 2, 2, 3],
                vec![3, 2, 1, 2, 3],
                vec![3, 2, 2, 2, 3],
                vec![3, 3, 3, 3, 3]
            ]),
            10
        );
    }
}
