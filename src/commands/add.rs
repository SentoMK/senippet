use crate::{models::Prompt, storage::save_prompts};
use std::io::{self, Write};
use colored::*;

pub fn execute() {
    let mut title = String::new();
    let mut content = String::new();
    let mut tags = String::new();

    print!("{}", "📝 Title: ".bold());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut title).unwrap();

    print!("{}", "💬 Prompt content: ".bold());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut content).unwrap();

    print!("{}", "🏷️ Tags (comma-separated): ".bold());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut tags).unwrap();

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
