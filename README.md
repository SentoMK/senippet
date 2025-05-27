# SENPROMPT

`current version: 0.1.0`

一个用 Rust 构建的 CLI 代码片段/模板 管理工具，旨在帮助您组织、存储和使用各种常用的代码片段、配置模板、或命令行脚本，以便在软件开发中提高效率。

## 特性

- 添加代码片段/模板： 轻松添加带有标题、内容和标签的代码片段或模板。

- 列出代码片段/模板： 查看所有已保存的代码片段或模板。

- 按标签搜索： 快速查找与特定标签相关的代码片段或模板。

- 编辑代码片段/模板： 修改现有的代码片段或模板，包括标题、内容和标签。

- 删除代码片段/模板： 移除不再需要的代码片段或模板。

- 显示数据路径： 查找存储代码片段或模板数据的目录。

- 版本信息： 显示工具的版本。

- 用户友好的界面： 彩色输出和清晰的菜单选项。

## 安装

### 前提条件

- 安装 [Rust](https://www.rust-lang.org/tools/install) 和 Cargo (Rust 包管理器)。

### 从 Crates.io 安装

```
cargo install senprompt
```

### 从源代码安装

1. 克隆仓库

```
git clone <https://github.com/SentoMK/senprompt>
cd senprompt
```

2. 构建并安装

```
cargo install --path .
```

## 使用方法

1. 在终端中运行 senprompt 命令。

```
senprompt
```

2. 按照菜单提示进行操作：

```
🛠️  SENPROMPT CLI v0.1.0


📂 SENPROMPT CLI
1) Add Snippet/Template
2) List Snippets/Templates
3) Search by Tag
4) Edit Snippets/Templates
5) Delete Snippets/Templates
6) Show Data Path
6) Exit
Choose option:
```

- 1. **Add Snippet/Template：**

     输入代码片段/模板的标题、内容和标签（以逗号分隔）。

- 2. **List Snippets/Templates：**

     显示所有已保存的代码片段/模板，包括序号、标题和标签。

- 3. **Search by Tag：**

     输入要搜索的标签，显示匹配的代码片段/模板。

- 4. **Edit Snippets/Templates：**

     选择要编辑的代码片段/模板的序号，然后修改标题、内容或标签。

- 5. **Delete Snippets/Templates：**

     选择要删除的代码片段/模板的序号，可以一次删除多个。

- 6. **Show Data Path：**

     显示存储代码片段/模板数据的目录的路径。

- 7. **Exit：**

     退出程序。

## 使用场景

- **存储常用的代码片段：** 例如，常用的函数、类、循环结构等。

- **管理配置文件模板：** 例如，Dockerfile、Kubernetes YAML 文件、Nginx 配置文件等。

- **存储常用命令行脚本：** 例如，用于部署、构建、测试的脚本。

- **快速查找和复用代码：** 通过标签快速找到需要的代码片段或模板。

- **提高开发效率：** 避免重复编写相同的代码，减少出错的可能性

## 详细操作说明

### 添加代码片段/模板

选择 `1) Add Snippet/Template` 后，会依次提示你输入标题、内容和标签。标签之间用逗号分隔。

### 列出代码片段/模板

选择 `2) List Snippets/Templates` 后，会列出所有已保存的代码片段/模板，包括序号、标题和标签。

### 按标签搜索

选择 `3) Search by Tag` 后，输入要搜索的标签。程序会显示所有包含该标签的代码片段/模板。

### 编辑代码片段/模板

选择 `4) Edit Snippets/Templates` 后，会列出所有代码片段/模板。输入要编辑的代码片段/模板的序号（多个序号用逗号分隔）。然后，你可以修改每个代码片段/模板的标题、内容和标签。如果不想修改某个字段，直接回车即可保留原值。

### 删除代码片段/模板

选择 `5) Delete Snippets/Templates` 后，会列出所有代码片段/模板。输入要删除的代码片段/模板的序号（多个序号用逗号分隔）。程序会删除指定的代码片段/模板。

### 显示数据路径

选择 `6) Show Data Path` 后，会显示存储代码片段/模板数据的目录的路径。你可以在该目录下找到存储代码片段/模板的 JSON 文件。

## 配置文件

代码片段/模板数据存储在 JSON 文件中。该文件位于以下目录：

- `Linux： $HOME/.local/share/senprompt/data`

- `macOS： $HOME/Library/Application Support/com.sentomk.senprompt/data`

- `Windows： C:\Users\<Your User>\AppData\Roaming\sentomk\senprompt\data`

## 贡献

欢迎贡献！请提交 Issue 或 Pull Request。
