// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use serde::Serialize;
use sysinfo::{PidExt, ProcessExt, System, SystemExt, UserExt};

#[derive(Serialize)]
struct Process {
    info: ProcessInfo,
    resources: ProcessResources,
}

#[derive(Serialize)]
struct ProcessInfo {
    pid: u32,
    name: String,
    user: Option<String>,
    status: String,
}

#[derive(Serialize)]
struct ProcessResources {
    memory: u64,
    virtual_memory: u64,
    cpu_usage: u32,
}

struct State {
    sys: Mutex<System>,
}

#[tauri::command]
fn get_processes(state: tauri::State<State>) -> Vec<Process> {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_processes();

    let nb_cpus = sys.cpus().len() as u32;

    sys.processes()
        .iter()
        .map(|(pid, proc)| Process {
            info: ProcessInfo {
                pid: pid.as_u32(),
                name: proc.name().to_owned(),
                user: proc
                    .user_id()
                    .and_then(|uid| sys.get_user_by_id(uid))
                    .map(UserExt::name)
                    .map(ToOwned::to_owned),
                status: format!("{:?}", proc.status()),
            },
            resources: ProcessResources {
                memory: proc.memory(),
                virtual_memory: proc.virtual_memory(),
                cpu_usage: (proc.cpu_usage() * 100.0) as u32 / nb_cpus,
            },
        })
        .collect()
}

fn main() {
    let state = State {
        sys: Mutex::new(System::new_all()),
    };
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![get_processes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
