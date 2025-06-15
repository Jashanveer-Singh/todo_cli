# to_do - A simple command-line task manager 📝

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
* [Contributing](#contributing)
* [License](#license)
* [Contact](#contact)
* [Acknowledgements](#acknowledgements)

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

Follow these instructions to get a copy of `to_do` up and running on your local machine.

### Prerequisites

You need to have Rust and Cargo (Rust's package manager) installed. The easiest way to get them is via `rustup`.

* **Rustup (Rust Toolchain Installer)**
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
    ```
    Follow the on-screen instructions. You might need to restart your terminal or `source ~/.cargo/env`.

### Installation

1.  **Clone the repository:**
    ```bash
    git clone [https://github.com/YOUR_GITHUB_USERNAME/to_do.git](https://github.com/YOUR_GITHUB_USERNAME/to_do.git)
    ```
2.  **Navigate into the project directory:**
    ```bash
    cd to_do
    ```
3.  **Build the application:**
    For a release (optimized) build, which is recommended for daily use:
    ```bash
    cargo build --release
    ```
    This will compile the executable to `./target/release/to_do`.

---

## Usage

Once installed, you can run `to_do` commands from your terminal.

**General USAGE:**
to_do &lt;COMMAND> [ARGS]


**Available COMMANDS:**

* `add <task>` &nbsp; &nbsp; &nbsp; &nbsp; Add a single new task
* `show` &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; Display all tasks
* `update` &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; Update an existing task
* `delete` &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; Delete a task

Use of any other command will display the help message.

**EXAMPLES:**

To add a new task:
```bash
to_do add Read a book

To view all tasks:
Bash

to_do show

To update a task (the command will guide you through selecting a task and its new status):
Bash

to_do update

To delete a task (the command will guide you through selecting a task to delete):
Bash

to_do delete

Running the executable:
Assuming you ran cargo build --release, you can run the commands directly:
Bash

./target/release/to_do add "Learn Rust"
./target/release/to_do show

For convenience, you might consider adding ./target/release to your system's PATH, or creating a symbolic link:
Bash

# From the project root
sudo ln -s $(pwd)/target/release/to_do /usr/local/bin/to_do
# Now you can just type 'to_do add ...' from anywhere
to_do add "Plan vacation"

Data Storage

All your to_do app data (tasks and their statuses) is stored as a JSON file.

    Location: The data file is located in your operating system's default data directory for applications, under a todo subdirectory.
        Linux/WSL: ~/.local/share/todo/tasks
        macOS: ~/Library/Application Support/todo/tasks
        Windows: C:\Users\<YourUser>\AppData\Local\todo\tasks

The file is named tasks.json.
Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are greatly appreciated.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

    Fork the Project
    Create your Feature Branch (git checkout -b feature/AmazingFeature)
    Commit your Changes (git commit -m 'feat: Add some AmazingFeature')
    Push to the Branch (git push origin feature/AmazingFeature) 5. Open a Pull Request
