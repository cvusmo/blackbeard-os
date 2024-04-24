// main.rs
mod core;
mod config;

use core::{bootloader, disk, file_ops, network};
use core::install::{installer, install_steps, install_config};
use std::process::Command;
use std::io::{self, Write};

fn check_internet_connection() -> bool {
    println!("Checking internet connection...");
    let output = Command::new("ping")
        .args(&["-c", "1", "1.1.1.1"]) // fuck google, use cloudfare
        .output();
    match output {
        Ok(output) => {
            let success = output.status.success();
            if success {
                println!("Internet connection verified.");
                true
            } else {
                println!("No internet connection detected. Please connect to the internet before proceeding.");
                if let Err(e) = network::run_networksetup()
                {
                    println!("[NETWORKSETUP] Beginning networksetup");
                }
                false
            }
        },
        Err(_) => {
            println!("Failed to execute internet connection check.");
            false
        }
    }
}

fn main() {
    println!("blackbeard-os: hoist the black flag, it's a pirates life for me. (btw, i use arch)");

    println!("Choose installation mode: CLI or GUI (type 'CLI or 'GUI'):");
    let mut mode = String::new();
    io::stdin().read_line(&mut mode).expect("Failed to read line");

    if mode.trim().eq_ignore_ascii_case("GUI") {
        // Launch GUI installer
        println!("Filthy casual... launching GUI installer....");
    } else {
        println!("Parry this... continuing with CLI installer...");
    }

    if !check_internet_connection() {
        return; // send to network.rs to setup network connection if no connection
    }

    println!("Do you want to start the installation? (yes/no)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim().eq_ignore_ascii_case("yes") {
        let disk = choice_selections[0].clone();
        let desktop_environment = choice_selections[1].clone();
    
        for (question, choices) in questions_with_choices {
            println!("{}", question);
            for (index, choice) in choices.iter().enumerate() {
                println!("{}: {}", (b'A' + index as u8) as char, choice);
            }
            println!("Enter your choice (A, B, C):");

            input.clear();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let choice_index = input.trim().to_uppercase().chars().next().unwrap_or('A') as usize - 'A' as usize;
            choice_selections.push(choices[choice_index].clone());
        }
        
        // choice_selections creates an instance using data provided by the user
        let config = InstallConfig::new(choice_selections.disk.clone(), choice_selections.desktopenvironment.clone());
        config.save_to_file("path/to/file.json").expect("failed to save file");

        //bootloader installation
        let bootloader_result = core::bootloader::install_systemd_boot(
            "/path/to/esp",
            "blackbeard-os",
            "/boot/vmlinuz-linux-zen",
            "/boot/initramfs-linux-zen.img"
        );

        installer::run_installation(&choice_selections); 
        println!("Installation is complete!");
    } else {
        println!("Installation canceled.");
    }
}

