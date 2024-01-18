use std::env;
use std::process::{exit, Command};

fn main() {
    // Get the project name from command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure that the project name is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <project_name>", args[0]);
        exit(1);
    }

    // Extract the project name from the command-line arguments
    let project_name = &args[1];

    // Run `cargo new` to create a new project
    let status = Command::new("cargo")
        .arg("new")
        .arg(project_name)
        .status()
        .expect("Failed to execute 'cargo new' command");

    // Check if the 'cargo new' command was successful
    if !status.success() {
        eprintln!("Failed to create a new project");
        exit(1);
    }

    // Navigate into the project directory
    let project_path = format!(
        "{}/{}",
        std::env::current_dir().unwrap().to_str().unwrap(),
        project_name
    );
    if let Err(err) = env::set_current_dir(&project_path) {
        eprintln!(
            "Failed to change directory to the newly created project: {}",
            err
        );
        exit(1);
    }

    // Add the 'axum' crate to the project using 'cargo add'
    let status_axum = Command::new("cargo")
        .arg("add")
        .arg("axum")
        .status()
        .expect("Failed to execute 'cargo add axum' command");

    // Check if the 'cargo add axum' command was successful
    if !status_axum.success() {
        eprintln!("Failed to add 'axum' crate to the project");
        exit(1);
    }

    // Add the 'tokio' crate to the project using 'cargo add' with features
    let status_tokio = Command::new("cargo")
        .arg("add")
        .arg("tokio")
        .arg("--features")
        .arg("full")
        .status()
        .expect("Failed to execute 'cargo add tokio' command");

    // Check if the 'cargo add tokio' command was successful
    if !status_tokio.success() {
        eprintln!("Failed to add 'tokio' crate to the project");
        exit(1);
    }

    // Add the 'serde' crate to the project using 'cargo add' with features
    let status_serde = Command::new("cargo")
        .arg("add")
        .arg("serde")
        .arg("--features")
        .arg("derive")
        .status()
        .expect("Failed to execute 'cargo add serde' command");

    // Check if the 'cargo add serde' command was successful
    if !status_serde.success() {
        eprintln!("Failed to add 'serde' crate to the project");
        exit(1);
    }
}
