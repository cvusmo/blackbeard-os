mod core;

use core::{questions, answers, installer};

fn main() {
    println!("Welcome to the installation!");
    println!("Do you want to start the installation? (yes/no)");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim().eq_ignore_ascii_case("yes") {
        let questions_with_choices = questions::get_questions();
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
        installer::run_installation(&choice_selections);
        println!("Installation is complete!");
    } else {
        println!("Installation canceled.");
    }
}

