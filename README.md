# Diary

A minimal personal diary CLI tool written in Rust.

> Don’t be anxious — this project is under active development.

---

## Overview

**Diary** helps you quickly record, view, and manage short daily notes from the command line. It is designed to be:

* Simple and fast
* Script-friendly
* Backed by SQLite
* Easy to extend

The project is still evolving, and some features are experimental.

---

## Features

* ✅ Add diary entries by date
* ✅ Read entries for a specific day
* ✅ Persistent storage using SQLite
* 🚧 `ls` (list entries) — under development

---

## Installation

### Build from source

```bash
git clone https://github.com/yourname/diary.git
cd diary
cargo build --release
```

The binary will be located at:

```text
target/release/diary
```

You may want to move it into your `$PATH`.

---

## Usage

### Add an entry

```bash
diary add 2025-01-01 "What happened today"
```

Supported date formats:

* `YYYY-MM-DD`
* `YYYYMMDD`

### Read an entry

```bash
diary check 2025-01-01
```

### List entries (WIP)

```bash
diary ls
```

> ⚠️ This command is not fully implemented yet.

---

## Data Storage

* Entries are stored locally in an SQLite database
* No network access
* No background services

---

## Roadmap

* [ ] Complete `ls` command
* [ ] Interactive mode
* [ ] Full-text search
* [ ] Configurable editor support
* [ ] Import / export

---

## Development Status

This project is actively developed for learning and daily use. APIs and behavior may change without notice.

---

## License

MIT License

---

## Contributing

Issues and pull requests are welcome. Feel free to experiment, refactor, or suggest improvements.

---

> A diary should be simple. The tool should be even simpler.
