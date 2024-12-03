# Advent of Code 2024 Solutions

This repository contains Rust-based solutions to the [2024 Advent of Code](https://adventofcode.com/2024/about) 
challenge.

To run a solution to a puzzle, use the just recipe. This assumes that you have the `just` command-runner installed
and your input data is located in a file using the pattern `day??.txt` saved in the `.\data` directory. These files
are excluded from the repository per Advent of Code [rules](https://adventofcode.com/2024/about). The data files for
the puzzles from the puzzle text has been left in the repository for unit test purposes.

```shell
just day01part1
```

Receipts are also provided to run the solution against the example puzzle data given in each puzzle description:

```shell
just day01part1example
```

The data provided to me for my puzzles can be found in `./data`. The answer to a given puzzle can be found in
the top source code comments for a given solution. 

Common data types, helper functions, etc., are in a [common library](lib/src/lib.rs).

* Day One: Historian Hysteria
    * [Part One](day01part1/README.md) ([source](day01part1/src/main.rs))
    * [Part Two](day01part1/README.md) ([source](day01part2/src/main.rs))
* Day Two: Red-Nosed Reports
    * [Part One](day02part1/README.md) ([source](day02part1/src/main.rs))
    * [Part Two](day02part2/README.md) ([source](day02part2/src/main.rs))
