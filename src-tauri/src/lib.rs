use std::{env, fs};
use std::env::current_dir;
use std::fmt::{Debug, Pointer};
use std::path::Path;
use std::time::UNIX_EPOCH;
use srtlib::Subtitles;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[derive(Debug)]
#[derive(serde::Serialize)]
struct Paths{
    current_path:String,
    files: Vec<File>
}
#[derive(Debug)]
#[derive(serde::Serialize)]
struct File{
    path:String,
    name:String,
    created:u128,
    len:u64
}
#[tauri::command]
fn  query_path(path:&str)-> Paths{

    let mut files:Vec<File> = Vec::new();
    let curr_dir = current_dir().unwrap();
    let mut dir= fs::read_dir(&curr_dir).unwrap();
    let mut current_path=String::from(curr_dir.to_str().unwrap());

    if path!="None" {
        match fs::read_dir(Path::new(path)) {
            Ok(x) => {
                dir=x
            }
            Err(_) => {
                let current_path = String::from(path);
                return Paths{current_path,files}
            }
        }
        current_path = String::from(path);
    }

    for entry in dir{
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();
        let path = String::from(entry.path().to_str().unwrap()).replace("\\\\","\\");
        let len = metadata.len();
        let created =  metadata.created().unwrap().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let name = entry.file_name().into_string().unwrap();
        files.push(File{path,name,created,len});
    }

    Paths{current_path,files}
}
#[tauri::command]
fn read_srt(path: &str) {
    let mut subs = Subtitles::parse_from_file(path,None).unwrap();
    for x in subs {
        println!("{}",x.text)
    }
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![query_path,read_srt])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
