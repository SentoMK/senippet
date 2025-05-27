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