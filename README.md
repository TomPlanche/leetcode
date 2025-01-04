# LeetCode Solutions in Rust 🦀

> A collection of LeetCode problem solutions implemented in Rust, with a focus on clean code and detailed explanations.

<div
    style="display: flex; justify-content: center; align-items: center;"
>
    <img
        src="https://img.shields.io/badge/Solutions-109-green" width="7.5%"
        alt="Solutions"
    />
    <img
        src="https://img.shields.io/github/languages/top/tomPlanche/leetcode" width="7.5%" al
        ="Top Language"
    />
    <img
        src="https://img.shields.io/badge/license-MIT-blue" width="7.5%"
        alt="License"
    />
    <br />
    <img
        src="https://assets.leetcode.com/static_assets/marketing/2024-50-lg.png"
        alt="Leetcode 50 days badge"
        width="7.5%"
    />
    <img
        src="https://leetcode.com/static/images/badges/dcc-2024-11.png"
        alt="Leetcode November badge"
        width="7.5%"
    />
</div>

## 📊 Progress

- **Total Solved**: 109 problems
    - **Easy**: 16 (14.7%) 🟢
    - **Medium**: 73 (67.0%) 🟡
    - **Hard**: 20 (18.3%) 🔴

## 🌟 Featured Solutions

Here are some noteworthy solutions with interesting approaches:

1. [Regular Expression Matching](./id_10) (Hard)
    - Uses dynamic programming
    - Complex pattern matching implementation
    - 98th percentile runtime performance

2. [Sliding Window Maximum](./id_239) (Hard)
    - Implements a monotonic queue
    - O(n) time complexity solution
    - Detailed explanation of the algorithm

3. [Two Sum](./id_1) (Easy)
    - Classic hash table approach
    - Optimal space-time trade-off
    - Perfect for beginners

## 📁 Repository Structure

```text
leetcode/
├── problems/
│   └── id_*/ (Solution directories)
│       ├── src/
│       │   └── main.rs (Solution implementation)
│       └── Cargo.toml
├── leetcode_cli/ (Project creation tool)
└── update_readme/ (README update script)
```

## 🛠️ Tools & Utilities

### LeetCode CLI

A command-line tool for quickly creating new solution projects:

```bash
new_leetcode 1234 --difficulty Medium --tags "array,dp" --title "Problem Title"
```

[Learn more about LeetCode CLI](./leetcode_cli)

### Automatic README Updates

This repository uses a pre-commit hook to automatically update the README.md file whenever changes are committed. The system includes:

1. **update_readme** - A Rust script that:
   - Scans the problems directory
   - Extracts solution metadata
   - Updates statistics
   - Generates the solutions table
   - Updates the README.md file

2. **Pre-commit Hook Setup**
    - Add the following script to `.git/hooks/pre-commit`:
    ```bash
    #!/usr/bin/env bash
    # run the `./update_readme/target/release/update_readme` binary to update the README.md
    # when files in `./problems/id_*` are changed.

    # get the list of files that have been changed
    # since the last commit
    files=$(git diff --cached --name-only)

    # check if any of the files are in the `./problems/id_*` directory
    if [[ $files == *"problems/id_"* ]]; then
        # if so, run the `update_readme` binary
        ./update_readme/target/release/update_readme
        # add the changes to the commit
        git add README.md
    fi

    # continue with the commit
    exit 0
    ```

    Make sure to give the script execution permissions:
    ```bash
    chmod +x .git/hooks/pre-commit
    ```

The script will run automatically before each commit, ensuring the README is always up to date with:
- Current solution count and statistics
- Complete solutions table
- Difficulty distribution
## 📝 Solution Format

Each solution includes this header format for easy navigation and reference (and for the readme generator to parse):

```rust
///
/// # Problem Title (Difficulty) [Tag1, Tag2]
/// LeetCode Problem {id}
///
```

## 📋 Solutions Table

| ID | Title | Difficulty | Tags |
|----|-------|------------|------|
| [1](./problems/id_1) | Two sums | Easy | `Array`, `Hash Table` |
| [7](./problems/id_7) | Reverse Integer | Medium | `Math` |
| [9](./problems/id_9) | Palindrome Number | Easy | `Math` |
| [10](./problems/id_10) | Regular Expression Matching | Hard | `String`, `Dynamic Programming`, `Recursion` |
| [12](./problems/id_12) | Integer to Roman | Medium | `Hash Table`, `Math`, `String` |
| [20](./problems/id_20) | Valid Parentheses | Easy | `String`, `Stack` |
| [22](./problems/id_22) | Generate Parentheses | Medium | `String`, `Dynamic Programming`, `Backtracking` |
| [29](./problems/id_29) | Divide Two Integers | Medium | `Math`, `Bit Manipulation` |
| [37](./problems/id_37) | Sudoku Solver | Hard | `Array`, `Hash Table`, `Backtracking`, `Matrix` |
| [239](./problems/id_239) | Sliding Window Maximum | Hard | `Array`, `Queue`, `Sliding Window`, `Heap (Priority Queue)`, `Monotonic Queue` |
| [273](./problems/id_273) | Integer to English Words | Hard | `Math`, `String`, `Recursion` |
| [494](./problems/id_494) | Target Sum | Medium | `Array`, `Dynamic Programming`, `Backtracking` |
| [515](./problems/id_515) | Find Largest Value in Each Tree Row | Medium | `Tree`, `Depth First Search`, `Breadth First Search`, `Binary Tree` |
| [567](./problems/id_567) | Permutation in String | Medium | `Hash Table`, `Two Pointers`, `String`, `Sliding Window` |
| [670](./problems/id_670) | Maximum Swap | Medium | `Math`, `Greedy` |
| [689](./problems/id_689) | Maximum Sum of 3 Non-Overlapping Subarrays | Hard | `Array`, `Dynamic Programming` |
| [729](./problems/id_729) | My Calendar I | Medium | `Array`, `Binary Search`, `Design`, `Segment Tree`, `Ordered Set` |
| [769](./problems/id_769) | Max Chunks To Make Sorted | Medium | `Array`, `Stack`, `Greedy`, `Sorting`, `Monotonic Stack` |
| [773](./problems/id_773) | Sliding Puzzle | Hard | `Array`, `Breadth First Search`, `Matrix` |
| [796](./problems/id_796) | Rotate String | Easy | `String`, `String Matching` |
| [862](./problems/id_862) | Shortest Subarray with Sum at Least K | Hard | `Array`, `Binary Search`, `Queue`, `Sliding Window`, `Heap (Priority Queue)`, `Prefix Sum`, `Monotonic Queue` |
| [921](./problems/id_921) | Minimum Add to Make Parentheses Valid | Medium | `String`, `Stack`, `Greedy` |
| [951](./problems/id_951) | Minimum Add to Make Parentheses Valid | Medium | `String`, `Stack`, `Greedy` |
| [962](./problems/id_962) | Maximum Width Ramp | Medium | `Array`, `Stack`, `Monotonic Stack` |
| [983](./problems/id_983) | Minimum Cost For Tickets | Medium | `Array`, `Dynamic Programming` |
| [1014](./problems/id_1014) | Best Sightseeing Pair | Medium | `Array`, `Dynamic Programming` |
| [1072](./problems/id_1072) | Flip Columns For Maximum Number of Equal Rows | Medium | `Array`, `Hash Table`, `Matrix` |
| [1106](./problems/id_1106) | Parsing A Boolean Expression | Hard | `String`, `Stack`, `Recursion` |
| [1233](./problems/id_1233) | Remove Sub-Folders from the Filesystem | Medium | `Array`, `String`, `Depth-First Search`, `Trie` |
| [1277](./problems/id_1277) | Count Square Submatrices with All Ones | Medium | `Array`, `Dynamic Programming`, `Matrix` |
| [1331](./problems/id_1331) | Rank Transform of an Array | Easy | `Array`, `Hash Table`, `Sorting` |
| [1346](./problems/id_1346) | Check If N and Its Double Exist | Easy | `Array`, `Hash Table`, `Two Pointers`, `Binary Search`, `Sorting` |
| [1405](./problems/id_1405) | Longest Happy String | Medium | `String`, `Greedy`, `Heap (Priority Queue)` |
| [1422](./problems/id_1422) | Maximum Score After Splitting a String | Easy | `String`, `Prefix Sum` |
| [1455](./problems/id_1455) | Check If a Word Occurs As a Prefix of Any Word in a Sentence | Easy | `Two Pointers`, `String`, `String Matching` |
| [1475](./problems/id_1475) | Final Prices With a Special Discount in a Shop | Easy | `Array`, `Stack`, `Monotonic Stack` |
| [1497](./problems/id_1497) | Check If Array Pairs Are Divisible by k | Medium | `Array`, `Hash Table`, `Counting` |
| [1545](./problems/id_1545) | Find Kth Bit in Nth Binary String | Medium | `String`, `Recursion`, `Simulation` |
| [1574](./problems/id_1574) | Shortest Subarray to be Removed to Make Array Sorted | Medium | `Array`, `Two Pointers`, `Binary Search`, `Stack`, `Monotonic Stack` |
| [1590](./problems/id_1590) | Make Sum Divisible by P | Medium | `Array`, `Hash Table`, `Prefix Sum` |
| [1593](./problems/id_1593) | Split a String Into the Max Number of Unique Substrings | Medium | `Hash Table`, `String`, `Backtracking` |
| [1639](./problems/id_1639) | Number of Ways to Form a Target String Given a Dictionary | Hard | `Array`, `String`, `Dynamic Programming` |
| [1652](./problems/id_1652) | Defuse the Bomb | Easy | `Array`, `Sliding Window` |
| [1671](./problems/id_1671) | Minimum Number of Removals to Make Mountain Array | Hard | `Array`, `Binary Search`, `Dynamic Programming`, `Greedy` |
| [1760](./problems/id_1760) | Minimum Limit of Balls in a Bag | Medium | `Array`, `Binary Search` |
| [1792](./problems/id_1792) | Maximum Average Pass Ratio | Medium | `Array`, `Greedy`, `Heap (Priority Queue)` |
| [1813](./problems/id_1813) | Sentence Similarity III | Medium | `Array`, `Two Pointers`, `String` |
| [1829](./problems/id_1829) | Maximum XOR for Each Query | Medium | `Array`, `Bit Manipulation`, `Prefix Sum` |
| [1861](./problems/id_1861) | Rotating the Box | Medium | `Array`, `Two Pointers`, `Matrix` |
| [1930](./problems/id_1930) | Unique Length-3 Palindromic Subsequences | Medium | `Hash Table`, `String`, `Bit Manipulation`, `Prefix Sum` |
| [1942](./problems/id_1942) | The Number of the Smallest Unoccupied Chair | Medium | `Array`, `Hash Table`, `Heap (Priority Queue)` |
| [1957](./problems/id_1957) | Delete Characters to Make Fancy String | Easy | `String` |
| [1963](./problems/id_1963) | Minimum Number of Swaps to Make the String Balanced | Medium | `Two Pointers`, `String`, `Stack`, `Greedy` |
| [1975](./problems/id_1975) | Maximum Matrix Sum | Medium | `Array`, `Greedy`, `Matrix` |
| [2044](./problems/id_2044) | Count Number of Maximum Bitwise-OR Subsets | Medium | `Array`, `Backtracking`, `Bit Manipulation`, `Enumeration` |
| [2054](./problems/id_2054) | Two Best Non-Overlapping Events | Medium | `Array`, `Binary Search`, `Dynamic Programming`, `Sorting`, `Heap (priority Queue)` |
| [2064](./problems/id_2064) | Minimized Maximum of Products Distributed to Any Store | Medium | `Array`, `Binary Search` |
| [2070](./problems/id_2070) | Most Beautiful Item for Each Query | Medium | `Array`, `Binary Search`, `Sorting` |
| [2097](./problems/id_2097) | Valid Arrangement of Pairs | Hard | `Depth First Search`, `Graph`, `Eulerian Circuit` |
| [2109](./problems/id_2109) | Adding Spaces to a String | Medium | `Array`, `Two Pointers`, `String`, `Simulation` |
| [2182](./problems/id_2182) | Construct String With Repeat Limit | Medium | `Hash Table`, `String`, `Greedy`, `Heap (priority Queue)`, `Counting` |
| [2222](./problems/id_2222) | Number of Ways to Select Buildings | Medium | `String`, `Dynamic Programming`, `Prefix Sum` |
| [2257](./problems/id_2257) | Count Unguarded Cells in the Grid | Medium | `Array`, `Matrix`, `Simulation` |
| [2270](./problems/id_2270) | Number of Ways to Split Array | Medium | `Array`, `Prefix Sum` |
| [2275](./problems/id_2275) | Largest Combination With Bitwise AND Greater Than Zero | Medium | `Array`, `Hash Table`, `Bit Manipulation`, `Counting` |
| [2290](./problems/id_2290) | Minimum Obstacle Removal to Reach Corner | Hard | `Array`, `Breadth-First Search`, `Graph`, `Heap (Priority Queue)`, `Matrix`, `Shortest Path` |
| [2335](./problems/id_2335) | Moving Pieces to Obtain a String | Medium | `String`, `Two Pointers` |
| [2415](./problems/id_2415) | Reverse Odd Levels of Binary Tree | Medium | `Tree`, `Depth First Search`, `Breadth First Search`, `Binary Tree` |
| [2416](./problems/id_2416) | Sum of Prefix Scores of Strings | Hard | `Array`, `String`, `Trie`, `Counting` |
| [2423](./problems/id_2423) | Remove Letter To Equalize Frequency | Easy | `Hash Table`, `String`, `Counting` |
| [2458](./problems/id_2458) | Height of Binary Tree After Subtree Removal Queries | Hard | `Array`, `Tree`, `Depth-First Search`, `Breadth-First Search`, `Binary Tree` |
| [2461](./problems/id_2461) | Maximum Sum of Distinct Subarrays With Length K | Medium | `Array`, `Hash Table`, `Sliding Window` |
| [2463](./problems/id_2463) | Minimum Total Distance Traveled | Hard | `Array`, `Dynamic Programming`, `Sorting` |
| [2466](./problems/id_2466) | Count Ways To Build Good Strings | Medium | `Dynamic Programming` |
| [2471](./problems/id_2471) | Minimum Number of Operations to Sort a Binary Tree by Level | Medium | `Tree`, `Breadth First Search`, `Binary Tree` |
| [2490](./problems/id_2490) | Circular Sentence | Easy | `String` |
| [2501](./problems/id_2501) | Longest Square Streak in an Array | Medium | `Array`, `Hash Table`, `Binary Search`, `Dynamic Programming`, `Sorting` |
| [2516](./problems/id_2516) | Take K of Each Character From Left and Right | Medium | `Hash Table`, `String`, `Sliding Window` |
| [2554](./problems/id_2554) | Maximum Number of Integers to Choose From a Range I | Medium | `Array`, `Hash Table`, `Binary Search`, `Greedy`, `Sorting` |
| [2558](./problems/id_2558) | Take Gifts From the Richest Pile | Easy | `Array`, `Heap (priority Queue)`, `Simulation` |
| [2559](./problems/id_2559) | Count Vowel Strings in Ranges | Medium | `Array`, `String`, `Prefix Sum` |
| [2563](./problems/id_2563) | Count the Number of Fair Pairs | Medium | `Array`, `Two Pointers`, `Binary Search`, `Sorting` |
| [2577](./problems/id_2577) | Minimum Time to Visit a Cell In a Grid | Hard | `Array`, `Breadth First Search`, `Graph`, `Heap (priority Queue)`, `Matrix`, `Shortest Path` |
| [2583](./problems/id_2583) | Kth Largest Sum in a Binary Tree | Medium | `Tree`, `Breadth-First Search`, `Sorting`, `Binary Tree` |
| [2593](./problems/id_2593) | Find Score of an Array After Marking All Elements | Medium | `Array`, `Hash Table`, `Sorting`, `Heap (priority Queue)`, `Simulation` |
| [2601](./problems/id_2601) | Prime Subtraction Operation | Medium | `Array`, `Math`, `Binary Search`, `Greedy`, `Number Theory` |
| [2641](./problems/id_2641) | Cousins in Binary Tree II | Medium | `Hash Table`, `Tree`, `Depth-First Search`, `Breadth-First Search`, `Binary Tree` |
| [2684](./problems/id_2684) | Maximum Number of Moves in a Grid | Medium | `Array`, `Dynamic Programming`, `Matrix` |
| [2696](./problems/id_2696) | Minimum String Length After Removing Substrings | Easy | `String`, `Stack`, `Simulation` |
| [2762](./problems/id_2762) | Continuous Subarrays | Medium | `Array`, `Queue`, `Sliding Window`, `Heap (priority Queue)`, `Ordered Set`, `Monotonic Queue` |
| [2779](./problems/id_2779) | Maximum Beauty of an Array After Applying Operation | Medium | `Array`, `Binary Search`, `Sliding Window`, `Sorting` |
| [2825](./problems/id_2825) | Make String a Subsequence Using Cyclic Increments | Medium | `Two Pointers`, `String` |
| [2872](./problems/id_2872) | Maximum Number of K-Divisible Components | Hard | `Tree`, `Depth First Search` |
| [2914](./problems/id_2914) | Minimum Changes to Make Binary String Beautiful | Medium | `String` |
| [2924](./problems/id_2924) | Find Champion II | Medium | `Graph` |
| [2938](./problems/id_2938) | Separate Black and White Balls | Medium | `Two Pointers`, `String`, `Greedy` |
| [2940](./problems/id_2940) | Find Building Where Alice and Bob Can Meet | Hard | `Array`, `Binary Search`, `Stack`, `Monotonic Stack` |
| [2981](./problems/id_2981) | Find Longest Special Substring That Occurs Thrice I | Medium | `Hash Table`, `String`, `Binary Search`, `Sliding Window`, `Counting` |
| [3011](./problems/id_3011) | Find if Array Can Be Sorted | Medium | `Array`, `Bit Manipulation`, `Sorting` |
| [3017](./problems/id_3017) | Count the Number of Houses at a Certain Distance II | Hard | `Graph`, `Prefix Sum` |
| [3043](./problems/id_3043) | Find the Length of the Longest Common Prefix | Medium | `Array`, `Hash Table`, `String`, `Trie` |
| [3090](./problems/id_3090) | Shortest Subarray With OR at Least K II | Medium | `Array`, `Bit Manipulation`, `Sliding Window` |
| [3133](./problems/id_3133) | Minimum Array End | Medium | `Bit Manipulation` |
| [3152](./problems/id_3152) | Special Array II | Medium | `Array`, `Binary Search`, `Prefix Sum` |
| [3163](./problems/id_3163) | String Compression III | Medium | `String` |
| [3203](./problems/id_3203) | Find Minimum Diameter After Merging Two Trees | Hard | `Tree`, `Depth First Search`, `Breadth First Search`, `Graph` |
| [3243](./problems/id_3243) | Shortest Distance After Road Addition Queries I | Medium | `Array`, `Breadth First Search`, `Graph` |
| [3254](./problems/id_3254) | Find the Power of K-Size Subarrays I | Medium | `Array`, `Sliding Window` |
| [3264](./problems/id_3264) | Final Array State After K Multiplication Operations I | Easy | `Array`, `Math`, `Heap (priority Queue)`, `Simulation` |

## 🚀 Getting Started

1. Clone the repository:
```bash
git clone https://github.com/tomPlanche/leetcode.git
```

2. Navigate to a solution:
```bash
cd problems/id_<problem_number>
```

3. Run the solution:
```bash
cargo run
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## 👤 Author

**Tom Planche**
- GitHub: [@tomPlanche](https://github.com/tomPlanche)
- LinkedIn: [Tom Planche](https://www.linkedin.com/in/tom-planche/)