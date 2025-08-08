# FerrumShell 🐚⚙️

A minimal, educational shell written in Rust.  
The goal: learn **process management**, **I/O redirection**, **pipes**, and **command parsing** by building a shell from scratch.

Ferrum = Latin for iron → Rust vibes.  
`fsh` is the binary name — short, memorable, and sounds like "fish".

---

## 📜 Project Goals

FerrumShell is not meant to be a replacement for Bash/Zsh.  
Instead, it’s a learning project to explore:

- **REPL loops** (Read–Eval–Print Loop)
- Parsing user commands into tokens
- Running external commands via `std::process::Command`
- Handling **builtins** like `cd` and `exit`
- Connecting commands via **pipes** (`|`)
- Redirecting input/output (`<`, `>`)
- Managing multiple child processes

---

## 🛠 Features (MVP)

- Interactive prompt (`fsh>`)
- Run external programs (`ls`, `grep`, etc.)
- Builtins:
  - `cd` — change directory
  - `exit` — exit shell
- Piping: `cmd1 | cmd2`
- Redirection:
  - Output: `cmd > file`
  - Input: `cmd < file`
- Quoted arguments supported (`echo "hello world"`)

---

## 🚀 Roadmap

| Stage | Feature | Status |
|-------|---------|--------|
| 1 | REPL loop | ✅ |
| 2 | Run commands | ✅ |
| 3 | Builtins (`cd`, `exit`) | ✅ |
| 4 | Pipes (`|`) | ✅ |
| 5 | Redirection (`<`, `>`) | ✅ |
| 6 | Quoted args via `shell-words` crate | ✅ |
| 7 | Command history (via `rustyline`) | 🔜 |
| 8 | Background jobs (`&`) | 🔜 |
| 9 | Signal handling (Ctrl+C) | 🔜 |
| 10 | Autocomplete | 🔜 |

---

## 📂 Project Structure

ferrumshell/
├── Cargo.toml # Dependencies and project metadata
└── src/
└── main.rs # Shell implementation



---

## 🔧 Installation

**Prerequisites:**
- Rust toolchain (install via [rustup](https://rustup.rs/))
- Unix-like OS recommended (Linux/macOS). Windows works with some differences.

**Build & Run:**
```bash
git clone https://github.com/OmSharma42306/ferrumshell.git
cd ferrumshell
cargo build --release
./target/release/fsh
