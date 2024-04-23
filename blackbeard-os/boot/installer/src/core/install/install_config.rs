// Responsible for managing configuration data

use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, prelude::*};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct InstallConfig {
    disk: String,
    desktop_environment: String,
}

trait Cloneable {
    fn clone(&self) -> Self where Self: Sized;
}

impl Cloneable for InstallConfig {
    fn clone(&self) -> Self {
        InstallConfig {
            disk: self.disk.clone(),
            desktop_environment: self.desktop_environment.clone(),
        }
    }
}

impl InstallConfig { 
    pub fn new(disk: String, desktop_environment: String) -> Self {
        #[cfg(feature = "safe-mode")]
        {
            println!("[SAFEMODE] Creating InstallConfig with disk: {}, desktop: {}", &disk, &desktop_environment); 
        }
        InstallConfig {
            disk,
            desktop_environment,
        }
    }

    pub fn save_to_file(&self, file_path: &str) -> std::io::Result<()> {
        #[cfg(feature = "safe-mode")]
        {
            println!("[SAFEMODE] Saving InstallConfig to file: {}", file_path);
            return Ok(());
        }

        #[cfg(not(feature = "safe-mode"))]
        {
            let file = File::create(file_path)?;
            let writer = std::io::BufWriter::new(file);
            serde_json::to_writer_pretty(writer, &self)?;
            Ok(())
        }
    }
}

