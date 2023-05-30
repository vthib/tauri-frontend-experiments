use std::sync::Arc;

use leptos::html::Dialog;
use leptos::{component, create_effect, create_node_ref, view, Children, IntoView, Scope};

#[component]
pub fn Modal<F, G>(cx: Scope, show: F, hide: G, children: Children) -> impl IntoView
where
    F: Fn() -> bool + 'static,
    G: Fn() + 'static,
{
    let dialog_ref = create_node_ref::<Dialog>(cx);

    create_effect(cx, move |_| {
        if show() {
            if let Some(dialog) = dialog_ref.get() {
                dialog.show_modal().unwrap();
            }
        }
    });

    let hide = Arc::new(hide);

    let on_click = {
        let hide = Arc::clone(&hide);
        move |_| {
            let dialog = dialog_ref.get().unwrap();
            dialog.close();
            hide();
        }
    };

    view! { cx,
        <dialog
            _ref=dialog_ref
            on:close=move |_| hide()
            on:click=on_click
            class="backdrop:backdrop-blur-sm border-solid rounded-md"
        >
            <div on:click=|e| e.stop_propagation()>
                {children(cx)}
            </div>
        </dialog>
    }
}
