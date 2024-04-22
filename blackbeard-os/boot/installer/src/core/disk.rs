// Responsible for disk partitioning and formatting
use std::process::Command;
use std::io;

// Enum to define all filesystems
enum FileSystemType {
    Ext4,
    Btrfs,
    Xfs,
}

pub fn create_partition(disk: &str, fs_type: FileSystemType) -> io::Result<()> {
    let fs_type_str = match fs_type {
        Ext4,
        Btrfs,
        Xfs,
    };

    #[cfg(feature = "safe-mode")]
    {
        println!("[SAFEMODE] Running parted and mkfs with: {}", fs_type_str);
        return Ok(()); //mock return in safe mode
    }

    #[cfg(not(feature = "safe-mode"))]
    {
        let status = Command::new("parted")
            .arg(disk)
            .arg("--script")
            .arg("--")
            .arg("mklabel")
            .arg("gpt")
            .arg("mkpart")
            .arg("primary")
            .arg(fs_type_str)
            .arg("1MiB")
            .arg("100%")
            .status()?;

        // to do: add more filesystems
        if status.success() {
            let format_cmd = format!("mkfs.{}", fs_type_str);
            let format_status = Command::new(format_cmd)
                .arg(format!("{}1", disk))
                .status()?;

        if format_status.success() {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Failed to format disk"))
        }
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Failed to partition"))
        }
    }
}
