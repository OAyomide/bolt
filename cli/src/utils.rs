use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::io::Write;
use crate::build_dist;

pub fn run_command(shell_command: String, dir: String) {
    let mut child = Command::new("sh")
        .current_dir(dir)
        .arg("-c")
        .arg(shell_command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start command");

    let stdout = BufReader::new(child.stdout.take().unwrap());
    let stderr = BufReader::new(child.stderr.take().unwrap());

    let stdout_lines = stdout.lines();
    let stderr_lines = stderr.lines();

    let mut output = io::LineWriter::new(io::stdout());

    for line in stdout_lines.chain(stderr_lines) {
        if let Ok(line) = line {
            writeln!(output, "{}", line).expect("Failed to write to stdout");
            io::stdout().flush().expect("Failed to flush stdout");
        } else {
            break;
        }
    }

    let status = child.wait().expect("Failed to wait for command");
    if !status.success() {
        eprintln!("Command failed with status: {}", status);
    }
}

pub fn open_browser(link: String) {
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    println!("opening browser");

    webbrowser::open(&link).unwrap();
}

pub fn reset_home() {
    println!("reseting dist");

    let home_path = get_home();

    std::fs::remove_dir_all(home_path).unwrap(); // deletes the local copy

    verify_home();
    verify_dist();

    println!("Dist has been reset");
}

pub fn get_dist() -> String {
    get_home() + "dist/"
}

pub fn get_home() -> String {
    let path = dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/bolt/";
    path
}

pub fn verify_home() {
    let path = get_home();
    if !dir_exists(&path) {
        println!("Creating directory {}", path);
        create_home(&path);
    }
}

pub fn verify_dist() {
    let path = get_dist();
    if !dir_exists(&path) {
        println!("Creating dist");
        build_dist();
    }
}

pub fn dir_exists(path: &String) -> bool {
    if Path::new(&path).exists() {
        true
    } else {
        println!("Directory {} does not exist", path);
        false
    }
}

pub fn create_home(path: &String) {
    std::fs::create_dir(path).unwrap();
}
