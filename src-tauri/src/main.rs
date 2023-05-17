// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;

use dll_syringe::{process::OwnedProcess, Syringe};

mod task;
use task::TasksList;

#[tauri::command]
fn get_tasks_list(filter: &str) -> TasksList {
    TasksList::new(filter)
}

#[tauri::command]
fn inject_dll(pid: u32, path_to_dll: &Path) -> String {
    if !path_to_dll.exists() {
        return format!("DLL ({}) does not exists.", path_to_dll.display());
    }

    let target = match OwnedProcess::from_pid(pid) {
        Ok(t) => t,
        Err(e) => return e.to_string(),
    };

    match Syringe::for_process(target).inject(path_to_dll) {
        Ok(_) => String::new(),
        Err(e) => return e.to_string(),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_tasks_list, inject_dll])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
