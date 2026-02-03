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
- Detect all `.c/cpp` files in the current directory and add them to the `Makefile`.
- Output a success message.

> If no name is provided, it defaults to the current directory name.

---

### 3. Add or Update Files

```bash
epik update-file
```

This will:

- Read your existing `Makefile`.
- Detect all `.c/cpp` files in the current directory.
- Update the `Makefile` to include any new or missing files.

```bash
epik update-file
```

---

### 4. Add Compiler Flags

#### Multiple Flags

```bash
epik add-flags -O2 -g -Wall -Wextra
epik add-flags -O2 # work too
```

> Flags are added to the `CFLAGS` (or `CXXFLAGS` for C++) section of your `Makefile`.

---

## 📚 Roadmap

- [ ] Support for `Makefile` templates (e.g., for different project types).
- [ ] `epik remove-flag` — Remove flags from Makefile.
- [ ] `epik list-files` — List all source files in Makefile.

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

*Made with ❤️ and 💻 in Rust*  
*CLI Tool | C/C++ Project Automation | MIT Licensed*
