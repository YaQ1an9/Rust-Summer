use std::path::Path;
use std::fs;
use regex::Regex;
use colored::*;

#[derive(PartialEq)]
pub enum Mode {
    Normal,
    Verbose,
}
pub fn find<P: AsRef<Path>>(root: P, regex: &Regex, mode: &Mode) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut matches = Vec::new();
    walk_tree(root.as_ref(), regex, mode, &mut matches)?;
    Ok(matches)
}

pub fn walk_tree(
    dir: &Path,
    regex: &Regex,
    mode: &Mode,
    matches: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        if *mode == Mode::Verbose {
            println!("Searching: {}", dir.display().to_string().yellow());
        }
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_tree(&path, regex, mode, matches)?;
            } else if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                if regex.is_match(filename) {
                    matches.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    Ok(())
}
