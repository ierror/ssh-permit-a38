use colored::Colorize;
use std::process::exit;

pub fn error(msg: String) {
    println!("{} {}", "Error:".red(), msg);
    exit(1);
}

pub fn info(msg: String) {
    println!("{}", msg.yellow());
}
