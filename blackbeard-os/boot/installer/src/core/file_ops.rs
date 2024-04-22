// Copy filesystem and other file-based operations during install
use std::fs;
use std::io;
use std::path::Path;

pub fn copy_files(src: &str, dest: &str) -> io::Result<()> {
    
    #[cfg(feature = "safe-mode")]
    {
        println!("[SAFEMODE] Executing copy_files with src && dest: {}", src, dest);
        return Ok(());
    }

    #[cfg(not(feature = "safe-mode"))]
    {
        fs::copy(src, dest)?;
    }

    Ok(())
}

