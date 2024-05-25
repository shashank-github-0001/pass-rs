use chrono::Utc;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Password {
    uuid: Uuid,
    username: String,
    link: String,
    password: String,
}

impl Password {
    fn new(username: String, link: String, password: String) -> Password {
        Password {
            uuid: uuid::Uuid::new_v4(),
            username,
            link,
            password,
        }
    }
}

fn password() -> String {
    const ALPHANUMERIC: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const SIZE: usize = 20;
    let mut rng = rand::thread_rng();
    let mut chars: Vec<char> = ALPHANUMERIC.chars().collect();
    chars.shuffle(&mut rng);
    let selected_chars: String = chars[..SIZE].iter().cloned().collect();
    selected_chars
}

fn main() {
    let exe_path = env::current_exe().expect("not get the current exe path");
    let exe_dir = exe_path.parent().unwrap();
    let path: PathBuf = [exe_dir, Path::new("../../passwords.txt")].iter().collect();
    let mut passwords: Vec<Password> = Vec::new();
    let args: Vec<String> = std::env::args().collect();
    let contents = fs::read_to_string(path.clone()).expect("Something went wrong reading the file");

    //deserializing data
    if !contents.is_empty() {
        passwords = serde_json::from_str(&contents).expect("Failed to deserialize passwords list");
    }

    //finally the things that will be shown
    if args[1] == "--help" || args[1] == "-h" {
        println!("Usage:");
        println!("\tconfig [options]");
        println!("Options:");
        println!("\t--help, -h\tShow this help message");
        println!("\t--version, -v\tShow the version");
        println!("\t--list, -l\tList all configs");
        println!("\t--add, -a\tAdd a config");
        println!("\t--remove, -r\tRemove a config");
    } else if args[1] == "--version" || args[1] == "-v" {
        println!("version: 0.1.0");
    } else if args[1] == "--list" || args[1] == "-l" {
        if passwords.is_empty() {
            println!("No passwords found");
        } else {
            println!("========================================================================================================");
            for password in passwords.iter() {
                println!(
                    "uuid: {}   username: {}   link: {}   password: {}",
                    password.uuid, password.username, password.link, password.password
                );
            }
            println!("========================================================================================================");
        }
    } else if args[1] == "--add" || args[1] == "-a" {
        let password = Password::new(args[2].to_string(), args[3].to_string(), password());
        passwords.push(password);
    } else if args[1] == "--remove" || args[1] == "-r" {
        let mut temp_passwords: Vec<Password> = Vec::new();
        for password in passwords.iter() {
            if password.uuid != args[2].parse::<Uuid>().unwrap() {
                temp_passwords.push(password.clone());
            }
        }
        passwords = temp_passwords;
    } else {
        println!("usage: cargo run -- --add <description> or --remove <id> or --list");
    }

    //serializing data
    let serialized_data = serde_json::to_string(&passwords).expect("Failed to serialize task list");
    fs::write(path, serialized_data).expect("Failed to write to file");
    println!("execution successful");
    println!(
        "Current UTC time: {}",
        Utc::now().format("%Y-%m-%d %H:%M:%S")
    );
}
