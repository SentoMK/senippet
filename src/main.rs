// src/main.rs
use std::io::{stdout};
use crossterm::terminal::ClearType;
use crossterm::{
    execute, cursor
};
mod models;
mod storage;
mod commands;

fn main() { 
    // 添加主循环
    loop {
        clear_screen();
        println!("📂 SENPROMPT CLI");
        println!("1) Add Prompt");
        println!("2) List Prompts");
        println!("3) Search by Tag");
        println!("4) Show Data Path");
        println!("5) Exit");

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
