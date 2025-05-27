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

    print!("{}", "Enter the number of the prompt to delete: ".yellow());
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    let index = match choice.trim().parse::<usize>() {
        Ok(index) => index,
        Err(_) => {
            println!("{}", "❌ Invalid input. Please enter a number.".red());
            return;
        }
    };

    if index > 0 && index <= prompts.len() {
        prompts.remove(index - 1);
        if let Ok(_) = save_prompts(&prompts) {
            println!("{}", "✅ Prompt deleted successfully!".green());
        } else {
            println!("{}", "❌ Failed to save prompts.".red());
        }
    } else {
        println!("{}", "❌ Invalid prompt number.".red());
    }
}

