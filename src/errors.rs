use std::process::exit;

const RED_BACKGROUND: &str = "\x1b[41m";
const BOLD: &str    = "\x1b[1m";
const RESET: &str = "\x1b[0m";

pub fn error(message: &str) {
    println!("\n{RED_BACKGROUND} {RESET} {BOLD}Epik{RESET}\n{RED_BACKGROUND} {RESET} {message}\n");
    exit(0);
}
