// sets hostname, timezone, and other system-wide settings
use std::fs;
use std::io;

pub fn set_hostname(hostname: &str) -> io::Result<()> {
    fs::write("/etc/hostname", hostname)?;
    Ok(())
}

pub fn set_timezone(tz: &str) -> io::Result<()> {
    fs::remove_file("/etc/localtime")?;
    fs::symlink(format!("/usr/share/zoneinfo/{}", tz), "/etc/localtime")?;
    Ok(())
}

