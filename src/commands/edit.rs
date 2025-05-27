use crate::{models::Prompt, storage::{load_prompts, save_prompts}};
use std::io::{self, Write};
use colored::*;

pub fn execute() {
    let mut prompts = load_prompts();

    if prompts.is_empty() {
        println!("{}", "❌ No prompts available to edit.".red());
        return;
    }

    println!("{}", "Available Prompts:".bold());
    for (i, prompt) in prompts.iter().enumerate() {
        println!("{}. {} [{}]", i + 1, prompt.title, prompt.tags.join(", "));
    }

    print!("{}", "Enter the numbers of the prompts to edit (comma-separated, e.g., 1,3,5): ".yellow());
    io::stdout().flush().unwrap();

    let mut choices = String::new();
    io::stdin().read_line(&mut choices).unwrap();

    let indices_to_edit: Vec<usize> = choices
        .trim()
        .split(',')
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .collect();

    if indices_to_edit.is_empty() {
        println!("{}", "❌ No valid prompt numbers entered.".red());
        return;
    }

    // 操作选中的prompts
    for index in indices_to_edit {
        if index > 0 && index <= prompts.len() {
            let prompt_index = index - 1;
            edit_prompt(&mut prompts[prompt_index]);
        } else {
            println!("{}", format!("❌ Invalid prompt number: {}.", index).red());
        }
    }
    
    // 保存编辑的prompts
    if let Err(e) = save_prompts(&prompts) {
        eprintln!("{}", format!("❌ Failed to save prompts: {}", e).red());
    } else {
        println!("{}", "✅ Successfully edited prompts!".green());
    }
}

fn edit_prompt(prompt: &mut Prompt) {
    println!("{}", format!("Editing prompt: {}", prompt.title).bold());
    
    // 编辑 title
    println!("{}", format!("Current title: {}", prompt.title).dimmed()); // Show current value
    print!("{}", "Enter new title (leave empty to keep current): ".bold());
    io::stdout().flush().unwrap();
    let mut new_title = String::new();
    io::stdin().read_line(&mut new_title).unwrap();
    let new_title = new_title.trim();
    if !new_title.is_empty() {
        prompt.title = new_title.to_string();
    }
    
    // 编辑 tags
    println!("{}", format!("Current tags: {}", prompt.tags.join(", ")).dimmed()); // Show current value
    print!("{}", "Enter new tags (comma-separated, leave empty to keep current): ".bold());
    io::stdout().flush().unwrap();
    let mut new_tags = String::new();
    io::stdin().read_line(&mut new_tags).unwrap();
    let new_tags = new_tags.trim();
    if !new_tags.is_empty() {
        prompt.tags = new_tags.split(',').map(|s| s.trim().to_string()).collect();
    }
    
    // 编辑 content
    println!("{}", format!("Current content:\n{}", prompt.content).dimmed()); // Show current value
    print!("{}", "Enter new content (leave empty to keep current): ".bold());
    io::stdout().flush().unwrap();
    let mut new_content = String::new();
    io::stdin().read_line(&mut new_content).unwrap();
    let new_content = new_content.trim();
    if !new_content.is_empty() {
        prompt.content = new_content.to_string();
    }
}
