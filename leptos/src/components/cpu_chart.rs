use std::sync::Mutex;

use leptos::html::Canvas;
use leptos::{
    component, create_effect, create_node_ref, view, IntoView, ReadSignal, Scope, SignalGet,
};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen(module = "/assets/generated/chart.js")]
extern "C" {
    fn buildChart(canvas: &HtmlCanvasElement, values: Vec<JsValue>) -> JsValue;
    fn updateChart(chart: &JsValue, values: Vec<JsValue>);
}

#[component]
pub fn CpuChart(cx: Scope, values: ReadSignal<Vec<JsValue>>) -> impl IntoView {
    let canvas_ref = create_node_ref::<Canvas>(cx);

    let chart: Mutex<Option<JsValue>> = Mutex::new(None);
    create_effect(cx, move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let vals = values.get();

            {
                let v = chart.lock().unwrap();
                if let Some(chart) = &*v {
                    updateChart(chart, vals);
                    return;
                }
            }
            *chart.lock().unwrap() = Some(buildChart(&canvas, vals));
        }
    });

    view! { cx,
        <canvas _ref=canvas_ref />
    }
}
