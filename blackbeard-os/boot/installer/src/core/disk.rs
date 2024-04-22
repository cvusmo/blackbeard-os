// responsible for disk partitioning and formatting
use std::process::Command;
use std::io;

pub fn create_partition(disk: &str) -> io::Result<()> {
    let status = Command::new("parted")
        .arg(disk)
        .arg("--script")
        .arg("--")
        .arg("mklabel")
        .arg("gpt")
        .arg("mkpart")
        .arg("primary")
        .arg("ext4")
        .arg("1MiB")
        .arg("100%")
        .status()?;

    if status.success() {
        Command::new("mkfs.ext4")
            .arg(format!("{}1", disk))
            .status()?;
    } else {
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to partition

