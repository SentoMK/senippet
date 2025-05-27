use crate::storage::{load_prompts, save_prompts};
use std::io::{self, Write};
use colored::*;

pub fn execute() {
    let mut prompts = load_prompts();

    if prompts.is_empty() {
        println!("{}", "❌ No prompts available to delete.".red());
        return;
    }

    println!("{}", "Available Prompts:".bold());
    for (i, prompt) in prompts.iter().enumerate() {
        println!("{}. {} [{}]", i + 1, prompt.title, prompt.tags.join(", "));
    }

    print!("{}", "Enter the numbers of the prompts to delete (comma-separated, e.g., 1,3,5): ".yellow());
    io::stdout().flush().unwrap();

    let mut choices = String::new();
    io::stdin().read_line(&mut choices).unwrap();

    let indices_to_delete: Vec<usize> = choices
        .trim()
        .split(',')
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .collect();

    // 从大到小排序，防止删除时索引错位
    let mut sorted_indices = indices_to_delete;
    sorted_indices.sort_unstable_by(|a, b| b.cmp(a));

    let mut deleted_count = 0;
    for index in sorted_indices {
        if index > 0 && index <= prompts.len() {
            prompts.remove(index - 1);
            deleted_count += 1;
        } else {
            println!("{}", format!("❌ Invalid prompt number: {}.", index).red());
        }
    }

    if deleted_count > 0 {
        if let Ok(_) = save_prompts(&prompts) {
            println!("{}", format!("✅ Successfully deleted {} prompts!", deleted_count).green());
        } else {
            println!("{}", "❌ Failed to save prompts.".red());
        }
    } else {
        println!("{}", "❌ No prompts were deleted.".red());
    }
}

