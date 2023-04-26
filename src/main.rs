use std::env;
use std::error::Error;
use std::io::{self, Read};
use std::fs::File;
use std::process::{Command, Stdio};
use dialoguer::Select;
use serde::{Deserialize, Serialize};

const VSCODE_TASKS_FILE_PATH: &str = ".vscode/tasks.json";

#[derive(Serialize, Deserialize)]
struct Task {
    label: String,
    command: String,
    #[serde(rename = "type")]
    task_type: String,
}

#[derive(Serialize, Deserialize)]
struct TaskFile {
    version: String,
    tasks: Vec<Task>,
}

fn get_vscode_tasks_file()-> Result<File, io::Error>  {
    let current_dir = env::current_dir()?;
    let file_path = current_dir.join(VSCODE_TASKS_FILE_PATH);
    let file = File::open(file_path)?;

    Ok(file)
}

fn run_shell_command(command: &str)-> () {
    let output =  Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    let mut stdout = output.stdout.expect("Failed to open stdout");
    let mut stdout_writer = io::BufWriter::new(io::stdout());

    io::copy(&mut stdout, &mut stdout_writer)
        .expect("Failure durning writing output of command");
}

fn run_command()-> Result<(), Box<dyn Error>> {
    let mut file = get_vscode_tasks_file()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");

    let task_file: TaskFile = serde_json::from_str(&contents)
        .expect("Failed to deserialize JSON");

    let labels: Vec<String> = task_file.tasks
        .iter()
        .map(|task| task.label.clone())
        .collect();

    let selection = Select::new()
        .with_prompt("Select a task:")
        .items(&labels)
        .interact()
        .unwrap();

    let vscode_command = &task_file.tasks[selection].command;

    println!("Running command: {}", vscode_command);

    let commands: Vec<&str> = vscode_command.split("&&").collect();

    commands.iter().for_each(|command| run_shell_command(command));

    Ok(())
}

fn main() {
    if let Err(e) = run_command() {
        println!("Error: {}", e.to_string());
    }
}

