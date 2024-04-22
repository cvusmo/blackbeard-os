// Copy filesystem and other file-based operations during install
use std::fs;
use std::io;
use std::path::Path;

pub fn copy_files(src: &str, dest: &str) -> io::Result<()> {
    fs::copy(src, dest)?;
    Ok(())
}

