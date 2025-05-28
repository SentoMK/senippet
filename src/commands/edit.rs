// commands/edit.rs
use crate::{models::Prompt, storage::{load_prompts, save_prompts}};
use std::io;
use colored::*;

pub fn execute(
    id: &String,
    name: Option<String>,
    content: Option<String>,
    multiline: bool,
    tags: Option<String>,
) {
    let mut prompts = load_prompts();

    // 根据 ID 查找要编辑的 Prompt
    let mut found = false;
    for prompt in &mut prompts {
        if &prompt.id == id {
            found = true;
            edit_prompt(prompt, name, content, multiline, tags);
            break;
        }
    }

    if !found {
        println!("{}", format!("❌ No prompt found with ID: {}", id).red());
        return;
    }

    // 保存修改后的 Prompts
    if let Err(e) = save_prompts(&prompts) {
        eprintln!("{}", format!("❌ Failed to save prompts: {}", e).red());
    } else {
        println!("{}", "✅ Successfully edited prompt!".green());
    }
}

fn edit_prompt(
    prompt: &mut Prompt,
    new_name: Option<String>,
    new_content: Option<String>,
    multiline: bool,
    new_tags: Option<String>,
) {
    println!("{}", format!("Editing prompt: {}", prompt.title).bold());

    // 编辑 title
    if let Some(name) = new_name {
        println!("{}", format!("Current title: {}", prompt.title).dimmed());
        prompt.title = name;
    }

    // 编辑 tags
    if let Some(tags) = new_tags {
        println!("{}", format!("Current tags: {}", prompt.tags.join(", ")).dimmed());
        prompt.tags = tags.split(',').map(|s| s.trim().to_string()).collect();
    }

    // 编辑 content
    if let Some(content) = new_content {
        println!("{}", format!("Current content:\n{}", prompt.content).dimmed());
        prompt.content = content;
    } else if multiline {
        println!("{}", format!("Current content:\n{}", prompt.content).dimmed());
        println!("{}", "Enter new content (multi-line, enter '.' on a new line to finish, leave empty to keep current):".bold());
        let mut new_content = String::new();
        loop {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            let line = line.trim();
            if line == "." {
                break;
            }
            if line.is_empty() && new_content.is_empty() {
                // 如果用户直接输入 ".", 并且当前内容为空，则保留原来的内容
                return;
            }
            new_content.push_str(&line);
            new_content.push_str("\n");
        }
        if !new_content.is_empty() {
            prompt.content = new_content.to_string();
        }
    }
}
