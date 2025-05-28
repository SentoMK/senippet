// src/commands/add.rs
use crate::{models::Prompt, storage::save_prompts};
use std::io::{self, Write};
use colored::*;

pub fn execute_with_params(
    title: Option<String>,
    content: Option<String>,
    tags: Option<String>,
    multiline: bool,
) {
    let title = title.unwrap_or_else(|| {
        print!("{}", "📝 Title: ".bold());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
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
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    });

    let prompt = Prompt::new(
        title,
        content,
        tags.split(',').map(|s| s.to_string()).collect()
    );

    let mut prompts = crate::storage::load_prompts();
    prompts.push(prompt);
    let _ = save_prompts(&prompts);
    println!("{}", "✅ Prompt added!".green());
}


fn get_single_line_content() -> String {
    let mut content = String::new();
    print!("{}", "💬 Prompt content (single-line): ".bold());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut content).unwrap();
    content
}

fn get_multi_line_content() -> String {
    println!("{}", "💬 Prompt content (multi-line, enter '.' on a new line to finish):".bold());
    let mut content = String::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim(); // 删除首尾空格和换行符
        if line == "." {
            break;
        }
        content.push_str(&line);
        content.push_str("\n"); // 保留换行
    }
     content
}

pub fn execute() { // 保留原有的 execute 函数，如果没有参数，则运行原有的逻辑
 execute_with_params(None, None, None, false);
}

