# Intro
This project implements a basic pattern detector as a practice. The aim of the project is to get familiar with Rust programing language, by trying to implement a solution to a non-trivial problem.

# Problem statement
The goal is to write a struct to implement basic pattern matching. The struct will be initialized using an input pattern and `find_matches` method will be called with a string input, which has to output all matches in the string **in order**, based on input pattern. The pattern can be any combination of the form: `[c1,c2,....,cx]{n,m}` or `[c1-c2]{n,m}`, where c<sub>x</sub> can be any alphanumeric character or symbol, excluding `[`, `]`, `,`, `-`, `{` and `}`, n & m are numbers.  `[c1,c2,....,cx]{n,m}` means any character in set c1, c2,..., cx repeated any number of times between n & m (inclusive), while `[c1-c2]{n,m}` means any character in (ascii) range c1 & c2 (inclusive) repeated any number of times between n & m (inclusive). The input pattern can have these patterns repeated any number of times. For example, all of these are valid patterns:
- `[a,z,x]{3,4}`
- `[a-z]{5,6}`
- `[A-Z]{1,4}[1,2,3]{1,1}[a-z]{2,2}`
- `[x]{1,1}`
- `[x-z]{2,2}[$]{0,4}`

The pattern will always be in this form, always valid, and won't have spaces (unless space is one of the character in range or set).

In case there are multiple pattern matches starting at a position in the string, we have to output the largest match. For example, for pattern `[a,b,c]{3,4}` and input string `abca`, both initial `abc` and while `abca` matches, but `abca` is larger so we will output `abca`.

In case there are multiple overlapping pattern matches in a substring, output the one which starts first. This is similar to pattern matching in regex. For example, for pattern `[a-d]{2,3}[d-e]{1,1}`, and input string `abdde`, all `abd`, `abdd` and `bdde` match but overlap, out of which both `abdd` and `bdde` are 4 character long. But `abdd` starts first, so we will output only `abdd`.

The input and pattern is always expected to be a valid ascii character, we don't have to consider unicode.

# Code structure
The base code skeleton is provided. To solve the problem, we have to update `pattern.rs` file by adding fields in `Pattern` struct and updating associated functions `new` and `find_matches` in `impl Pattern`. The main function is already implemented for manual testing, so we can run it and input any pattern and strings for testing. Unit tests are provided in `pattern.rs` file. To complete the task, implement the functions mentioned above to make the tests pass. To run the tests, just run command `cargo test --bin pattern` in the directory.