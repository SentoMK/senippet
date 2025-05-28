// src/main.rs
use clap::{Parser, Subcommand};
use colored::*;
use crossterm::terminal::ClearType;
use crossterm::{cursor, execute};
use std::io::stdout;

mod commands;
mod models;
mod storage;

#[derive(Parser)]
#[clap(
    name = "senpt",
    version = env!("CARGO_PKG_VERSION"),
    author = "SentoMK",
    about = "A CLI tool for managing prompts"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[clap(short = 'n', long = "name", value_parser, help = "Snippet name")]
        name: Option<String>,
        #[clap(short = 'c', long = "content", value_parser, help = "Snippet content")]
        content: Option<String>,
        #[clap(
            short = 't',
            long = "tags",
            value_parser,
            help = "Comma-separated tags"
        )]
        tags: Option<String>,
        #[clap(long = "multiline", action, help = "Use multi-line input for content")]
        multiline: bool,
    },

    List,

    Search {
        #[clap(value_parser, help = "Tag to search for")]
        tag: String,
    },

    Edit {
        #[clap(
            short = 'i',
            long = "id",
            value_parser,
            help = "Snippet ID to edit",
            required = true
        )]
        id: String,
        #[clap(short = 'n', long = "name", value_parser, help = "New snippet name")]
        name: Option<String>,
        #[clap(
            short = 'c',
            long = "content",
            value_parser,
            help = "New snippet content"
        )]
        content: Option<String>,
        #[clap(long = "multiline", action, help = "Use multi-line input for content")]
        multiline: bool,
        #[clap(
            short = 't',
            long = "tags",
            value_parser,
            help = "New comma-separated tags"
        )]
        tags: Option<String>,
    },

    Delete,

    Info,

    Exit,
}

fn main() {
    let cli = Cli::parse();
    if let Some(command) = cli.command {
        match command {
            Commands::Add {
                name,
                content,
                tags,
                multiline,
            } => {
                let _ = commands::add::execute_with_params(name, content, tags, multiline);
            }
            Commands::List => {
                let _ = commands::list::execute();
            }
            Commands::Search { tag } => {
                commands::search::execute(&tag);
            }
            Commands::Edit {
                id,
                name,
                content,
                multiline,
                tags,
            } => {
                commands::edit::execute(&id, name, content, multiline, tags);
            }
            Commands::Delete => {
                commands::delete::execute();
            }
            Commands::Info => {
                commands::info::show_data_path();
            }
            Commands::Exit => {
                std::process::exit(0);
            }
        }
    } else {
        // 添加主循环（如果没有任何命令）
        loop {
            clear_screen();
            let version_line = format!("🛠️  SENIPPET CLI v{}", env!("CARGO_PKG_VERSION"));
            println!("{}", version_line.cyan());
            print!("\n");
            println!("{}", "📂 SENIPPET CLI".bold());
            println!("{}", "1) Add Snippet".italic());
            println!("{}", "2) List Snippets".italic());
            println!("{}", "3) Search by Tag".italic());
            println!("{}", "4) Edit Snippets".italic());
            println!("{}", "5) Delete Snippets".italic());
            println!("{}", "6) Show Data Path".italic());
            println!("{}", "7) Exit".italic());
            // 处理命令并获取退出标志
            if commands::handle_command() {
                break;
            }
            // 添加操作间隔
            println!("\n─────────────────────");
        }
        println!("👋 Goodbye!");
    }
}

fn clear_screen() {
    let mut stdout = stdout();
    execute!(stdout, crossterm::terminal::Clear(ClearType::All)).unwrap();
    execute!(stdout, cursor::MoveTo(0, 0)).unwrap(); // 重置光标到左上角
}
