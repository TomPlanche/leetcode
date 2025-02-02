# Difficulty completions
complete -c new_leetcode -s d -l difficulty -x -a "Easy Medium Hard" -d "Set problem difficulty"

# Tags completion
complete -c new_leetcode -s t -l tags -x -d "Set problem tags (comma-separated)"

# Title completion
complete -c new_leetcode -s t -l title -x -d "Set problem title"

# Verbose flag
complete -c new_leetcode -s v -l verbose -d "Enable verbose output"

# Daily challenge flag
complete -c new_leetcode -l daily -d "Get today's daily challenge"
