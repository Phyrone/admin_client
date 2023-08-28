// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(clippy::unwrap_used, clippy::print_stdout)]

use clap::Parser;
use rand::{Rng, thread_rng};
use serde::{Deserialize, Serialize};

use crate::startup::StartupParams;
use crate::utils::random_alphanumeric_string;

mod shell;
mod startup;
mod utils;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize, Deserialize, Clone)]
enum HiBack {
    #[serde(rename = "test1")]
    Test1(String),
    #[serde(rename = "test2")]
    Test2(Test2Content),
}
#[derive(Serialize, Deserialize, Clone)]
struct Test2Content {
    x: i32,
    y: i32,
    z: u16,
    pitch: f32,
    yaw: f32,
}

#[tauri::command]
async fn hi() -> HiBack {
    if rand::random() {
        let random_string = random_alphanumeric_string(thread_rng().gen_range(10..30));
        HiBack::Test1(random_string)
    } else {
        HiBack::Test2(Test2Content {
            x: rand::random(),
            y: rand::random(),
            z: rand::random(),
            pitch: rand::random(),
            yaw: rand::random(),
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params = StartupParams::parse();
    startup::init_logger(params.log_level);

    tauri::Builder::default()
        .setup(|_application| Ok(()))
        .invoke_handler(tauri::generate_handler![greet, hi])
        .run(tauri::generate_context!())?;

    Ok(())
}
