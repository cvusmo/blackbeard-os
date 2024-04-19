use std::fs::File;
use std::io::{self, Write};

pub fn save_answers(choice_selections: &[String]) -> io::Result<()> {
    let mut file = File::create("answers.txt")?;
    for choice_selection in choice_selections {
        writeln!(file, "{}", choice_selection)?;
    }
    Ok(())
}

