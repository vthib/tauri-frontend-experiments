use leptos::*;

use crate::components::processes::{Processes, ProcessesProps};
use crate::components::resources::{Resources, ResourcesProps};
use crate::components::sidebar::{Component, Sidebar, SidebarProps};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (active_component, set_active_component) = create_signal(cx, Component::Processes);

    view! { cx,
      <main class="container mx-auto">
        <Sidebar active=set_active_component />
        <div class="ml-20 p-4">
          // Need to be a function to be reactive.
          // This is tricky, got burnt multiple times by this
          {move || {
            match active_component.get() {
              Component::Processes => view! { cx, <Processes /> }.into_view(cx),
              Component::Resources => view! { cx, <Resources /> }.into_view(cx),
            }
          }}
        </div>
      </main>
    }
}
