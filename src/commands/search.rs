use crate::storage::load_prompts;

pub fn execute(tag: &str) {
    let prompts = load_prompts();
    let results: Vec<_> = prompts
        .into_iter()
        .filter(|p| p.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)))
        .collect();

    if results.is_empty() {
        println!("❌ No prompts found with tag '{}'", tag);
    } else {
        for prompt in results {
            println!("🔹 {}: {}\n{}\n", 
                prompt.title, 
                prompt.tags.join(", "), 
                prompt.content);
        }
    }
}
