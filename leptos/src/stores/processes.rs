use leptos::set_interval_with_handle;
use leptos::{create_signal, on_cleanup, spawn_local, ReadSignal, Scope, SignalSet};

use crate::api::{get_processes, Process};

pub fn processes_store(cx: Scope) -> ReadSignal<Vec<Process>> {
    let (processes, set_processes) = create_signal(cx, Vec::new());

    let handle = set_interval_with_handle(
        move || {
            spawn_local(async move {
                let procs = get_processes().await;
                set_processes.set(procs);
            });
        },
        std::time::Duration::from_secs(1),
    )
    .unwrap();
    on_cleanup(cx, move || {
        handle.clear();
    });

    processes
}
