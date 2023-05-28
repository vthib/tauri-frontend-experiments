// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Mutex,
};

use serde::Serialize;
use sysinfo::{CpuExt, PidExt, ProcessExt, System, SystemExt, UserExt};
use tauri::{AppHandle, Manager, Window};

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
    sys: Arc<Mutex<System>>,
    global_cpu_usage: StateGlobalCpuUsage,
}
struct StateGlobalCpuUsage {
    nb_subscribers: AtomicU64,
    thread: Mutex<Option<CpuUsageThread>>,
}

struct CpuUsageThread {
    stop_notifier: Arc<AtomicBool>,
}

impl StateGlobalCpuUsage {
    fn new() -> Self {
        Self {
            nb_subscribers: AtomicU64::new(0),
            thread: Mutex::new(None),
        }
    }

    fn subscribe(&self, sys: Arc<Mutex<System>>, app_handle: AppHandle) {
        self.nb_subscribers.fetch_add(1, Ordering::SeqCst);

        let mut thread = self.thread.lock().unwrap();
        if thread.is_none() {
            let stop_notifier = Arc::new(AtomicBool::new(false));
            let stopper = Arc::clone(&stop_notifier);
            std::thread::spawn(move || {
                while !stopper.load(Ordering::SeqCst) {
                    {
                        let mut sys = sys.lock().unwrap();
                        sys.refresh_cpu();
                        println!("emitting {}", sys.global_cpu_info().cpu_usage() as u32);
                        app_handle
                            .emit_all("global_cpu_usage", sys.global_cpu_info().cpu_usage() as u32)
                            .unwrap();
                    }
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            });
            *thread = Some(CpuUsageThread { stop_notifier });
        }
    }

    fn unsubscribe(&self) {
        let prev = self.nb_subscribers.fetch_sub(1, Ordering::SeqCst);
        if prev == 1 {
            let thread = self.thread.lock().unwrap().take();
            if let Some(thread) = thread {
                thread.stop_notifier.store(true, Ordering::SeqCst);
            }
        }
    }
}

#[tauri::command]
fn get_process(state: tauri::State<State>, pid: u32) -> Option<Process> {
    println!("get process {}", pid);
    let pid = sysinfo::Pid::from_u32(pid);

    let mut sys = state.sys.lock().unwrap();
    sys.refresh_process(pid);

    let nb_cpus = sys.cpus().len() as u32;

    sys.process(pid)
        .map(|proc| convert_process(proc, &sys, nb_cpus))
}

fn convert_process(proc: &sysinfo::Process, sys: &System, nb_cpus: u32) -> Process {
    Process {
        info: ProcessInfo {
            pid: proc.pid().as_u32(),
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
    }
}

#[tauri::command]
fn get_processes(state: tauri::State<State>) -> Vec<Process> {
    println!("get processes");
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_processes();

    let nb_cpus = sys.cpus().len() as u32;

    sys.processes()
        .values()
        .map(|proc| convert_process(proc, &sys, nb_cpus))
        .collect()
}

#[tauri::command]
fn subscribe_global_cpu_usage(state: tauri::State<State>, window: Window) {
    println!("subscribe global cpu!");
    state
        .global_cpu_usage
        .subscribe(Arc::clone(&state.sys), window.app_handle());
}

#[tauri::command]
fn unsubscribe_global_cpu_usage(state: tauri::State<State>) {
    println!("unsubscribe global cpu!");
    state.global_cpu_usage.unsubscribe();
}

fn main() {
    let state = State {
        sys: Arc::new(Mutex::new(System::new_all())),
        global_cpu_usage: StateGlobalCpuUsage::new(),
    };
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_process,
            get_processes,
            subscribe_global_cpu_usage,
            unsubscribe_global_cpu_usage
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
