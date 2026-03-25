use std::io;
use crate::{SourceID, SourceManager, SourceText};
use std::path::Path;

/// Loads a file's content into the source manager.
///
/// # Args
/// * `manager`: The SourceManager to add to.
/// * `path`: The file path to load.
///
/// # Errors
/// Returns io::Error if file reading fails.
///
/// # Returns
/// (SourceID, &SourceText) for the loaded file.
pub fn load_from_file<'a>(manager: &'a mut SourceManager, path: &Path) -> io::Result<(SourceID, &'a SourceText)> {
    let contents = std::fs::read_to_string(path)?;
    let path = path.to_str().unwrap_or("<unknown file path>").to_string();
    Ok(manager.add_source(path, contents))
}

/// Loads virtual content into the source manager.
///
/// # Args
/// * `manager`: The SourceManager to add to.
/// * `path`: The virtual path/name.
/// * `contents`: The content string.
///
/// # Returns
/// (SourceID, &SourceText)
pub fn load_virtual<'a>(
    manager: &'a mut SourceManager,
    path: &str,
    contents: &str,
) -> (SourceID, &'a SourceText) {
    manager.add_source(path.to_string(), contents.to_string())
}

/// Loads a line from stdin into the source manager.
///
/// # Args
/// * `manager`: The SourceManager to add to.
///
/// # Errors
/// Returns io::Error if reading fails.
///
/// # Returns
/// (SourceID, &SourceText)
pub fn load_from_stdin(manager: &mut SourceManager) -> io::Result<(SourceID, &SourceText)> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(manager.add_source("<stdin>".to_string(), buffer))
}
