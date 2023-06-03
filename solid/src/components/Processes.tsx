import Modal from "@/components/Modal";
import ProcessDetails from "@/components/ProcessDetails";

import { processes } from "@/stores/processes";
import type { Process } from "@/api";
import { For, Show, Switch, Match, createSignal } from "solid-js";
import { Icon } from "solid-heroicons";
import { chevronDown, chevronUp } from "solid-heroicons/solid";
import { formatWithUnits } from "@/utils";

// Fields to display in the table.
interface Field {
    title: string;
    accessor: (p: Process) => string;
    cmp?: (a: Process, b: Process) => number;
    classes: string;
}
const fields: Field[] = [
    {
        title: "PID",
        accessor: (p) => p.info.pid.toString(),
        cmp: (a, b) => a.info.pid - b.info.pid,
        classes: "w-12",
    },
    {
        title: "Name",
        accessor: (p) => p.info.name,
        classes: "w-80",
    },
    {
        title: "User",
        accessor: (p) => p.info.user || "",
        classes: "w-20",
    },
    {
        title: "Status",
        accessor: (p) => p.info.status,
        classes: "w-20",
    },
    {
        title: "Mem",
        accessor: (p) => formatWithUnits(p.resources.memory),
        cmp: (a, b) => a.resources.memory - b.resources.memory,
        classes: "w-20",
    },
    {
        title: "CPU",
        accessor: (p) => `${p.resources.cpu_usage}%`,
        cmp: (a, b) => a.resources.cpu_usage - b.resources.cpu_usage,
        classes: "w-32",
    },
];

export default function Processes() {
    // Index of the field to sort
    let [fieldToSortIndex, setFieldToSortIndex] = createSignal<number | undefined>(undefined);
    let [sortDirection, setSortDirection] = createSignal<"desc" | "asc" | "none">("none");
    let setSort = (fieldIndex: number) => {
        if (fieldToSortIndex() != fieldIndex) {
            setFieldToSortIndex(fieldIndex);
            setSortDirection("none");
        }

        switch (sortDirection()) {
            case "none":
                setSortDirection("asc");
                break;
            case "asc":
                setSortDirection("desc");
                break;
            case "desc":
                setSortDirection("none");
                break;
        }
    };

    let processesStore = processes();
    const sortedProcesses = (): Process[] => {
        let index = fieldToSortIndex();
        if (index === undefined) {
            return processesStore;
        }
        let accessor = fields[index].accessor;
        let cmp =
            fields[index].cmp ||
            ((a, b) => accessor(a).localeCompare(accessor(b)));

        switch (sortDirection()) {
            case "none":
                return processesStore;
            case "desc":
                return [...processesStore].sort((a, b) => cmp(b, a));
            case "asc":
                return [...processesStore].sort(cmp);
        }
    };


    const [processToShow, setProcessToShow] = createSignal<Process | undefined>(undefined);
    return <>
        <table class="border-y border-collapse">
            <tbody>
                <tr>
                    <For each={fields}>{(field, fieldIndex) =>
                        <th
                            class="p-1 border-y first:border-l last:border-r border-solid border-slate-700"
                            onClick={() => setSort(fieldIndex())}
                        >
                            <div class="flex flex-row">
                                <Show when={fieldIndex() == fieldToSortIndex()}>
                                    <Switch>
                                        <Match when={sortDirection() == "asc"}>
                                            <Icon path={chevronUp} class="w-6 h-6" />
                                        </Match>
                                        <Match when={sortDirection() == "desc"}>
                                            <Icon path={chevronDown} class="w-6 h-6" />
                                        </Match>
                                    </Switch>
                                </Show>
                                {field.title}
                            </div>
                        </th>
                    }
                    </For>
                </tr>
                <For each={sortedProcesses()}>{(process) =>
                    <tr
                        class="odd:bg-white even:bg-slate-200"
                        onClick={() => setProcessToShow(process)}
                    >
                        <For each={fields}>{(field) =>
                            <td
                                class={field.classes +
                                    " p-1 first:border-l last:border-r border-y border-solid border-slate-700"}
                            >{field.accessor(process)}</td>}
                        </For>
                    </tr>
                }
                </For>
            </tbody >
        </table>
        <Modal show={!!processToShow()} hide={() => setProcessToShow(undefined)}>
            <Show when={processToShow()}>
                <ProcessDetails process={processToShow()!} />
            </Show>
        </Modal >
    </>;
}
