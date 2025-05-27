use crate::storage;
use colored::*;

pub fn show_data_path() {
    match storage::get_data_path() {
        Some(path) => println!("📁 Data location: {}", path.display()),
        None => eprintln!("{}", "❌ Failed to determine data directory".red()),
    }
}
