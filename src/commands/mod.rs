// src/commands/mod.rs
pub mod add;
pub mod list;
pub mod search;
pub mod info;

use std::io::{self, Write};

// 修改函数返回类型
pub fn handle_command() -> bool {
    print!("Choose option: ");
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
            print!("Enter tag: ");
            io::stdout().flush().unwrap();
            let mut tag = String::new();
            io::stdin().read_line(&mut tag).unwrap();
            search::execute(tag.trim());
            wait_for_enter();
        }
        "4" => {
            info::show_data_path();
            wait_for_enter();
        }
        "5" => {
            return true; // 触发退出
        }
        _ => {
            println!("❌ Invalid option, please try again.");
            wait_for_enter();
        }
    }
    
    false // 默认继续循环
}

// 新增等待函数
fn wait_for_enter() {
    let mut dummy = String::new();
    println!("\nPress Enter to continue...");
    println!("Or press C-c to exit");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dummy).unwrap();
}
