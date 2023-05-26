<script lang="ts">
    import { processes } from "@/stores/processes";
    import type { Process } from "@/api";
    import { onMount } from "svelte";
    import { derived } from "svelte/store";

    // Fields to display in the table.
    interface Field {
        title: string;
        accessor: (p: Process) => string;
        cmp?: (a: Process, b: Process) => number;
    }
    const fields: Field[] = [
        {
            title: "PID",
            accessor: (p) => p.info.pid.toString(),
            cmp: (a, b) => a.info.pid - b.info.pid,
        },
        {
            title: "Name",
            accessor: (p) => p.info.name,
        },
        {
            title: "User",
            accessor: (p) => p.info.user || "",
        },
        {
            title: "Status",
            accessor: (p) => p.info.status,
        },
        {
            title: "Mem",
            accessor: (p) => p.resources.memory.toString(),
            cmp: (a, b) => a.resources.memory - b.resources.memory,
        },
        {
            title: "CPU",
            accessor: (p) => `${p.resources.cpu_usage}%`,
            cmp: (a, b) => a.resources.cpu_usage - b.resources.cpu_usage,
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
        console.log(`dir: ${sortDirection}, index: ${fieldToSortIndex}`);
    };

    // Computed over processes to handle the sort.
    const generateSortedStore = (fieldToSortIndex, sortDirection) =>
        derived(processes, ($processes) => {
            console.log(fieldToSortIndex);
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
</script>

<table class="border border-collapse border-slate-500">
    <tr>
        {#each fields as field, fieldIndex}
            <th on:click={() => setSort(fieldIndex)}>{field.title}</th>
        {/each}
    </tr>
    {#each $sortedProcesses as process (process.info.pid)}
        <tr>
            {#each fields as field}
                <td class="border border-solid border-slate-600"
                    >{field.accessor(process)}</td
                >
            {/each}
        </tr>
    {/each}
</table>
