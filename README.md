# tkrar

A fast and feature-rich CLI tool written in Rust to count frequency of words in a file or a directory.

**Name origin:**  
The name **tkrar** (تكرار) comes from the Arabic word for *repetition* or *frequency*.  
It's pronounced like: **tek-raar** (with a rolled 'r').

## Features

- Count frequency of words in a file or a directory recursively
- Process input from stdin
- Supports case sensitivity
- ignore stopwords
- ignoring words with a minimum character count
- ignoring words with a regex pattern
- ignoring non-alphanumeric characters
- ignoring provided files path
- Supports outputting results in JSON or CSV format
- Pretty-print the results (but not when outputting to TTY)

## Installation

```sh
cargo install tkrar
```

## Usage

```sh
tkrar [OPTIONS] [TARGET]...
```

### Flags

- `-c`, `--case-sensitive`: case sensitivity when counting words
- `--no-stopwords`: ignore stopwords when counting words
- `--alphabetic-only`: ignore non-alphanumeric characters
- `-h`, `--help`: print help
- `-V`, `--version`: print version

### Options

- `-t`, `--top <N>`: show the N most frequent words
- `-m`, `--min-char <N>`: ignore words with less than N characters
- `-s`, `--sort <SORT>`: sort order (default: desc) (asc or desc)
- `-i`, `--ignore-words <REGEX>`: ignore words that match the provided regex pattern
- `-I`, `--ignore-files <FILE>`: ignore provided files path
- `-o`, `--output-format <FORMAT>`: output with the specified format (default: text) (text, json, csv)


### Arguments

- `[TARGET]...`: path to the multiple target files or directories (default: stdin)

## Examples

```sh
# Count frequency of words in a file
tkrar ./path/to/target

# Count frequency of words from stdin
echo "Hello, world!" | tkrar

# Count frequency of words from multiple files and directories
tkrar ./path/to/file1.txt ./path/to/directory ./path/to/another/directory

# Ignore stopwords
tkrar --no-stopwords ./path/to/target

# Ignore words with provided regex patterns
tkrar --ignore-words "the|and|is|in|to" ./path/to/target

# Ignore provided files path
tkrar --ignore-files "./path/to/file1.txt,./path/to/file2.txt" ./path/to/target

# Ignore non-alphanumeric characters
tkrar --alphabetic-only ./path/to/target

# Output results in JSON or CSV format
tkrar --output-format json ./path/to/target

# Sort order (asc or desc)
tkrar --sort asc ./path/to/target

# Show the N most frequent words
tkrar --top 10 ./path/to/target
```
