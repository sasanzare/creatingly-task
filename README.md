# Log Word Analyzer

A Rust-based log analysis tool that identifies and ranks the most frequently occurring words in log files.

## Features

- Case-insensitive word counting
- Alphanumeric word processing (a-z, A-Z, 0-9)
- Sorting by frequency (descending) and alphabetically for ties
- Two different implementations:
  - CLI version: Reads input from log files
  - Static version: Processes predefined data

## Usage

### CLI Version (log_word_analyzer_cli)

```bash
cd log_word_analyzer_cli
cargo run -- logs.txt 2
```

### Static Version (log_word_analyzer_static)

```bash
cd log_word_analyzer_static
cargo run
```

## Example Output

```rust
[("error", 3), ("disk", 2)]
```

## Tests

The project includes comprehensive tests to ensure correct functionality:

```bash
cargo test
```

Implemented tests include:
- Basic functionality and case sensitivity
- Correct sorting order
- Alphanumeric word and special character processing
- Edge cases like empty input and k values larger than unique word count

## Requirements

- Rust 1.60 or higher
- Cargo

## Project Structure

```
/
├── log_word_analyzer_cli/
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml
│   └── logs.txt
├── log_word_analyzer_static/
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
└── README.md
```

## Algorithm Explanation

1. Convert all log lines to lowercase
2. Split lines into words using non-alphanumeric separators
3. Count word occurrences using HashMap
4. Sort by frequency (descending) and alphabetically for ties
5. Select top k words

## Implementation Details

The core function signature:
```rust
fn top_k_words(logs: &[String], k: usize) -> Vec<(String, usize)>
```

This function:
- Processes each log line by converting to lowercase
- Splits on non-alphanumeric characters
- Counts word frequencies using a HashMap
- Sorts results by frequency (descending) and alphabetically for ties
- Returns the top k results