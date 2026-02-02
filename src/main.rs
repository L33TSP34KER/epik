mod errors;
mod makefile;
mod menu;
mod utils;
mod colors;

use colors::{*};
use clap::{Parser, Subcommand};
use makefile::Makefile;
use std::{
    fs,
    io::{self, Write},
};

///
/// Epik CLI
#[derive(Parser, Debug)]
#[command(
    name = "epik",
    version,
    about = "Epik makefile manager",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize a new epik project
    Init {
        /// Optional project name
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Update a file or add a new one
    #[command(alias = "update")]
    UpdateFile,

    /// Add multiple flags
    AddFlags {
        /// List of flags to add
        #[arg(value_name = "FLAG")]
        flags: Vec<String>,
    },

    /// Add a single flag
    AddFlag {
        /// Name of the flag to add
        flag: String,
    },
}

fn main() {
    let mut makefile = Makefile::new();
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => {
            println!("Running `epik init` for project: {:?}", name);
            menu::make_base(&mut makefile, name);
            let files = utils::collect_c_files(".");
            for i in files {
                makefile.add_file(&i);
            }
            let _ = fs::write("Makefile", makefile.compile());
            println!("\n{BOLD}{CYAN}Done setting up your Makefile!{RESET}");
        }
        Commands::UpdateFile => {
            if let Ok(makefile_file) = fs::read_to_string("Makefile") {
                let files = utils::collect_c_files(".");
                makefile::add_files_to_src(makefile_file, files);
                println!("{BOLD} {CYAN}the Makefile as been updated{RESET}");
                return;
            };
            errors::error("Makefile not found try to create a makefile or use epik init");
        }
        Commands::AddFlags { flags } => {
            println!("Running `epik add-flags` with: {:?}", flags);
            // TODO: implement multi-flag logic
        }
        Commands::AddFlag { flag } => {
            println!("Running `epik add-flag` with: {flag}");
            // TODO: implement single-flag logic
        }
    }
}
