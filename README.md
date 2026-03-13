# Diary

A minimal personal diary CLI tool written in Rust.

> This project is under active development.

---

## Overview

**Diary** is a lightweight command-line diary tool for quickly recording, viewing, and managing daily brief notes.

Design goals:

* Simple and fast
* Suitable for scripts and terminal use
* Local storage using SQLite
* Easy to extend and refactor

The project is still evolving, and some features may change over time.

---

## Features

* ✅ Add diary entries by date
* ✅ View entries for a specific date
* ✅ Persistent storage with SQLite
* ✅ List existing entries

---

## Installation

### Build from source

```bash
git clone https://github.com/x0710/diary.git
cd diary
cargo build --release
```

The binary will be located at:

```text
target/release/{diary-cli,diary-gui}
```

You can move it to your `$PATH` for easier use.

> Note: CLI version is highly unstable on Windows terminals because they don’t support CLI editors. Please use the GUI version on Windows.

### Use Prebuilt Releases

You can also download precompiled binaries directly from the GitHub Releases page.

---

## Usage

## Cli Version

Run `diary-cli` to start the interactive mode.

```bash
diary-cli
```

### Add an entry

```bash
>: ad 20250101 "What happened today"
```

Supported date formats:

* 16th of this month: `m16`
* March 27 of this year: `ye0327`
* today: `today` / `t`
* tomorrow: `tomorrow` / `tom`
* yesterday: `yesterday` / `yes` / `y`
* specific date: `YYYYMMDD` / ~~`YYYY-MM-DD`~~

### View Entries for a Specific Date

```bash
>: check 2025-01-01
```

### List all entries

```bash
>: ls
```

### Import / Export Data

Now, Cli Version supports CSV and JSON formats.

```bash
# Export data to filename.json with json
diary-cli export <filename.json> --json

# Import data from filename.json with json
diary-cli import <filename.json> --json
```

---

## Data Storage

* All data is stored locally in an SQLite database
* No network connection required
* No background service running

---

## Development Roadmap

* [ ] Full-text search
* [x] Configurable editor support
* [x] Import / Export
* [x] Graphical User Interface (GUI)
* [ ] Visualization / Charts?

---

## Development Status

This project is primarily for learning and personal use.
APIs and behavior may change in the future, and backward compatibility is not guaranteed.

---

## License

MIT License

---

## Contributing

**Contributions via Issues and Pull Requests are welcome.**

Whether it’s refactoring, experimental changes, or feature suggestions, all contributions are appreciated.

---

> The diary should be simple enough,
> and the tool itself should be even simpler.
