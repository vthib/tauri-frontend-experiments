use leptos::set_interval_with_handle;
use leptos::{create_signal, on_cleanup, spawn_local, ReadSignal, Scope, SignalSet};

use crate::api::{get_process, Process};

pub fn process_store(cx: Scope, pid: u32) -> ReadSignal<Option<Process>> {
    let (process, set_process) = create_signal(cx, None);

    let handle = set_interval_with_handle(
        move || {
            spawn_local(async move {
                let proc = get_process(pid).await;
                set_process.set(proc);
            });
        },
        std::time::Duration::from_secs(1),
    )
    .unwrap();
    // FIXME: This cleanup does not appear to work
    on_cleanup(cx, move || {
        handle.clear();
    });

    process
}
