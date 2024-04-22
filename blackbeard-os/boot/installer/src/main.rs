// main.rs
mod core;
mod install_steps; // Assuming install_steps.rs is properly set up for installation tasks
use core::{answers, installer};
use std::process::Command;
use std::io::{self, Write};

fn check_internet_connection() -> bool {
    println!("Checking internet connection...");
    let output = Command::new("ping")
        .args(&["-c", "1", "1.1.1.1"]) // fuck google, lets use cloudfare
        .output();
    match output {
        Ok(output) => {
            let success = output.status.success();
            if success {
                println!("Internet connection verified.");
            } else {
                println!("No internet connection detected. Please connect to the internet before proceeding."); // if no internet connected -> goes to network.rs 
            }
            success
        },
        Err(_) => {
            println!("Failed to execute internet connection check.");
            false
        }
    }
}

fn main() {
    println!("Welcome to the installation of your custom Arch Linux distro!");

    if !check_internet_connection() {
        return;
    }

    println!("Do you want to start the installation? (yes/no)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim().eq_ignore_ascii_case("yes") {
        let questions_with_choices = install_steps::get_installation_questions(); // Replaces questions::get_questions()
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

        answers::save_answers(&choice_selections).expect("Failed to save answers");
        installer::run_installation(&choice_selections); // Consider integrating installation logic directly here or in `install_steps`
        println!("Installation is complete!");
    } else {
        println!("Installation canceled.");
    }
}

