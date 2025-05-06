//!
//! # Minimum Cost For Tickets (Medium) [Array, Dynamic Programming]
//! LeetCode Problem 983
//!

/// # `mincost_tickets`
/// Given a list of travel days and costs for different types of passes,
/// find the minimum cost to cover all travel days using a combination of:
/// * 1-day pass (costs[0])
/// * 7-day pass (costs[1])
/// * 30-day pass (costs[2])
///
/// # Approach
/// Uses dynamic programming to build up the solution:
/// 1. Create a DP array where dp[i] represents minimum cost up to day i
/// 2. For each travel day, consider buying each type of pass and take minimum
/// 3. For non-travel days, cost remains same as previous day
///
/// # Arguments
/// * `days` - A vector of integers representing travel days (1 to 365, strictly increasing)
/// * `costs` - A vector of 3 integers representing costs for 1-day, 7-day, and 30-day passes
///
/// # Returns
/// * `i32` - The minimum cost in dollars needed to cover all travel days
pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
    let last_day = *days.last().unwrap() as usize;
    let mut dp = vec![0; last_day + 1];
    let days_set: std::collections::HashSet<i32> = days.into_iter().collect();

    // Start from day 1 since days[i] >= 1
    for day in 1..=last_day {
        if !days_set.contains(&(day as i32)) {
            // If not a travel day, cost remains same as previous day
            dp[day] = dp[day - 1];
        } else {
            // Consider all three pass options and take minimum
            let one_day = dp[day - 1] + costs[0];
            let seven_day = dp[day.saturating_sub(7)] + costs[1];
            let thirty_day = dp[day.saturating_sub(30)] + costs[2];

            dp[day] = one_day.min(seven_day).min(thirty_day);
        }
    }

    dp[last_day]
}

fn main() {
    println!("LeetCode problem 983: Minimum Cost For Tickets");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_cases() {
        // Test case 1 from problem description
        assert_eq!(mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]), 11);

        // Test case 2 from problem description
        assert_eq!(
            mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15]),
            17
        );
    }

    #[test]
    fn test_edge_cases() {
        // Single day
        assert_eq!(mincost_tickets(vec![1], vec![2, 7, 15]), 2);

        // Consecutive days where 7-day pass is optimal
        assert_eq!(
            mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7], vec![10, 7, 15]),
            7
        );

        // Sparse days where individual passes are optimal
        assert_eq!(
            mincost_tickets(vec![1, 4, 7, 10, 13, 16, 19], vec![2, 7, 15]),
            14
        );
    }
}
