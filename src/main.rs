// src/main.rs
use std::io::{stdout};
use crossterm::terminal::ClearType;
use crossterm::{
    execute, cursor
};
use colored::*;

mod models;
mod storage;
mod commands;

fn main() { 
    // 添加主循环
    loop {
        clear_screen();
        let version_line = format!("🛠️  SENPROMPT CLI v{}", env!("CARGO_PKG_VERSION"));        
        println!("{}", version_line.cyan());
        print!("\n");
        println!("{}", "📂 SENPROMPT CLI".bold());
        println!("{}", "1) Add Prompt".italic());
        println!("{}", "2) List Prompts".italic());
        println!("{}", "3) Search by Tag".italic());
        println!("{}", "4) Show Data Path".italic());
        println!("{}", "5) Exit".italic());

        // 处理命令并获取退出标志
        if commands::handle_command() {
            break;
        }

        // 添加操作间隔
        println!("\n─────────────────────");
    }
    println!("👋 Goodbye!");
}

fn clear_screen() {
    let mut stdout = stdout();
    execute!(stdout, crossterm::terminal::Clear(ClearType::All)).unwrap();
    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();  // 重置光标到左上角
}
