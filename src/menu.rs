use crate::colors::{*};
use crate::makefile::Makefile;
use std::io::{self, Write};

pub fn make_base(make: &mut Makefile, name: Option<String>) {
    io::stdout().flush().unwrap();

    let mut name: String = name.unwrap_or_default();
    let mut compiler = String::new();
    let mut flags_str = String::new();

    println!("{BOLD}{CYAN_BACKGROUND} {RESET}");
    println!("{BOLD}{CYAN_BACKGROUND} {RESET}{CYAN}   Makefile Setup Menu   {RESET}");
    println!("{BOLD}{CYAN_BACKGROUND} {RESET}                        {RESET}");
    println!("{YELLOW}Fill in the fields below (comma-separated flags).{RESET}\n");

    if name.is_empty() {
        print!("{BOLD}{CYAN}Name:{RESET}\n");
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");
    }

    print!("{BOLD}{CYAN}Compiler:{RESET}\n");
    io::stdin()
        .read_line(&mut compiler)
        .expect("Failed to read line");

    print!("{BOLD}{CYAN}Flags:{RESET}\n");
    io::stdin()
        .read_line(&mut flags_str)
        .expect("Failed to read line");

    make.set_name(&name).set_compiler(&compiler);
    for flag in flags_str.split(",") {
        make.add_flag(&flag.trim().to_string());
    }

    println!("{}", make.compile());
    println!("{BOLD}{CYAN}does it look good? (y/n)");

    let mut confirmation = String::new();
    io::stdin()
        .read_line(&mut confirmation)
        .expect("Failed to read line");
    match confirmation.trim() {
        "y" => {
            println!("{RESET}");
            return;
        }
        _ => {
            make_base(make, None);
        }
    }
}
