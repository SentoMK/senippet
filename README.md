# SENPROMPT

[简体中文](README_zh-CN.md)

`Current Version: 0.1.0`

Senprompt is a CLI code snippet/template management tool built with Rust, designed to help you organize, store, and utilize frequently used code snippets, configuration templates, or command-line scripts, thereby enhancing efficiency in software development.

## Features

- **Add Snippets/Templates:** Easily add code snippets or templates with a title, content, and tags.

- **List Snippets/Templates:** View all saved code snippets or templates.

- **Search by Tag:** Quickly find code snippets or templates related to specific tags.

- **Edit Snippets/Templates:** Modify existing code snippets or templates, including the title, content, and tags.

- **Delete Snippets/Templates:** Remove code snippets or templates that are no longer needed.

- **Show Data Path:** Locate the directory where code snippets or template data is stored.

- **Version Information:** Display the tool's version.

- **User-Friendly Interface:** Features colorful output and clear menu options.

## Installation

### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install) and Cargo (the Rust package manager).

### Install from Crates.io

```bash
cargo install senprompt
```

### Install from Source Code

1.  Clone the repository:

    ```bash
    git clone https://github.com/SentoMK/senprompt
    cd senprompt
    ```

2.  Build and install:

    ```bash
    cargo install --path .
    ```

## Usage

1.  Run the senprompt command in the terminal:

    ```bash
    senprompt
    ```

2.  Follow the menu prompts:

    ```
    🛠️  SENPROMPT CLI v0.1.0


    📂 SENPROMPT CLI
    1) Add Snippet/Template
    2) List Snippets/Templates
    3) Search by Tag
    4) Edit Snippets/Templates
    5) Delete Snippets/Templates
    6) Show Data Path
    7) Exit
    Choose option:
    ```

    - **1. Add Snippet/Template:**

      Enter the title, content, and tags (comma-separated) for the code snippet/template.

    - **2. List Snippets/Templates:**

      Display all saved code snippets/templates, including the index, title, and tags.

    - **3. Search by Tag:**

      Enter the tag to search for, and display matching code snippets/templates.

    - **4. Edit Snippets/Templates:**

      Select the index of the code snippet/template to edit, then modify the title, content, or tags.

    - **5. Delete Snippets/Templates:**

      Select the index of the code snippet/template to delete (multiple indices can be specified at once).

    - **6. Show Data Path:**

      Display the path to the directory where code snippet/template data is stored.

    - **7. Exit:**

      Exit the program.

## Use Cases

- **Store frequently used code snippets:** Such as common functions, classes, or loop structures.

- **Manage configuration file templates:** Such as Dockerfile, Kubernetes YAML files, or Nginx configuration files.

- **Store common command-line scripts:** Such as scripts for deployment, building, or testing.

- **Quickly find and reuse code:** Quickly find the required code snippets or templates using tags.

- **Improve development efficiency:** Avoid repeatedly writing the same code and reduce the possibility of errors.

## Detailed Instructions

### Adding a Code Snippet/Template

After selecting `1. Add Snippet/Template`, you will be prompted to enter the title, content, and tags in sequence. Separate tags with commas.

### Listing Code Snippets/Templates

After selecting `2. List Snippets/Templates`, all saved code snippets/templates will be listed, including the index, title, and tags.

### Searching by Tag

After selecting `3. Search by Tag`, enter the tag you want to search for. The program will display all code snippets/templates containing that tag.

### Editing a Code Snippet/Template

After selecting `4. Edit Snippets/Templates`, all code snippets/templates will be listed. Enter the index of the code snippet/template to edit (multiple indices can be separated by commas). You can then modify the title, content, and tags of each code snippet/template. If you do not want to modify a field, simply press Enter to keep the original value.

### Deleting Code Snippets/Templates

After selecting `5. Delete Snippets/Templates`, all code snippets/templates will be listed. Enter the index of the code snippet/template to delete (multiple indices can be separated by commas). The program will delete the specified code snippets/templates.

### Showing the Data Path

After selecting `6. Show Data Path`, the path to the directory where code snippet/template data is stored will be displayed. You can find the JSON file storing the code snippets/templates in this directory.

## Configuration File

Code snippet/template data is stored in a JSON file. This file is located in the following directory:

- `Linux: $HOME/.local/share/senprompt/data`

- `macOS: $HOME/Library/Application Support/com.sentomk.senprompt/data`

- `Windows: C:\Users\<Your User>\AppData\Roaming\sentomk\senprompt\data`

## Contributing

Contributions are welcome! Please submit an Issue or a Pull Request.
