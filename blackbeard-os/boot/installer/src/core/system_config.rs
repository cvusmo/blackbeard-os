// Sets hostname, timezone, and other system-wide settings
use std::fs;
use std::io;
use std::os::unix::fs::symlink;

#[cfg(feature = "safe-mode")]
pub fn set_hostname(hostname: &str) -> io::Result<()> {
    fs::write("/etc/hostname", hostname)?;
    Ok(())
}

#[cfg(feature = "safe-mode")]
pub fn set_timezone(tz: &str) -> io::Result<()> {
    fs::remove_file("/etc/localtime")?;
    symlink(format!("/usr/share/zoneinfo/{}", tz), "/etc/localtime")?;
    Ok(())
}

