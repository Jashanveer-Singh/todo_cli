# todo - A simple command-line task manager 📝

![Rust](https://github.com/rust-lang/rust/actions/workflows/ci.yml/badge.svg) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![GitHub Stars](https://img.shields.io/github/stars/YOUR_GITHUB_USERNAME/YOUR_REPO_NAME?style=social)
![GitHub Forks](https://img.shields.io/github/forks/YOUR_GITHUB_USERNAME/YOUR_REPO_NAME?style=social)

A lightweight and efficient command-line application written in Rust for managing your daily tasks. Keep track of your to-dos with simple commands, assigning them clear statuses like `waiting`, `ongoing`, and `completed`. All your task data is securely stored locally in a JSON format.

## Table of Contents

* [Features](#features)
* [Built With](#built-with)
* [Getting Started](#getting-started)
    * [Prerequisites](#prerequisites)
    * [Installation](#installation)
* [Usage](#usage)
* [Data Storage](#data-storage)

---

## Features

* **Add Tasks:** Quickly add new tasks from the command line.
* **Show Tasks:** Display a clear overview of all your tasks and their statuses.
* **Update Tasks:** Easily change task details or their status (e.g., from `waiting` to `ongoing` to `completed`).
* **Delete Tasks:** Remove completed or unwanted tasks.
* **Status Tracking:** Tasks can be in `waiting`, `ongoing`, or `completed` states.
* **Local JSON Storage:** All task data is persistently stored in a JSON file on your local machine.

## Built With

* [Rust](https://www.rust-lang.org/)

---

## Getting Started

Follow these instructions to get a copy of `todo` up and running on your local machine.

### Prerequisites

You need to have Rust and Cargo (Rust's package manager) installed. The easiest way to get them is via `rustup`.

### Installation

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/Jashanveer-Singh/todo_cli.git todo
    ```
2.  **Navigate into the project directory:**
    ```bash
    cd todo
    ```
3.  **Build the application:**
    For a release (optimized) build, which is recommended for daily use:
    ```bash
    cargo install --path .
    ```
    This will compile the executable to `.cargo/bin/todo`.


---

## Usage

Once installed, you can run `todo` commands from your terminal.

**General USAGE:**
todo &lt;COMMAND> [ARGS]


**Available COMMANDS:**

* `add <task>` &nbsp; &nbsp; &nbsp; &nbsp; Add a single new task
* `show` &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; Display all tasks
* `update` &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; Update an existing task
* `delete` &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; Delete a task

Use of any other command will display the help message.

**EXAMPLES:**

To add a new task:
```bash
todo add Read a book
```

To view all tasks:
```bash
todo show
```

To update a task (the command will guide you through selecting a task and its new status):
```bash
todo update
```
To delete a task (the command will guide you through selecting a task to delete):
```bash
todo delete
```

Running the executable:
Assuming you ran `cargo install --path .`, you can run the commands directly:

```bash
todo add Learn Rust
todo show
```
## Data Storage

All your todo app data (tasks and their statuses) is stored as a JSON file.

    Location: The data file is located in your operating system's default data directory for applications, under a todo subdirectory.
        Linux/WSL: ~/.local/share/todo/tasks
        macOS: ~/Library/Application Support/todo/tasks
        Windows: C:\Users\<YourUser>\AppData\Local\todo\tasks

The file is named tasks.
