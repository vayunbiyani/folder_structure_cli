# Folder Structure CLI

A simple and efficient CLI tool built with Rust to recursively list the entire folder and file structure of a specified directory. This tool is optimized for quick repository exploration, identifying key entry points, and analyzing project structures.

## Features

- Recursively lists all files and folders within a given directory.
- Skips files and directories specified in `.gitignore`, if present.
- Outputs an indented structure suitable for easy inspection or pasting into GPT for analysis and suggestions.
- Option to export the folder structure as a plain text output.

## Installation

### Option 1: Install via Cargo (if available on crates.io)

You can install this tool directly from [crates.io](https://crates.io/) by running:

```bash
cargo install folder_structure_cli
```
### Option 2: Build from Source

1. Clone the repository:
    ```bash
    git clone https://github.com/yourusername/folder_structure_cli.git
    cd folder_structure_cli
    ```

2. Build the project in release mode:
    ```bash
    cargo build --release
    ```

3. The binary will be located in `target/release`. You can run it with:
    ```bash
    ./target/release/folder_structure_cli <path_to_directory>
    ```

4. To list the structure of a directory, run the command with the path to the directory as an argument:
    ```bash
    ./target/release/folder_structure_cli <path_to_directory>
    ```

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request with any improvements or bug fixes.

1. Fork the repository.
2. Create your feature branch:
    ```bash
    git checkout -b feature/my-feature
    ```
3. Commit your changes:
    ```bash
    git commit -m 'Add some feature'
    ```
4. Push to the branch:
    ```bash
    git push origin feature/my-feature
    ```
5. Open a pull request.
