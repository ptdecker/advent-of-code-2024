lint:
    cargo fmt
    cargo clippy

# Day 01, Part 1: Example Input Solution
day01part1example: lint
    cargo run --bin day01part1 < data/example01.txt

# Day 01, Part 1: Puzzle Input Solution
day01part1: lint
    cargo run --bin day01part1 < data/day01.txt

# Day 01, Part 2: Example Input Solution
day01part2example: lint
    cargo run --bin day01part2 < data/example01.txt

# Day 01, Part 2: Puzzle Input Solution
day01part2: lint
    cargo run --bin day01part2 < data/day01.txt