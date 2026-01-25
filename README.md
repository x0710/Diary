# Diary

A minimal personal diary CLI tool written in Rust.

> This project is under active development.

---

## Overview

**Diary** is a lightweight command-line tool for recording, viewing, and managing short daily notes.

It is designed to be:

* Simple and fast
* Script-friendly
* Backed by SQLite
* Easy to extend and hack on

The project is still evolving, and some features are experimental.

---

## Features

* ✅ Add diary entries by date
* ✅ Read entries for a specific day
* ✅ Persistent storage using SQLite
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
target/release/diary
```

You may want to move it into your `$PATH`.

### From releases

Download the prebuilt binary from the Releases page and run it directly.

---

## Usage

Run `diary` to start the interactive mode.

```bash
diary
```

### Add an entry

```bash
>: add 20250101 "What happened today"
```

Supported date formats:

* 16th of this month: `m16`
* March 27 of this year: `ye0327`
* today: `today` / `t`
* tomorrow: `tomorrow` / `tom`
* yesterday: `yesterday` / `yes` / `y`
* specific date: `YYYYMMDD` / ~~`YYYY-MM-DD`~~

### Read an entry

```bash
>: check 2025-01-01
```

### List all entries

```bash
>: ls
```

---

## Data Storage

* Entries are stored locally in an SQLite database
* No network access
* No background services

---

## Roadmap

* [ ] Full-text search
* [x] Configurable editor support (CLI)
* [x] Import / export (Only support for JSON now)
* [x] GUI
* [ ] Visualization / charts?

---

## Development Status

This project is primarily developed for learning and personal daily use.
APIs and behavior may change without notice.

---

## License

MIT License

---

## Contributing

**Issues and pull requests are welcome.**

Feel free to experiment, refactor, or suggest improvements.

---

> A diary should be simple.
> The tool should be even simpler.

