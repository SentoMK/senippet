// src/commands/mod.rs
use colored::*;
use std::io::{ self, Write };
pub mod add;
pub mod list;
pub mod search;
pub mod info;
pub mod delete;
pub mod edit;

// 修改函数返回类型
pub fn handle_command() -> bool {
    print!("{}", "Choose option: ".yellow());
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => {
            add::execute();
            wait_for_enter();
        }
        "2" => {
            list::execute();
            wait_for_enter();
        }
        "3" => {
            print!("{}", "Enter tag: ".bold());
            io::stdout().flush().unwrap();
            let mut tag = String::new();
            io::stdin().read_line(&mut tag).unwrap();
            search::execute(tag.trim());
            wait_for_enter();
        }
        "4" => {
            edit::execute();
            wait_for_enter();
        }
        "5" => {
            delete::execute();
            wait_for_enter();
        }
        "6" => {
            info::show_data_path();
            wait_for_enter();
        }
        "7" => {
            return true; // 触发退出
        }
        _ => {
            println!("{}", "❌ Invalid option, please try again.".red());
            wait_for_enter();
        }
    }
    
    false // 默认继续循环
}

// 新增等待函数
fn wait_for_enter() {
    let mut dummy = String::new();
    println!("\nPress Enter to continue...");
    println!("{}", "Or press Ctrl-c to force exit...".bright_yellow());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dummy).unwrap();
}
