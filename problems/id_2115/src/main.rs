//!
//! # Find All Possible Recipes from Given Supplies (Medium) [Array, Hash Table, String, Graph, Topological Sort]
//! LeetCode Problem 2115
//!
use std::collections::{HashMap, HashSet};

/// # `find_all_recipes`
/// Returns a list of all recipes that can be created given the available supplies and ingredients.
/// Uses a topological sort approach with cycle detection to handle recipe dependencies.
///
/// # Algorithm
/// 1. Convert supplies to HashSet for O(1) lookup
/// 2. Build graph representation of recipe dependencies
/// 3. Use DFS with state tracking to detect cycles and find makeable recipes
/// 4. Return all recipes that can be made
///
/// # Arguments
/// * `recipes` - Vector of recipe names
/// * `ingredients` - 2D vector where ingredients[i] contains ingredients needed for recipes[i]
/// * `supplies` - Vector of initially available ingredients
///
/// # Returns
/// * Vector of recipe names that can be created
pub fn find_all_recipes(
    recipes: Vec<String>,
    ingredients: Vec<Vec<String>>,
    supplies: Vec<String>,
) -> Vec<String> {
    // Convert supplies to HashSet for O(1) lookup
    let supplies: HashSet<String> = supplies.into_iter().collect();

    // Create recipe lookup set
    let recipe_set: HashSet<String> = recipes.iter().cloned().collect();

    // Build adjacency list representation
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for (recipe, ingr) in recipes.iter().zip(ingredients.iter()) {
        graph.insert(recipe.clone(), ingr.clone());
    }

    // Track visited states: 0 = not visited, 1 = in progress, 2 = completed
    let mut visited: HashMap<String, i32> = HashMap::new();
    // Track which recipes can be made
    let mut can_make: HashSet<String> = HashSet::new();

    // DFS helper function to check if recipe can be made
    fn can_make_recipe(
        recipe: &str,
        graph: &HashMap<String, Vec<String>>,
        supplies: &HashSet<String>,
        recipe_set: &HashSet<String>,
        visited: &mut HashMap<String, i32>,
        can_make: &mut HashSet<String>,
    ) -> bool {
        // Check visited state
        if let Some(&state) = visited.get(recipe) {
            if state == 1 {
                return false;
            } // Cycle detected
            if state == 2 {
                return can_make.contains(recipe);
            }
        }

        // Mark as in progress
        visited.insert(recipe.to_string(), 1);

        // Check if we can make this recipe
        if let Some(ingredients) = graph.get(recipe) {
            let can_make_all = ingredients.iter().all(|ing| {
                if supplies.contains(ing) {
                    true
                } else if recipe_set.contains(ing) {
                    can_make_recipe(ing, graph, supplies, recipe_set, visited, can_make)
                } else {
                    false
                }
            });

            if can_make_all {
                can_make.insert(recipe.to_string());
            }

            // Mark as completed
            visited.insert(recipe.to_string(), 2);
            return can_make_all;
        }

        false
    }

    // Try to make each recipe
    for recipe in &recipes {
        can_make_recipe(
            recipe,
            &graph,
            &supplies,
            &recipe_set,
            &mut visited,
            &mut can_make,
        );
    }

    // Return makeable recipes in original order
    recipes
        .into_iter()
        .filter(|r| can_make.contains(r))
        .collect()
}

fn main() {
    println!("LeetCode problem 2115")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_all_recipes() {
        assert_eq!(
            find_all_recipes(
                vec!["bread".to_string()],
                vec![vec!["yeast".to_string(), "flour".to_string()]],
                vec!["yeast".to_string(), "flour".to_string(), "corn".to_string()]
            ),
            vec!["bread"]
        );

        assert_eq!(
            find_all_recipes(
                vec!["bread".to_string(), "sandwich".to_string()],
                vec![
                    vec!["yeast".to_string(), "flour".to_string()],
                    vec!["bread".to_string(), "meat".to_string()]
                ],
                vec!["yeast".to_string(), "flour".to_string(), "meat".to_string()]
            ),
            vec!["bread", "sandwich"]
        );
    }
}
