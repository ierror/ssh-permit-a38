use colored::Colorize;
use std::process::exit;

pub fn error(msg: &str) {
    println!("{} {}", "Error:".red(), msg);
    exit(1);
}

pub fn prompt(msg: &str) {
    println!("{}", msg.yellow());
}

pub fn ok(msg: &str) {
    println!("{}", msg.green().bold());
}

pub fn warning(msg: &str) {
    println!("{}", msg.magenta());
}

pub fn info(msg: &str) {
    println!("{}", msg);
}
