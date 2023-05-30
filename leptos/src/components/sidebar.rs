use crate::icons::heroicons::{ChartBarSquare, ChartBarSquareProps, ListBullet, ListBulletProps};
use leptos::{
    component, create_effect, create_signal, view, IntoView, Scope, SignalGet, SignalSet,
    WriteSignal,
};

#[derive(Debug, Copy, Clone)]
pub enum Component {
    Processes,
    Resources,
}

#[component]
pub fn Sidebar(cx: Scope, active: WriteSignal<Component>) -> impl IntoView {
    const COMPONENTS: [Component; 2] = [Component::Processes, Component::Resources];
    let component_icon = move |cx, component| match component {
        Component::Processes => view! { cx, <ListBullet /> }.into_view(cx),
        Component::Resources => view! { cx, <ChartBarSquare /> }.into_view(cx),
    };

    let (active_element_index, set_active_element_index) = create_signal(cx, 0);
    create_effect(cx, move |_| {
        let idx = active_element_index.get();
        active.set(COMPONENTS[idx]);
    });

    view! { cx,
    <aside class="fixed top-0 bottom-0 left-0 w-16 bg-lime-300">
        <ul class="flex flex-col">
            {COMPONENTS.iter().enumerate().map(|(idx, component)| {
                view! { cx,
                <li class="p-2" class=("bg-lime-600", move || active_element_index.get() == idx)>
                    <a
                        href="#"
                        class="hover:invert"
                        on:click=move |_| set_active_element_index.set(idx)
                    >
                        {component_icon(cx, *component)}
                    </a>
                </li>
                }
            }).collect::<Vec<_>>()}
        </ul>
    </aside >
        }
}
