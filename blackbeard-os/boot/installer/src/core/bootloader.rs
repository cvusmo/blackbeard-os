// responsible for installation and configuration of the bootloader
use std::fs;
use std::io;
use std::path::Path;

pub fn install_systemd_boot(esp_path: &str) -> io::Result<()> {
    let systemd_boot_path = Path::new(esp_path).join("EFI/BOOT");
    fs::create_dir_all(&systemd_boot_path)?;

    // Copy the systemd-boot binary to the ESP
    fs::copy("/usr/lib/systemd/boot/efi/systemd-bootx64.efi", systemd_boot_path.join("BOOTX64.EFI"))?;

    // Create loader entries, example configuration
    let loader_conf = systemd_boot_path.join("loader/loader.conf");
    let entry_conf = systemd_boot_path.join("loader/entries/arch.conf");

    fs::write(loader_conf, "default arch\n")?;
    fs::write(entry_conf, "title   Arch Linux\nlinux   /vmlinuz-linux\ninitrd  /initramfs-linux.img\noptions root=PARTUUID=xxx rw\n")?;
    Ok(())
}

