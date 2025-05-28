# SENIPPET

[中文](README_zh-CN.md) | [English](README.md)

`Current Version: 0.1.1`

A CLI snippet (template) / prompt management tool built with Rust, designed to help you organize, store, and utilize frequently used code snippets, configuration templates, or command-line scripts to improve efficiency in software development.

## Features

- The current version introduces powerful add/edit template functionalities, supporting two editing modes:

  1.  **Single-line Editing:** Allows users to make quick and convenient modifications within a single line of text. Ideal for simple variable replacements, attribute adjustments, and more.

  2.  **Multi-line Editing:** Provides a more flexible editing environment, supporting the handling of templates containing multiple lines of text. Suitable for complex code snippets, configuration files, or text content requiring large-scale modifications.

- Can be operated directly via the command line. See [Usage](#usage) for details.

## Installation

**For non-Rust users, executable files for corresponding operating systems can be directly downloaded from the [Release](https://github.com/SentoMK/senippet/releases) page.**

### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install) and Cargo (Rust package manager).

### Install from Crates.io

```bash
cargo install senippet
```

````

### Install from Source Code

1. Clone the repository

```bash
git clone <https://github.com/SentoMK/senippet>
cd senippet
```

2. Build and install

```bash
cargo install --path .
```

### Add `senipt` to Environment Variable (PATH)

To be able to run the `senipt` command directly in any directory, you need to add it to the system's environment variable `PATH`.
The following takes the operation steps in Linux/macOS systems as an example.

1. **Find the Executable File:**

   Confirm that the `senippet/target/release/senipt` file exists. This is the location of your `senipt` executable file.

2. **Determine the Configuration File to Modify:**

   You need to edit the shell configuration file, depending on the shell you are using:

   - **Bash:** `~/.bashrc` or `~/.bash_profile` or `~/.profile` (usually `.bashrc` is more commonly used)

   - **Zsh:** `~/.zshrc`

   - **Fish:** `~/.config/fish/config.fish`

   If you are not sure, you can run `echo $SHELL` in the terminal to see which shell you are using.

3. **Edit the Configuration File:**

   Use a text editor (such as `nano`, `vim`, `emacs`) to open the corresponding configuration file. For example, use `nano` to edit `.bashrc`:

   ```bash
   nano ~/.bashrc
   ```

4. Add or Modify the `PATH` Environment Variable:

   In the configuration file, find the definition of the `PATH` environment variable. If not found, add the following line. Add the `senippet/target/release/senipt` directory to the `PATH` variable:

   ```bash
     # Add to .bashrc, .zshrc, or other shell configuration file
     export PATH="$PATH:$HOME/senippet/target/release/senipt"
   ```

   **Explanation:**

- `$PATH`: Represents the current value of the `PATH` variable.

- `:`: Used to separate different directories in the `PATH` variable.

- `$HOME`: Represents your `home` directory (e.g., `/home/yourusername`).

- `senippet/target/release/senipt`: The directory where your `senipt` executable file is located. Make sure to replace it with your actual project path. Here, it is assumed that your project is located in `$HOME/senippet`.

5. Save and Close the File:

   Save the modified configuration file and close the text editor.

6. Activate the Configuration:

   You need to reload the configuration file for the changes to take effect. Run the following command in the terminal:

   `Bash`

   ```bash
      source ~/.bashrc
   ```

   `Zsh`

   ```bash
      source ~/.zshrc
   ```

   `Fish`

   ```bash
      source ~/.config/fish/config.fish
   ```

<font color = "red">**Important Note:**</font>

- Make sure to replace `/senippet` with your actual project path if your project is not in the `$HOME` directory.
- These steps apply to Linux and macOS. The environment variable settings for Windows are different, please search for "windows set environment variable".

## <a name="usage"></a> Usage

### Basic Usage

**Using the Interactive Menu**

1.  Run the program directly in the terminal to enter the interactive menu interface:

    ```bash
    senippet
    ```

2.  The program will display the following menu options:

    ```bash
    🛠️  senippet CLI v0.1.1

    📂 senippet CLI
    1) Add Snippet/Template
    2) List Snippets/Templates
    3) Search by Tag
    4) Edit Snippets/Templates
    5) Delete Snippets/Templates
    6) Show Data Path
    7) Exit
    Choose option:
    ```

    Use the number keys to select the corresponding function and press Enter to confirm.

**Using Command-Line Arguments**

The program supports quick execution of operations directly through command-line arguments:

**add Command:**

`--name <NAME> (or -n <NAME>)`: Required, the name of the code snippet.

`--content <CONTENT> (or -c <CONTENT>)`: Required, the content of the code snippet.

`--multiline <CONTENT>`: Optional, multi-line input.

`--tags <TAGS> (or -t <TAGS>)`: Optional, a comma-separated list of tags.

**list Command:**

Now, the list command displays concise information for all code snippets in a tabular format, including Short ID, Name, and Tags.

Output format:

```
│    ID    │ Name               │ Tags           │
├──────────┼────────────────────┼────────────────┤
│   sp001  │ My Awesome Prompt  │ tag1,tag2      │
│   sp002  │ Another Prompt     │ tag3,tag4,tag5 │
│   sp003  │ My Important Prompt│                │
│          │                    │                │
```

**search Command:**

`--tag <TAG> (or -t <TAG>)`: Required, search for snippets with a specific tag.

**edit Command:**

`--id <ID> (or -i <ID>)`: Required, the ID of the snippet to edit.

`--name <NAME> (or -n <NAME>)`: Optional, the new name.

`--content <CONTENT> (or -c <CONTENT>)`: Optional, the new content.

`--multiline <CONTENT>`: Optional, multi-line editing.

`--tags <TAGS> (or -t <TAGS>)`: Optional, the new tags.

**delete Command:**

`--id <ID> (or -i <ID>)`: Required, the ID of the snippet to delete.

**path Command:**

No additional parameters required, displays the data storage path.

## Usage Scenarios

- **Store frequently used code snippets:** For example, commonly used functions, classes, loop structures, etc.
- **Manage configuration file templates:** For example, Dockerfile, Kubernetes YAML files, Nginx configuration files, etc.
- **Store frequently used command-line scripts:** For example, scripts for deployment, building, and testing.
- **Quickly find and reuse code:** Quickly find the required code snippets or templates through tags.
- **Improve development efficiency:** Avoid repeatedly writing the same code and reduce the possibility of errors.

## Configuration File

Code snippets/template data is stored in a JSON file. The file is located in the following directory:

- `Linux: $HOME/.local/share/senippet/data`
- `macOS: $HOME/Library/Application Support/com.sentomk.senippet/data`
- `Windows: C:\Users\<Your User>\AppData\Roaming\sentomk\senippet\data`

## Contributing

Contributions are welcome! Please submit an Issue or Pull Request.
````
