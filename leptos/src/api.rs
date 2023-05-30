use leptos::{on_cleanup, spawn_local, Scope};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Event<T> {
    payload: T,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event_name: &str, handler: &Closure<dyn Fn(JsValue)>) -> JsValue;
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Process {
    pub info: ProcessInfo,
    pub resources: ProcessResources,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub user: Option<String>,
    pub status: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ProcessResources {
    pub memory: u64,
    pub virtual_memory: u64,
    pub cpu_usage: u32,
}

#[derive(Serialize, Deserialize)]
pub struct GetProcessArgs {
    pid: u32,
}

pub async fn get_process(pid: u32) -> Option<Process> {
    let args = to_value(&GetProcessArgs { pid }).unwrap();
    let res = invoke("get_process", args).await;
    from_value(res).unwrap()
}

pub async fn get_processes() -> Vec<Process> {
    let args = to_value(&()).unwrap();
    let res = invoke("get_processes", args).await;
    from_value(res).unwrap()
}

pub async fn subscribe_to_global_cpu_usage<F>(cx: Scope, cb: F)
where
    F: Fn(u32) + 'static,
{
    invoke("subscribe_global_cpu_usage", JsValue::UNDEFINED).await;

    let cb = Closure::<dyn Fn(JsValue)>::new(move |v| {
        let event: Event<u32> = serde_wasm_bindgen::from_value(v).unwrap();
        cb(event.payload);
    });
    let unlisten = listen("global_cpu_usage", &cb).await;
    cb.forget();

    on_cleanup(cx, move || {
        let unlisten = js_sys::Function::from(unlisten);
        unlisten.call0(&JsValue::UNDEFINED).unwrap();
        spawn_local(async move {
            invoke("unsubscribe_global_cpu_usage", JsValue::UNDEFINED).await;
        })
    })
}
