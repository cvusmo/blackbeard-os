use std::thread;
use std::time::Duration;

pub fn run_installation(choice_selections: &[String]) {
    println!("Starting the installation based on your answers...");
    for (index, choice_selection) in choice_selections.iter().enumerate() {
        println!("Processing choice {}: {}", index + 1, choice_selection);
        thread::sleep(Duration::from_secs(3));
    }
    println!("All options based on the answers have been processed.");
}

