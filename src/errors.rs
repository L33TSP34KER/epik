use std::process::exit;
use crate::colors::{*};

pub fn error(message: &str) {
    println!("\n{RED_BACKGROUND} {RESET}   {BOLD}Epik{RESET}\n{RED_BACKGROUND} {RESET}   {message}\n");
    exit(0);
}
