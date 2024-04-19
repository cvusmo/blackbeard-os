pub fn get_questions() -> Vec<(String, Vec<String>)> {
    vec![
        (
            "What's your favorite color?".to_string(),
            vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
        ),
        (
            "What's your favorite food?".to_string(),
            vec!["Pizza".to_string(), "Burger".to_string(), "Sushi".to_string()],
        ),
        (
            "Select your desktop environment: GNOME, KDE, or XFCE?".to_string(),
            vec!["GNOME".to_string(), "KDE".to_string(), "XFCE".to_string()],
        ),
    ]
}

