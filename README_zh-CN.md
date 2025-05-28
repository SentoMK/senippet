# SENIPPET

[English](README.md)

`current version: 0.1.1`

一个用 Rust 构建的 CLI 代码片段（模板）/ Prompts 管理工具，旨在帮助您组织、存储和使用各种常用的代码片段、配置模板、或命令行脚本，以便在软件开发中提高效率。

## 特性/功能

- 当前版本增加了强大的添加/编辑模板的功能，支持两种编辑模式：

  1.  **单行编辑：** 允许用户在单个文本行中进行快速、便捷的修改。 非常适合简单的变量替换、属性调整等操作。

  2.  **多行编辑：** 提供更灵活的编辑环境，支持处理包含多行文本的模板。 适用于复杂的代码片段、配置文件、或者需要进行大规模修改的文本内容。

- 可通过命令行直接操作。用法详见[使用说明](#usage)

## 安装

**对于非 Rust 用户，可直接在 [Release](https://github.com/SentoMK/senippet/releases)界面下载对应操作系统的可执行文件。**

### 前提条件

- 安装 [Rust](https://www.rust-lang.org/tools/install) 和 Cargo (Rust 包管理器)。

### 从 Crates.io 安装

```bash
cargo install senippet
```

### 从源代码安装

1. 克隆仓库

```bash
git clone <https://github.com/SentoMK/senippet>
cd senippet
```

2. 构建并安装

```bash
cargo install --path .
```

### 将 senpt 添加到环境变量 (PATH)

为了能够在任何目录下直接运行 `senpt` 命令，你需要将其添加到系统的环境变量 `PATH` 中。
以下以在 Linux/macOS 系统中的操作步骤为例。

1. **找到可执行文件:**

   确认 `senippet/target/release/senpt` 文件存在。 这是你的 `senpt` 可执行文件的位置。

2. **确定要修改的配置文件:**

   你需要编辑 shell 配置文件，具体取决于你使用的 shell：

   - **Bash:** `~/.bashrc` 或 `~/.bash_profile` 或 `~/.profile` (通常 `.bashrc` 更常用)

   - **Zsh:** `~/.zshrc`

   - **Fish:** `~/.config/fish/config.fish`

   如果你不确定，可以在终端中运行 `echo $SHELL` 来查看你正在使用的 shell。

3. **编辑配置文件:**

   使用文本编辑器 (如 `nano`, `vim`, `emacs`) 打开相应的配置文件。 例如，使用 `nano` 编辑 `.bashrc`：

   ```bash
   nano ~/.bashrc
   ```

4. 添加或修改 `PATH` 环境变量:

   在配置文件中，找到 `PATH` 环境变量的定义。 如果没有找到，请添加以下行。 将 `senippet/target/release/senpt` 目录添加到 `PATH` 变量中：

   ```bash
     # 添加到 .bashrc, .zshrc, 或其他 shell 配置文件
     export PATH="$PATH:$HOME/senippet/target/release/senpt"
   ```

   **解释：**

- `$PATH`: 表示当前 `PATH` 变量的值。

- `:`： 用于分隔 `PATH` 变量中的不同目录。

- `$HOME`: 表示你的 `home` 目录（例如 `/home/yourusername`）。

- `senippet/target/release/senpt`: 你的 `senpt` 可执行文件所在的目录。请确保替换为你的实际项目路径。这里假设你的项目位于 `$HOME/senippet`。

5. 保存并关闭文件:

   保存你修改的配置文件并关闭文本编辑器。

6. 激活配置:

   你需要重新加载配置文件以使更改生效。 在终端中运行以下命令：

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

<font color = "red">**重要提示:**</font>

- 确保将 `/senippet` 替换为你实际的项目路径，如果你的项目不在 `$HOME` 目录下。
- 这些步骤适用于 Linux 和 macOS。 Windows 的环境变量设置方式不同，请搜索 "windows 设置环境变量"。

## <a name="usage"></a> 使用方法

### 基本使用方式

**使用交互式菜单**

1. 在终端中直接运行程序即可进入交互式菜单界面：

   ```bash
   senippet
   ```

2. 程序会显示如下菜单选项：

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

   使用数字键选择对应功能，按回车确认。

**使用命令行参数**

程序支持直接通过命令行参数快速执行操作：

**add 命令：**

`--name <NAME> (或 -n <NAME>)`: 必须，代码片段的名称。

`--content <CONTENT> (或 -c <CONTENT>)`: 必须，代码片段的内容。

`--multiline <CONTENT>`: 可选，多行输入。

`--tags <TAGS> (或 -t <TAGS>)`: 可选，以逗号分隔的标签列表。

**list 命令：**

`无需额外参数，简单列出所有片段。`

**search 命令：**

`--tag <TAG> (或 -t <TAG>)`: 必须，搜索特定标签的片段。

**edit 命令：**

`--id <ID> (或 -i <ID>)`: 必须，要编辑的片段的 ID。

`--name <NAME> (或 -n <NAME>)`: 可选，新的名称。

`--content <CONTENT> (或 -c <CONTENT>)`: 可选，新的内容。

`--multiline <CONTENT>`: 可选，多行编辑。

`--tags <TAGS> (或 -t <TAGS>)`: 可选，新的标签。

**delete 命令：**

`--id <ID> (或 -i <ID>)`: 必须，要删除的片段的 ID。

**path 命令：**

`无需额外参数，显示数据存储路径。`

## 使用场景

- **存储常用的代码片段：** 例如，常用的函数、类、循环结构等。

- **管理配置文件模板：** 例如，Dockerfile、Kubernetes YAML 文件、Nginx 配置文件等。

- **存储常用命令行脚本：** 例如，用于部署、构建、测试的脚本。

- **快速查找和复用代码：** 通过标签快速找到需要的代码片段或模板。

- **提高开发效率：** 避免重复编写相同的代码，减少出错的可能性

## 配置文件

代码片段/模板数据存储在 JSON 文件中。该文件位于以下目录：

- `Linux： $HOME/.local/share/senippet/data`

- `macOS： $HOME/Library/Application Support/com.sentomk.senippet/data`

- `Windows： C:\Users\<Your User>\AppData\Roaming\sentomk\senippet\data`

## 贡献

欢迎贡献！请提交 Issue 或 Pull Request。
