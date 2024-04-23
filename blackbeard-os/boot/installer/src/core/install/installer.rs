// Responsible for completion of installation steps and prompt for reboot if success
use crate::core::install_steps;

#[cfg(feature = "safe-mode")]
pub fn run_installation() {
    println!("Welcome to the installation!");
    println!("Do you want to start the installation? (yes/no)");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim().eq_ignore_ascii_case("yes") {
        match install_steps::run_install_steps() {
            Ok(_) => {
                println!("Installation is complete! Please reboot your system to use the new installation.");
            },
            Err(e) => {
                println!("Installation failed: {}", e);
            }
        }
    } else {
        println!("Installation canceled.");
    }
}

#[cfg(feature = "safe-mode")]
fn main() {
    run_installation();
}

