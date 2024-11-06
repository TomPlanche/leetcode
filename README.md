# LeetCode Solutions in Rust 🦀.
This repository contains my solutions to LeetCode problems implemented in Rust.

## Stats

- Total problems solved: 42
- Easy: 7 (16.7%)
- Medium: 26 (61.9%)
- Hard: 9 (21.4%)

## Automated Documentation

This repository uses GitHub Actions to automatically maintain an up-to-date listing of all solutions. The automation:

1. Triggers whenever a new solution is pushed to the main branch
2. Scans all solution directories (those starting with `id_`)
3. Extracts problem metadata from the source files, including:
    - Problem ID
    - Title
    - Difficulty level
    - Topic tags
4. Generates a formatted table of all solutions
5. Updates statistics about problem difficulties
6. Automatically commits and pushes the updated README.md

The automation script is written in Rust and can be found in `.github/scripts/update_readme/`.

Each solution file must include a documentation header in this format:
```rust
///
/// # Problem Title (Difficulty) [Tag1, Tag2]
/// LeetCode Problem {id}
///
```
| ID | Title | Difficulty | Tags |
|----|-------|------------|------|
| [7](./id_7) | Reverse Integer | Medium | `Math` |
| [9](./id_9) | Palindrome Number | Easy | `Math` |
| [37](./id_37) | Sudoku Solver | Hard | `Array`, `Hash Table`, `Backtracking`, `Matrix` |
| [239](./id_239) | Sliding Window Maximum | Hard | `Array`, `Queue`, `Sliding Window`, `Heap (Priority Queue)`, `Monotonic Queue` |
| [273](./id_273) | Integer to English Words | Hard | `Math`, `String`, `Recursion` |
| [567](./id_567) | Permutation in String | Medium | `Hash Table`, `Two Pointers`, `String`, `Sliding Window` |
| [670](./id_670) | Maximum Swap | Medium | `Math`, `Greedy` |
| [729](./id_729) | My Calendar I | Medium | `Array`, `Binary Search`, `Design`, `Segment Tree`, `Ordered Set` |
| [796](./id_796) | Rotate String | Easy | `String`, `String Matching` |
| [921](./id_921) | Minimum Add to Make Parentheses Valid | Medium | `String`, `Stack`, `Greedy` |
| [951](./id_951) | Minimum Add to Make Parentheses Valid | Medium | `String`, `Stack`, `Greedy` |
| [962](./id_962) | Maximum Width Ramp | Medium | `Array`, `Stack`, `Monotonic Stack` |
| [1106](./id_1106) | Parsing A Boolean Expression | Hard | `String`, `Stack`, `Recursion` |
| [1233](./id_1233) | Remove Sub-Folders from the Filesystem | Medium | `Array`, `String`, `Depth-First Search`, `Trie` |
| [1277](./id_1277) | Count Square Submatrices with All Ones | Medium | `Array`, `Dynamic Programming`, `Matrix` |
| [1331](./id_1331) | Rank Transform of an Array | Easy | `Array`, `Hash Table`, `Sorting` |
| [1405](./id_1405) | Longest Happy String | Medium | `String`, `Greedy`, `Heap (Priority Queue)` |
| [1497](./id_1497) | Check If Array Pairs Are Divisible by k | Medium | `Array`, `Hash Table`, `Counting` |
| [1545](./id_1545) | Find Kth Bit in Nth Binary String | Medium | `String`, `Recursion`, `Simulation` |
| [1590](./id_1590) | Make Sum Divisible by P | Medium | `Array`, `Hash Table`, `Prefix Sum` |
| [1593](./id_1593) | Split a String Into the Max Number of Unique Substrings | Medium | `Hash Table`, `String`, `Backtracking` |
| [1671](./id_1671) | Minimum Number of Removals to Make Mountain Array | Hard | `Array`, `Binary Search`, `Dynamic Programming`, `Greedy` |
| [1942](./id_1942) | The Number of the Smallest Unoccupied Chair | Medium | `Array`, `Hash Table`, `Heap (Priority Queue)` |
| [1957](./id_1957) | Delete Characters to Make Fancy String | Easy | `String` |
| [1963](./id_1963) | Minimum Number of Swaps to Make the String Balanced | Medium | `Two Pointers`, `String`, `Stack`, `Greedy` |
| [2044](./id_2044) | Count Number of Maximum Bitwise-OR Subsets | Medium | `Array`, `Backtracking`, `Bit Manipulation`, `Enumeration` |
| [2222](./id_2222) | Number of Ways to Select Buildings | Medium | `String`, `Dynamic Programming`, `Prefix Sum` |
| [2416](./id_2416) | Sum of Prefix Scores of Strings | Hard | `Array`, `String`, `Trie`, `Counting` |
| [2423](./id_2423) | Remove Letter To Equalize Frequency | Easy | `Hash Table`, `String`, `Counting` |
| [2458](./id_2458) | Height of Binary Tree After Subtree Removal Queries | Hard | `Array`, `Tree`, `Depth-First Search`, `Breadth-First Search`, `Binary Tree` |
| [2463](./id_2463) | Minimum Total Distance Traveled | Hard | `Array`, `Dynamic Programming`, `Sorting` |
| [2490](./id_2490) | Circular Sentence | Easy | `String` |
| [2501](./id_2501) | Longest Square Streak in an Array | Medium | `Array`, `Hash Table`, `Binary Search`, `Dynamic Programming`, `Sorting` |
| [2583](./id_2583) | Kth Largest Sum in a Binary Tree | Medium | `Tree`, `Breadth-First Search`, `Sorting`, `Binary Tree` |
| [2641](./id_2641) | Cousins in Binary Tree II | Medium | `Hash Table`, `Tree`, `Depth-First Search`, `Breadth-First Search`, `Binary Tree` |
| [2684](./id_2684) | Maximum Number of Moves in a Grid | Medium | `Array`, `Dynamic Programming`, `Matrix` |
| [2696](./id_2696) | Minimum String Length After Removing Substrings | Easy | `String`, `Stack`, `Simulation` |
| [2938](./id_2938) | Separate Black and White Balls | Medium | `Two Pointers`, `String`, `Greedy` |
| [3011](./id_3011) | Find if Array Can Be Sorted | Medium | `Array`, `Bit Manipulation`, `Sorting` |
| [3017](./id_3017) | Count the Number of Houses at a Certain Distance II | Hard | `Graph`, `Prefix Sum` |
| [3043](./id_3043) | Find the Length of the Longest Common Prefix | Medium | `Array`, `Hash Table`, `String`, `Trie` |
| [3163](./id_3163) | String Compression III | Medium | `String` |

## Tools

- [LeetCode CLI](./leetcode_cli/): A command-line tool to create new LeetCode problem projects.

## License

This project is open-source and available under the MIT License.
