use leptos::{component, create_signal, log, spawn_local, view, IntoView, Scope, SignalUpdate};
use wasm_bindgen::prelude::*;

use crate::api::subscribe_to_global_cpu_usage;
use crate::components::cpu_chart::{CpuChart, CpuChartProps};

#[component]
pub fn Resources(cx: Scope) -> impl IntoView {
    let (history, set_history) = create_signal(cx, vec![JsValue::UNDEFINED; 180]);

    spawn_local(async move {
        subscribe_to_global_cpu_usage(cx, move |v| {
            set_history.update(|vec| {
                vec.remove(0);
                log!("got new value {:?}, new vec: {:?}", v, vec);
                vec.push(v.into());
            });
        })
        .await;
    });

    view! { cx, <CpuChart values=history /> }
}
