// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{io::BufReader, fs::File};

use nbt::tag::{Tag, TagError};
use tauri::api::dialog::blocking::FileDialogBuilder;

mod nbt;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command] // I know this is broken right now, have to implement serialization for Tag
fn load_file() -> Result<Tag, Option<&'static str>> {
    match FileDialogBuilder::new().pick_file() {
        Some(file) => {
            let file = File::open(file).map_err(|_e| Some("Reading file"))?;
            let mut reader = BufReader::new(file);

            Tag::read(&mut reader).map_err(|e| match e {
                TagError::InvalidType => Some("Invalid tag type encountered"),
                TagError::InvalidUtf8(_) => Some("Invalid string encountered"),
                TagError::IoError(_) => Some("Reading data from file"),
            })
        },
        None => Err(None)
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, load_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
