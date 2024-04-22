// responsible for installation steps

mod install_config;
use crate::install_config::InstallConfig;
use std::io::{self, Write};

pub fn run_install_steps() -> Result<(), String> {
    let mut input = String::new();

    println!("Starting the installation process...");
    println!("Do you want to format the disk? (yes/no)");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut disk_choice = String::new();

    if input.trim().eq_ignore_ascii_case("yes") {
        println!("Please enter the disk identifier (e.g., /dev/sda):");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        disk_choice = input.trim().to_string();

        if let Err(e) = disk::format_disk(&disk_choice) {
            return Err(format!("Failed to format disk: {}", e));
        }
        println!("Disk {} has been formatted.", disk_choice);
    }

    input.clear();
    println!("Which desktop environment do you want to install? (Hyprland/XFCE)");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let de_choice = input.trim().to_uppercase();

    let answers = InstallationAnswers::new(disk_choice, de_choice.clone());
    if let Err(e) = answers.save_to_file("installation_answers.json") {
        return Err(format!("Failed to save installation answers: {}", e));
    }

    match de_choice.as_str() {
        "HYPRLAND" => environments::install_hyprland().map_err(|e| e.to_string())?,
        "XFCE" => environments::install_xfce().map_err(|e| e.to_string())?,
        _ => return Err("Invalid input, no desktop environment was installed.".to_string()),
    }

    Ok(())
}

