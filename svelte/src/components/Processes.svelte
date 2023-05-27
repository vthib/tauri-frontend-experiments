<script lang="ts">
    import Modal from "@/components/Modal.svelte";
    import ProcessDetails from "@/components/ProcessDetails.svelte";

    import { processes } from "@/stores/processes";
    import type { Process } from "@/api";
    import { onMount } from "svelte";
    import { derived } from "svelte/store";
    import ChevronDown from "@/icons/heroicons/chevron-down.svelte";
    import ChevronUp from "@/icons/heroicons/chevron-up.svelte";
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

    // Index of the field to sort
    let fieldToSortIndex: number | undefined = undefined;
    let sortDirection: "desc" | "asc" | "none" = "none";
    let setSort = (fieldIndex: number) => {
        if (fieldToSortIndex != fieldIndex) {
            fieldToSortIndex = fieldIndex;
            sortDirection = "none";
        }

        switch (sortDirection) {
            case "none":
                sortDirection = "asc";
                break;
            case "asc":
                sortDirection = "desc";
                break;
            case "desc":
                sortDirection = "none";
                break;
        }
    };

    const generateSortedStore = (fieldToSortIndex, sortDirection) =>
        derived(processes, ($processes) => {
            if (fieldToSortIndex === undefined) {
                return $processes;
            }
            let accessor = fields[fieldToSortIndex].accessor;
            let cmp =
                fields[fieldToSortIndex].cmp ||
                ((a, b) => accessor(a).localeCompare(accessor(b)));

            switch (sortDirection) {
                case "none":
                    return $processes;
                case "desc":
                    return $processes.sort((a, b) => cmp(b, a));
                case "asc":
                    return $processes.sort(cmp);
            }
        });

    $: sortedProcesses = generateSortedStore(fieldToSortIndex, sortDirection);

    let processToShow;
    let showProcessDetails = (process) => {
        processToShow = process;
    };
</script>

<table class="border-y border-collapse">
    <tr>
        {#each fields as field, fieldIndex}
            <th
                class="p-1 border-y first:border-l last:border-r border-solid border-slate-700"
                on:click={() => setSort(fieldIndex)}
            >
                <div class="flex flex-row">
                    {#if fieldIndex == fieldToSortIndex}
                        {#if sortDirection == "asc"}
                            <ChevronUp />
                        {:else if sortDirection == "desc"}
                            <ChevronDown />
                        {/if}
                    {/if}
                    {field.title}
                </div>
            </th>
        {/each}
    </tr>
    {#each $sortedProcesses as process (process.info.pid)}
        <tr
            class="odd:bg-white even:bg-slate-200"
            on:click={() => showProcessDetails(process)}
        >
            {#each fields as field}
                <td
                    class={field.classes +
                        " p-1 first:border-l last:border-r border-y border-solid border-slate-700"}
                    >{field.accessor(process)}</td
                >
            {/each}
        </tr>
    {/each}
</table>

<Modal show={!!processToShow} on:close={() => (processToShow = undefined)}>
    {#if processToShow}
        <ProcessDetails process={processToShow} />
    {/if}
</Modal>
