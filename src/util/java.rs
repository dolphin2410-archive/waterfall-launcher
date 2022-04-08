use std::process::Command;

pub fn java_exists() -> bool {
    if let Ok(_) = Command::new("java").output() {
        true
    } else {
        false
    }
}

/// Finds the `JAVA_HOME` environment variable. Returns `Option::None` if doesn't exist.
pub fn java_home() -> Option<String> {
    if let Ok(data) = std::env::var("JAVA_HOME") {
        Option::Some(data)
    } else {
        Option::None
    }
}

/// Find the java executable. Searches the `java` command, and if it doesn't exist, it will search for the `JAVA_HOME` environment variable. Will panic otherwise.
pub fn find_executable() -> String {
    if java_exists() {
        String::from("java")
    } else if let Some(home) = java_home() {
        home
    } else {
        panic!("No Java Executable Found!")
    }
}