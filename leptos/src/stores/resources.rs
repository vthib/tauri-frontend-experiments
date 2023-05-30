use leptos::{create_signal, log, ReadSignal, Scope, SignalUpdate};
use wasm_bindgen::prelude::*;

use crate::api::subscribe_to_global_cpu_usage;

pub async fn global_cpu_usage(cx: Scope) -> ReadSignal<Vec<JsValue>> {
    let arr = vec![JsValue::UNDEFINED; 180];
    let (history, set_history) = create_signal(cx, arr);

    subscribe_to_global_cpu_usage(cx, move |v| {
        set_history.update(|vec| {
            vec.remove(0);
            log!("got new value {:?}, vec {:?}", v, &vec);
            vec.push(v);
        });
    })
    .await;

    return history;
}
