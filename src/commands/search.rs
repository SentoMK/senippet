use crate::storage::load_prompts;
use colored::*;

pub fn execute(tag: &str) {
    let prompts = load_prompts();
    let results: Vec<_> = prompts
        .into_iter()
        .filter(|p| p.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)))
        .collect();

    if results.is_empty() {
        print!("{}", "❌ No prompts found with tag: ".red()); 
        println!("{}", tag.bright_green().to_string())
    } else {
        for prompt in results {
            println!("🔹 {}: {}\n{}\n", 
                prompt.title, 
                prompt.tags.join(", "), 
                prompt.content);
        }
    }
}
