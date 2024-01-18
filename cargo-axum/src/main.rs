use std::env;
use std::process::{exit, Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <project_name>", args[0]);
        exit(1);
    }

    let project_name = &args[1];

    let status = Command::new("cargo")
        .arg("new")
        .arg(project_name)
        .status()
        .expect("Failed to execute 'cargo new' command");

    if !status.success() {
        eprintln!("Failed to create a new project");
        exit(1);
    }

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

    let status_axum = Command::new("cargo")
        .arg("add")
        .arg("axum")
        .status()
        .expect("Failed to execute 'cargo add axum' command");

    if !status_axum.success() {
        eprintln!("Failed to add 'axum' crate to the project");
        exit(1);
    }

    let status_tokio = Command::new("cargo")
        .arg("add")
        .arg("tokio")
        .arg("--features")
        .arg("full")
        .status()
        .expect("Failed to execute 'cargo add tokio' command");

    if !status_tokio.success() {
        eprintln!("Failed to add 'tokio' crate to the project");
        exit(1);
    }

    let status_serde = Command::new("cargo")
        .arg("add")
        .arg("serde")
        .arg("--features")
        .arg("derive")
        .status()
        .expect("Failed to execute 'cargo add serde' command");

    if !status_serde.success() {
        eprintln!("Failed to add 'serde' crate to the project");
        exit(1);
    }
}
