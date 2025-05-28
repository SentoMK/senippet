// src/main.rs
use std::io::{stdout};
use crossterm::terminal::ClearType;
use crossterm::{
    execute, cursor
};
use colored::*;
use clap::{ Parser, Subcommand };

mod models;
mod storage;
mod commands;

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
        #[clap(short, long, value_parser, help = "Prompt title")]
        title: Option<String>,
        #[clap(short, long, value_parser, help = "Prompt content")]
        content: Option<String>,
        #[clap(short, long, value_parser, help = "Comma-separated tags")]
        tags: Option<String>,
        #[clap(long, action, help = "Use multi-line input for content")]
        multiline: bool,
    },
    
    List,
    Search {
        #[clap(value_parser, help = "Tag to search for")]
        tag: String,
    },

    Edit,

    Delete,

    Info,

    Exit,
}

fn main() {
     let cli = Cli::parse();
     if let Some(command) = cli.command {
         match command {
             Commands::Add { title, content, tags, multiline } => {
                 commands::add::execute_with_params(title, content, tags, multiline);
             }
             Commands::List => {
                 commands::list::execute();
             }
             Commands::Search { tag } => {
                 commands::search::execute(&tag);
             }
             Commands::Edit => {
                 commands::edit::execute();
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
             let version_line = format!("🛠️  SENPROMPT CLI v{}", env!("CARGO_PKG_VERSION"));
             println!("{}", version_line.cyan());
             print!("\n");
             println!("{}", "📂 SENPROMPT CLI".bold());
             println!("{}", "1) Add Prompt".italic());
             println!("{}", "2) List Prompts".italic());
             println!("{}", "3) Search by Tag".italic());
             println!("{}", "4) Edit Prompts".italic());
             println!("{}", "5) Delete Prompts".italic());
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
    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();  // 重置光标到左上角
}
