use crate::components::cpu_chart::{CpuChart, CpuChartProps};
use crate::stores::process::process_store;
use leptos::{
    component, create_effect, create_signal, view, IntoView, Scope, SignalGet, SignalUpdate,
};
use wasm_bindgen::JsValue;

use crate::api::Process;

#[component]
pub fn ProcessDetails(cx: Scope, process: Process) -> impl IntoView {
    let (values, set_values) = create_signal(cx, vec![JsValue::UNDEFINED; 180]);

    let store = process_store(cx, process.info.pid);

    create_effect(cx, move |_| {
        set_values.update(|old| {
            old.remove(0);
            old.push(match store.get() {
                None => JsValue::UNDEFINED,
                Some(p) => p.resources.cpu_usage.into(),
            });
        });
    });

    view! { cx,
        <>
            <h1>{format!("Process details: {}", process.info.name)}</h1>
            <CpuChart values=values />
        </>
    }
}
