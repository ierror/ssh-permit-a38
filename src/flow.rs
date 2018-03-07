use colored::Colorize;
use std::process::exit;

pub fn error(msg: &str) {
    println!("{} {}", "Error:".red(), msg);
    exit(1);
}

pub fn info(msg: &str) {
    println!("{}", msg.yellow());
}
