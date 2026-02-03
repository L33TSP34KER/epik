use std::fs::{self, write};

pub struct Makefile {
    files: Vec<String>,
    flags: Vec<String>,
    name: Option<String>,
    compiler: Option<String>,
}

impl Makefile {
    pub fn new() -> Makefile {
        Makefile {
            files: vec![],
            flags: vec![],
            name: None,
            compiler: None,
        }
    }

    pub fn set_compiler(&mut self, name: &String) -> &mut Self {
        self.compiler = Some(name.into());
        self
    }

    pub fn set_name(&mut self, name: &String) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn add_file(&mut self, path: &String) -> &mut Self {
        self.files.push(path.into());
        self
    }

    pub fn add_flag(&mut self, flag: &String) -> &mut Self {
        self.flags.push(flag.into());
        self
    }

    pub fn compile(&self) -> String {
        let name = self.name.clone().unwrap_or_default();
        let compiler = self.compiler.clone().unwrap_or_default();
        let files = self.files.join(" \\\n\t");
        let args = self.flags.join(" ");
        let base = format!(
            r#"##
## EPITECH PROJECT, 2026
## Makefile
## File description:
## project makefile
##

SRC = {}

OBJ = $(SRC:.c=.o)
OBJ := $(OBJ:.cpp=.o)

CFLAGS   = -Wall -Wextra -Werror -pedantic -march=native {}
CXXFLAGS = -Wall -Wextra -Werror -pedantic -march=native -std=c++20 {}
LDFLAGS  = {}
NAME = {}
CC = {}

all: $(NAME)

$(NAME): $(OBJ)
	$(CC) $(OBJ) -o $(NAME) $(LDFLAGS)

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(OBJ) $(NAME)

re: clean all

.PHONY: all clean re
"#,
            files, args, args, args, name, compiler
        );
        return base.to_string();
    }
}

pub fn add_files_to_src(makefile: String, files: Vec<String>) {
    let files: String = format!("SRC = {}", files.join(" \\\n\t"));
    let mut makefile: Vec<&str> = makefile.split("\n").collect();
    let mut start: usize = 0;
    let mut stop: usize = 0;

    for i in 0..makefile.len() {
        if makefile[i].contains("SRC =") {
            start = i;
        }

        if makefile[i].is_empty() && start != 0 {
            stop = i;
            break;
        }
    }

    if start == 0 && stop == 0 {
        crate::errors::error("can't find the SRC part of the makefile");
    }

    for _ in start..stop {
        makefile.remove(start);
    }

    makefile.insert(start, files.as_str());
    let result = makefile.join("\n");
    if let Err(e) = fs::write("Makefile", result) {
        crate::errors::error(&format!("Failed to write Makefile: {}", e));
    }
}

pub fn add_flags_to_src(makefile: String, flags: Vec<String>) {
    let flags = flags.join(" ");
    let mut makefile: Vec<String> = makefile.lines().map(|s| s.to_owned()).collect();
    //let mut makefile: Vec<&str> = makefile.split("\n").collect();
    let mut index: Vec<usize> = Vec::new();

    for i in 0..makefile.len() {
        if makefile.get(i).unwrap().contains("FLAGS =") {
            index.push(i);
        }
    }

    for index in index {
        let line = makefile[index].clone();
        let new_line = format!("{line} {flags}").clone();
        makefile.remove(index);
        makefile.insert(index, new_line);
    }

    let result = makefile.join("\n");
    if let Err(e) = fs::write("Makefile", result) {
        crate::errors::error(&format!("Failed to write Makefile: {}", e));
    }
}
