use crate::storage::load_prompts;

pub fn execute() {
    let prompts = load_prompts();
    for (i, prompt) in prompts.iter().enumerate() {
        println!("{}. {} [{}]", i + 1, prompt.title, prompt.tags.join(", "));
    }
}
