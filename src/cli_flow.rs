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
    prompt(&format!("{}", msg));

    let mut yes_no = String::new();
    loop {
        yes_no = String::new();

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

pub fn okln(msg: &str) {
    println!("{}", msg.green().bold());
}

pub fn warningln(msg: &str) {
    println!("{}", msg.magenta());
}

pub fn infoln(msg: &str) {
    println!("{}", msg);
}
