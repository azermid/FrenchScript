mod arg_handling;

use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, ErrorKind};

pub fn validate_file(path_str: &str, expected_ext: &str) -> io::Result<PathBuf> {
    let path = Path::new(path_str);

    if !path.exists()
        return Err(io::Error::new(ErrorKind::NotFound, "Le fichier n'existe pas"));
    if !path.is_file()
        return Err(io::Error::new(ErrorKind::InvalidInput, "Le chemin ne pointe pas vers un fichier"));
    match path.extension().and_then(|s| s.to_str())
    {
        Some(ext) if ext == expected_ext => (),
        _ => return Err(io::Error::new(ErrorKind::InvalidData, format!("Extension invalide, attendait .{}", expected_ext))),
    }

    Ok(path.to_path_buf())
}