use crate::models::Prompt;
use directories::ProjectDirs;
use std::fs::{self};
use std::path::PathBuf;

// 获取配置文件路径
pub fn get_data_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "sentomk", "senippet")
        .map(|proj_dirs| proj_dirs.data_dir().to_path_buf())
}

// 确保目录存在
fn ensure_data_dir() -> std::io::Result<()> {
    if let Some(path) = get_data_path() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}

pub fn load_prompts() -> Vec<Prompt> {
    let path = match get_data_path() {
        Some(p) => p,
        None => return Vec::new(),
    };

    match fs::read_to_string(&path) {
        Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

pub fn save_prompts(prompts: &[Prompt]) -> Result<(), Box<dyn std::error::Error>> {
    // Return Result
    let path = match get_data_path() {
        Some(p) => p,
        None => return Err(From::from("Could not determine data directory")), // Return Error
    };
    ensure_data_dir()?;
    let json = serde_json::to_string_pretty(prompts)?;
    fs::write(path, json)?;
    Ok(())
}

// pub fn migrate_data() {
//    let old_path = PathBuf::from("prompts.json");
//    let new_path = match get_data_path() {
//        Some(p) => p,
//        None => return,
//    };

//    if old_path.exists() && !new_path.exists() {
//        if let Err(e) = fs::rename(&old_path, &new_path) {
//            eprintln!("Migration failed: {}", e);
//        }
//    }
//}
