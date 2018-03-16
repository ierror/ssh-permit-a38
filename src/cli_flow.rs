use colored::Colorize;
use std::io;
use std::io::Write;
use std::process::exit;

pub fn errorln(msg: &str) {
    println!("{} {}", "Error:".red(), msg);
    exit(1);
}

pub fn prompt(msg: &str) {
    print!("{} ", msg.yellow());
    io::stdout().flush().expect("Unable to flush");
}

pub fn promptln(msg: &str) {
    println!("{}", msg.yellow());
}

pub fn prompt_yes_no(msg: &str) -> String {
    let mut yes_no;

    loop {
        yes_no = String::from("");

        prompt(&format!("{}", msg));
        io::stdin()
            .read_line(&mut yes_no)
            .ok()
            .expect("Couldn't read line (y/n)");

        yes_no = yes_no.trim_right().trim_left().to_owned();
        if yes_no == "n" || yes_no == "y" {
            break;
        }
    }

    yes_no
}

pub fn read_line(msg: &str, default: &str) -> String {
    prompt(msg);

    let mut input = String::from("");

    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Couldn't read_line");

    input = String::from(input.trim_right().trim_left());

    if !default.is_empty() && input.is_empty() {
        return String::from(default);
    }

    input
}

pub fn okln(msg: &str) {
    println!("{}", msg.green().bold());
}

pub fn warningln(msg: &str) {
    println!("{} {}", "Warning:".magenta(), msg);
}

pub fn infoln(msg: &str) {
    println!("{}", msg);
}
