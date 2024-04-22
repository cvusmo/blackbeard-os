// main.rs
mod core;
mod install_steps;

use core::{install_config, installer};
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
                println!("No internet connection detected. Please connect to the internet before proceeding."); // if no internet connected -> goes to network.rs
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
        let questions_with_choices = install_steps::get_installation_questions(); 
        let mut choice_selections = Vec::new();

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
        
        install_config::save_answers(%choice_selections).expect("Failed to save answers");

        //bootloader installation
        let bootloader_result = core::bootlaoder::install_systemd_boot(
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

