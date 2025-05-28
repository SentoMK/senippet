// src/commands/add.rs
use crate::{models::Prompt, storage::save_prompts};
use colored::*;
use rand::Rng;
use std::io::{self, Write};

pub fn execute_with_params(
    name: Option<String>,
    content: Option<String>,
    tags: Option<String>,
    multiline: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // 返回 Result
    let name = name.unwrap_or_else(|| {
        print!("{}", "📝 Name (Title): ".bold());
        io::stdout().flush().expect("Failed to flush stdout"); // 更友好的错误处理
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line"); // 更友好的错误处理
        input.trim().to_string()
    });

    let content = content.unwrap_or_else(|| {
        if multiline {
            get_multi_line_content()
        } else {
            get_single_line_content()
        }
    });

    let tags = tags.unwrap_or_else(|| {
        print!("{}", "🏷️ Tags (comma-separated): ".bold());
        io::stdout().flush().expect("Failed to flush stdout"); // 更友好的错误处理
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line"); // 更友好的错误处理
        input.trim().to_string()
    });

    let mut prompts = crate::storage::load_prompts();
    let mut short_id = generate_short_id();

    while prompts.iter().any(|p| p.short_id == short_id) {
        short_id = generate_short_id();
    }
    let prompt = Prompt::new(
        name,
        content,
        tags.split(',').map(|s| s.to_string()).collect(),
        short_id.clone(),
    );

    prompts.push(prompt);
    save_prompts(&prompts)?; // 使用 ? 传播错误

    println!("{}", "✅ Prompt added!".green());
    println!(
        "{}",
        format!("UUID: {}", &prompts.last().unwrap().uuid)
            .bold()
            .green()
    ); //  从prompts取，避免clone
    println!("{}", format!("ID: {}", short_id).bold().green());
    Ok(()) // 返回 Ok
}

// 生成 short_id
fn generate_short_id() -> String {
    let mut rng = rand::rng();
    let id: u32 = rng.random_range(10000..99999); // 5 位数字
    id.to_string()
}

fn get_single_line_content() -> String {
    let mut content = String::new();
    print!("{}", "💬 Prompt content (single-line): ".bold());
    io::stdout().flush().expect("Failed to flush stdout"); // 更友好的错误处理
    io::stdin()
        .read_line(&mut content)
        .expect("Failed to read line"); // 更友好的错误处理
    content
}

fn get_multi_line_content() -> String {
    println!(
        "{}",
        "💬 Prompt content (multi-line, enter '.' on a new line to finish):".bold()
    );
    let mut content = String::new();
    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line"); // 更友好的错误处理
        let line = line.trim(); // 删除首尾空格和换行符
        if line == "." {
            break;
        }
        content.push_str(&line);
        content.push_str("\n"); // 保留换行
    }
    content
}

pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
    // 返回 Result
    // 保留原有的 execute 函数，如果没有参数，则运行原有的逻辑
    execute_with_params(None, None, None, false)
}
