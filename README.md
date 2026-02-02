# Epik — The Makefile Generator for C/C++ Projects 🚀

*Epik helps you focus on your code, not your build system*

---

## 📦 What is Epik?

Epik is a command-line utility designed to simplify the process of generating, updating, and managing `Makefile`s for C/C++ projects. Whether you're starting a new project or adding new files/flags to an existing one, Epik handles the boilerplate so you can focus on writing code.

---

## 🧰 Features

- ✅ `epik init` — Automatically generates a `Makefile` for your project with all `.c` files detected.
- ✅ `epik update-file` (or `epik add`) — Updates the `Makefile` to include new or changed source files.
- ✅ `epik add-flag` — Adds a single compiler flag (e.g., `-Wall`, `-O2`) to your `Makefile`.
- ✅ `epik add-flags` — Adds multiple compiler flags at once.
- 🎨 Colorful CLI output for better UX.
- 📁 Modular Rust architecture for easy extensibility.

---

## 🚀 Getting Started

### 1. Install

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed. Then:

```bash
cargo install epik
```

> *Note: This is a local dev tool. If you're working on the project, run `cargo build --release` and add the binary to your PATH.*

---

### 2. Initialize a New Project

```bash
epik init [PROJECT_NAME]
```

Example:

```bash
epik init my_project
```

This will:

- Create a `Makefile` in your current directory.
- Detect all `.c` files in the current directory and add them to the `Makefile`.
- Output a success message.

> If no name is provided, it defaults to the current directory name.

---

### 3. Add or Update Files

```bash
epik update-file
```

This will:

- Read your existing `Makefile`.
- Detect all `.c` files in the current directory.
- Update the `Makefile` to include any new or missing files.

> Use `--force` to force update even if no changes are detected.

```bash
epik update-file --force
```

---

### 4. Add Compiler Flags

#### Single Flag

```bash
epik add-flag -Wall
```

#### Multiple Flags

```bash
epik add-flags -O2 -g -Wall -Wextra
```

> Flags are added to the `CFLAGS` (or `CXXFLAGS` for C++) section of your `Makefile`.

---

## 📄 Example Makefile Output

After running `epik init`, your `Makefile` might look like this:

```makefile
CC = gcc
CFLAGS = -Wall -O2

all: main

main: main.c utils.c
	$(CC) $(CFLAGS) -o $@ $^

clean:
	rm -f main
```

---

## 🛠️ How It Works

Epik uses a modular Rust architecture:

- `mod menu;` — CLI command routing and UX.
- `mod utils;` — File system helpers (e.g., `collect_c_files`).
- `mod makefile;` — Core logic for parsing and generating Makefiles.
- `Makefile` struct — Represents the state of your Makefile.

---

## 📚 Roadmap

- [ ] Support for `Makefile` templates (e.g., for different project types).
- [ ] `epik remove-flag` — Remove flags from Makefile.
- [ ] `epik list-files` — List all source files in Makefile.
- [ ] `epik validate` — Validate Makefile syntax.

---

## 🧪 Contribute

Epik is open-source and welcomes contributions! You can:

- Report bugs 🐞
- Suggest features 💡
- Submit pull requests ✅

> Fork the repo, make your changes, and submit a PR!

---

## 📜 License

MIT License — See [LICENSE](LICENSE) for details.

---

## 📬 Contact

If you have any questions, suggestions, or just want to say hi:

- GitHub: [https://github.com/yourusername/epik](https://github.com/yourusername/epik)
- Email: your.email@example.com

---

## 🙌 Thanks

Thank you for using Epik — let’s make C/C++ project setup easier, one Makefile at a time!

> *“Epik” = Easy Project Initiation Kit — because no one should waste time writing Makefiles.* 🎯

---

*Made with ❤️ and 💻 in Rust*  
*CLI Tool | C/C++ Project Automation | MIT Licensed*
