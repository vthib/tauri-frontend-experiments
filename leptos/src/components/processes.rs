use std::borrow::Cow;

use leptos::{component, create_signal, view, IntoView, Scope, SignalGet, SignalSet, SignalUpdate};

use crate::api::Process;
use crate::components::modal::{Modal, ModalProps};
use crate::components::process_details::{ProcessDetails, ProcessDetailsProps};
use crate::icons::heroicons::{ChevronDown, ChevronDownProps, ChevronUp, ChevronUpProps};
use crate::stores::processes::processes_store;
use crate::utils::format_with_units;

// Fields to display in the table.
struct Field {
    title: &'static str,
    accessor: fn(&Process) -> Cow<str>,
    cmp: Option<for<'a, 'b> fn(&'a Process, &'b Process) -> std::cmp::Ordering>,
    classes: &'static str,
}

const FIELDS: [Field; 6] = [
    Field {
        title: "PID",
        accessor: |p| p.info.pid.to_string().into(),
        cmp: Some(|a, b| a.info.pid.cmp(&b.info.pid)),
        classes: "w-12",
    },
    Field {
        title: "Name",
        accessor: |p| (&p.info.name).into(),
        cmp: None,
        classes: "w-80",
    },
    Field {
        title: "User",
        accessor: |p| match &p.info.user {
            Some(v) => v.into(),
            None => "".into(),
        },
        cmp: None,
        classes: "w-20",
    },
    Field {
        title: "Status",
        accessor: |p| (&p.info.status).into(),
        cmp: None,
        classes: "w-20",
    },
    Field {
        title: "Mem",
        accessor: |p| format_with_units(p.resources.memory).into(),
        cmp: Some(|a, b| a.resources.memory.cmp(&b.resources.memory)),
        classes: "w-20",
    },
    Field {
        title: "CPU",
        accessor: |p| format!("{}%", p.resources.cpu_usage).into(),
        cmp: Some(|a, b| a.resources.cpu_usage.cmp(&b.resources.cpu_usage)),
        classes: "w-32",
    },
];

#[derive(Copy, Clone, Debug)]
enum SortDirection {
    None,
    Asc,
    Desc,
}

#[component]
pub fn processes(cx: Scope) -> impl IntoView {
    // Index of the field to sort
    let (fieldToSortIndex, setFieldToSortIndex) = create_signal(cx, None);
    let (sortDirection, setSortDirection) = create_signal(cx, SortDirection::None);

    let setSort = move |fieldIndex: usize| {
        if fieldToSortIndex.get() != Some(fieldIndex) {
            setFieldToSortIndex.set(Some(fieldIndex));
            setSortDirection.set(SortDirection::None);
        }

        setSortDirection.update(|v| {
            *v = match *v {
                SortDirection::None => SortDirection::Asc,
                SortDirection::Asc => SortDirection::Desc,
                SortDirection::Desc => SortDirection::None,
            }
        });
    };

    let processesStore = processes_store(cx);
    let sortedProcesses = move || {
        let mut processes = processesStore.get();
        let Some(index) = fieldToSortIndex.get() else {
            return processes;
        };

        fn sort(processes: &mut [Process], field: &Field, desc: bool) {
            let accessor = field.accessor;
            processes.sort_by(|a, b| match field.cmp {
                Some(cmp) if desc => cmp(b, a),
                Some(cmp) => cmp(a, b),
                None if desc => accessor(b).cmp(&accessor(a)),
                None => accessor(a).cmp(&accessor(b)),
            });
        }

        match sortDirection.get() {
            SortDirection::None => (),
            SortDirection::Asc => sort(&mut processes, &FIELDS[index], false),
            SortDirection::Desc => sort(&mut processes, &FIELDS[index], true),
        }
        processes
    };

    let headers: Vec<_> = FIELDS
        .iter()
        .enumerate()
        .map(|(fieldIndex, field)| {
            view! { cx,
                <th
                    class="p-1 border-y first:border-l last:border-r border-solid border-slate-700"
                    on:click=move |_| setSort(fieldIndex)
                >
                    <div class="flex flex-row">
                        {move || (Some(fieldIndex) == fieldToSortIndex.get()).then(|| match sortDirection.get() {
                                SortDirection::None => None,
                                SortDirection::Asc => Some(view! { cx, <ChevronUp /> }.into_view(cx)),
                                SortDirection::Desc => Some(view! { cx, <ChevronDown /> }.into_view(cx)),
                        })}
                        {field.title}
                    </div>
                </th>
            }
        })
        .collect();

    let (process_to_show, set_process_to_show) = create_signal(cx, None);
    // Must have this instead of processToShow to have a signal with the read type to pass to the child
    // component. This isn't great...
    view! { cx,
        <>
        <table class="border-y border-collapse">
            <tbody>
                <tr>
                    {headers}
                </tr>
                // For syntax is broken, this does not work
                // <For
                //     each=sortedProcesses
                //     key=|process| process.info.pid
                //     view=move |cx, process| {
                {move || {
                    let procs = sortedProcesses();
                    procs.into_iter().map(|process| {
                        let rows: Vec<_> = FIELDS.iter().map(|field| view! { cx,
                            <td
                                class={field.classes}
                                class=" p-1 first:border-l last:border-r border-y border-solid border-slate-700"
                            >
                                {(field.accessor)(&process)}
                            </td>
                        }).collect();

                        view! { cx,
                            <tr
                                class="odd:bg-white even:bg-slate-200"
                                on:click=move |_| {
                                    set_process_to_show.set(Some(process.clone()));
                                }
                            >
                                {rows}
                            </tr>
                        }
                    }).collect::<Vec<_>>()}}
            </tbody>
        </table>
        <Modal
            show=move || process_to_show.get().is_none()
            hide=move || {
                set_process_to_show.set(None);
            }
        >
            {move || process_to_show.get().map(|process| view! { cx, <ProcessDetails process=process /> })}
        </Modal >
    </>
    }
}
