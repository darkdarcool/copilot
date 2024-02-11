#![allow(dead_code)]

use chrono;
use homedir::get_my_home;
use rand;
use rand::Rng;
use uuid::Uuid;

pub(crate) fn generate_random_uuid4() -> String {
    Uuid::new_v4().to_string()
}

// rust equivlent of getRandomUuidv4() + String(Math.round(new Date().getTime()));
pub(crate) fn generate_vscode_session_id() -> String {
    format!(
        "{}{}",
        generate_random_uuid4(),
        chrono::Utc::now().timestamp_millis()
    )
}

pub(crate) fn random_hex_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut s = String::new();
    for _ in 0..length {
        s.push_str(&format!("{:x}", rng.gen::<u8>()));
    }
    s
}

fn get_config_path() -> String {
    let home = get_my_home().unwrap().unwrap();
    format!("{}/.config/copilot", home.to_str().unwrap())
}

pub(crate) fn append_to_file(file_path: &str, content: &str) {
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .unwrap();

    file.write_all(content.as_bytes()).unwrap();
}

pub(crate) fn read_config_file() -> String {
    // const CACHE_PATH = path.join(process.env.HOME || "~", ".config", ".copilot");
    let cache_path = get_config_path();
    let config_path = format!("{}/config.json", get_config_path());

    println!("Writing token to config file: {}", config_path);

    // create if not exists
    std::fs::create_dir_all(&cache_path).unwrap();

    let config = std::fs::read_to_string(config_path).unwrap_or("".to_string());
    config
}

pub(crate) fn write_token_to_config_file(token: &String) {
    let cache_path = get_config_path();
    let config_path = format!("{}/config.json", get_config_path());

    // create if not exists
    std::fs::create_dir_all(&cache_path).unwrap();

    std::fs::write(config_path, token).unwrap();
}
