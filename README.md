

# rwc

A simple `wc` (word count) clone written in Rust for Windows.

This is one of the projects in my month-long Rust learning project, where I am building small Unix-style command-line utilities for Windows from scratch.

## What is rwc?

`rwc` is a small command-line program that reads a file and counts:

- Lines
- Words
- Bytes

## Usage

Run the program with:

```cmd
cargo run -- <file>
```

 For example:

```
cargo run -- test.txt
```

 You can also run the compiled executable:

```
rwc.exe test.txt
```

 ## Example

 Given a file named `test.txt`:

```
hello world
this is rust
```

 Run:

```
cargo run -- test.txt
```

 Example output:

```
lines:2 words:5 bytes:25 filename:test.txt
```

 ## Building

 To build the project:

```
cargo build
```

 For a release build:

```
cargo build --release
```

 The release executable will be created at:

```
target\release\rwc.exe
```

 You can run it with:

```
target\release\rwc.exe test.txt
```

 ## Project Structure

```
rwc/
├── src/
│   └── main.rs
├── .gitignore
├── Cargo.toml
├── Cargo.lock
└── README.md
```

 ## Features

 Current features:

 - [x] Read a file
- [x] Count lines
- [x] Count words
- [x] Count bytes
- [x] Display the filename
- [x] Basic error handling
- [ ] `-l` line option
- [ ] `-w` word option
- [ ] `-c` byte option
- [ ] Multiple input files
- [ ] Standard input
- [ ] More complete compatibility with `wc`

 ## How It Works

 The program takes a filename from the command line.

 It opens the file using Rust's file I/O APIs and reads it using a buffered reader.

 For every line, the program:

 1. Counts the line.
2. Splits the line using whitespace and counts the words.
3. Counts the bytes in the line.
4. Adds the line-ending byte to the byte count.

 After reading the file, the program prints the results.

 ## Learning Goals

 I am using this project to learn and practice:

 - Rust
- File I/O
- `BufReader`
- Strings and bytes
- Iterators
- Command-line arguments
- Error handling
- Windows command-line programs
- Git
- GitHub
- Basic systems programming

 ## Why I Built This

 Instead of only following tutorials, I am building small programs from scratch to understand how common command-line utilities work internally.

 `rwc` is the first utility in my Unix utilities for Windows project.

 ## Project Status

 This project is currently under development.

 The implementation is intentionally simple and will be improved as I learn more Rust and systems programming.

 ## Future Projects

 After `rwc`, I plan to build more small utilities, including:

 - `grep` clone
- `cat` clone
- `find` clone
- JSON parser
- CSV parser
- Configuration file parser
- Mini allocator
- Arena allocator
- CLI argument parser

 The goal is to build these projects throughout my month-long Rust learning journey.

 ## License

 MIT License



 