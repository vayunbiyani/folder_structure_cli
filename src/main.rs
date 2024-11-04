use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashSet;
use std::io::{self, BufRead};

// Import `std::fmt::Write` for writing to a `String`
use std::fmt::Write;

// Main function to start the CLI tool
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }

    let root_path = Path::new(&args[1]);
    if !root_path.exists() || !root_path.is_dir() {
        eprintln!("Invalid path or not a directory: {}", root_path.display());
        std::process::exit(1);
    }

    // Load .gitignore paths if present
    let ignored_paths = load_gitignore(root_path)?;

    // Start listing the directory structure
    let mut output = String::new();
    list_directory(root_path, 0, &ignored_paths, &mut output)?;
    println!("{}", output);

    Ok(())
}

// Loads paths to ignore from .gitignore if it exists
fn load_gitignore(root: &Path) -> io::Result<HashSet<PathBuf>> {
    let mut ignored_paths = HashSet::new();
    let gitignore_path = root.join(".gitignore");
    
    if gitignore_path.exists() {
        let file = fs::File::open(gitignore_path)?;
        for line in io::BufReader::new(file).lines() {
            let line = line?;
            if !line.trim().is_empty() && !line.starts_with('#') {
                ignored_paths.insert(root.join(line.trim()));
            }
        }
    }
    Ok(ignored_paths)
}

// Recursive function to list files and folders with indentation
fn list_directory(
    path: &Path,
    level: usize,
    ignored_paths: &HashSet<PathBuf>,
    output: &mut String
) -> io::Result<()> {
    let indent = "  ".repeat(level);

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        
        if ignored_paths.contains(&entry_path) {
            continue; // Skip ignored paths
        }

        if entry_path.is_dir() {
            writeln_to_string(output, format!("{}- {}/\n", indent, entry.file_name().to_string_lossy()))?;
            list_directory(&entry_path, level + 1, ignored_paths, output)?;
        } else {
            writeln_to_string(output, format!("{}- {}\n", indent, entry.file_name().to_string_lossy()))?;
        }
    }

    Ok(())
}

// Helper function to handle the fmt::Error and convert it to io::Error
fn writeln_to_string(output: &mut String, content: String) -> io::Result<()> {
    output.write_str(&content).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}
