import os
import re
from collections import defaultdict

def get_problem_info(file_path):
    with open(file_path, 'r') as f:
        content = f.read()

        # Extract difficulty and tags from comments
        difficulty_match = re.search(r'//\s*Difficulty:\s*(Easy|Medium|Hard)', content)
        tags_match = re.search(r'//\s*Tags:\s*(.+)', content)
        title_match = re.search(r'//\s*Title:\s*(.+)', content)

        difficulty = difficulty_match.group(1) if difficulty_match else "Unknown"
        tags = tags_match.group(1).split(',') if tags_match else []
        title = title_match.group(1) if title_match else "Unknown"

        return {
            'title': title.strip(),
            'difficulty': difficulty.strip(),
            'tags': [tag.strip() for tag in tags]
        }

def main():
    problems = []
    stats = defaultdict(int)

    # Get all problem directories
    for dir_name in os.listdir('.'):
        if dir_name.startswith('id_') and os.path.isdir(dir_name):
            problem_id = dir_name[3:]  # Remove 'id_' prefix
            main_rs_path = os.path.join(dir_name, 'src', 'main.rs')

            if os.path.exists(main_rs_path):
                info = get_problem_info(main_rs_path)
                problems.append({
                    'id': problem_id,
                    **info
                })
                stats[info['difficulty']] += 1

    # Sort problems by ID
    problems.sort(key=lambda x: int(x['id']))

    # Generate README content
    readme_content = """# LeetCode Solutions in Rust 🦀

This repository contains my solutions to LeetCode problems implemented in Rust.

## Problems

| ID | Title | Difficulty | Tags |
|----|-------|------------|------|
"""

    for problem in problems:
        tags_str = '`' + '`, `'.join(problem['tags']) + '`' if problem['tags'] else ''
        readme_content += f"| [{problem['id']}](./id_{problem['id']}/) | {problem['title']} | {problem['difficulty']} | {tags_str} |\n"

    readme_content += f"""
## Tools

- [LeetCode CLI](./leetcode_cli/): A command-line tool to create new LeetCode problem projects.

## Stats

- Total problems solved: {sum(stats.values())}
- Easy: {stats['Easy']}
- Medium: {stats['Medium']}
- Hard: {stats['Hard']}

## License

This project is open-source and available under the MIT License.
"""

    # Write to README.md
    with open('README.md', 'w') as f:
        f.write(readme_content)

if __name__ == '__main__':
    main()
