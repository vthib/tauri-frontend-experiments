<script setup lang="ts">
import Modal from "@/components/Modal.vue";
import ProcessDetails from "@/components/ProcessDetails.vue";

import { processes as processesStore } from "@/stores/processes";
import type { Process } from "@/api";
import { computed, ref } from 'vue';
import ChevronDown from "@/icons/heroicons/ChevronDown.svg";
import ChevronUp from "@/icons/heroicons/ChevronUp.svg";
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
let fieldToSortIndex = ref<number | undefined>(undefined);
let sortDirection = ref<"desc" | "asc" | "none">("none");
let setSort = (fieldIndex: number) => {
    if (fieldToSortIndex.value != fieldIndex) {
        fieldToSortIndex.value = fieldIndex;
        sortDirection.value = "none";
    }

    switch (sortDirection.value) {
        case "none":
            sortDirection.value = "asc";
            break;
        case "asc":
            sortDirection.value = "desc";
            break;
        case "desc":
            sortDirection.value = "none";
            break;
    }
};

const processes = processesStore();
const sortedProcesses = computed(() => {
    if (fieldToSortIndex.value === undefined) {
        return processes.value;
    }
    let accessor = fields[fieldToSortIndex.value].accessor;
    let cmp =
        fields[fieldToSortIndex.value].cmp ||
        ((a, b) => accessor(a).localeCompare(accessor(b)));

    switch (sortDirection.value) {
        case "none":
            return processes.value;
        case "desc":
            return processes.value.sort((a, b) => cmp(b, a));
        case "asc":
            return processes.value.sort(cmp);
    }
});

let processToShow = ref<Process | undefined>(undefined);
</script>

<template>
    <table class="border-y border-collapse">
        <tr>
            <th v-for="(field, fieldIndex) in fields"
                class="p-1 border-y first:border-l last:border-r border-solid border-slate-700"
                @click="() => setSort(fieldIndex)">
                <div class="flex flex-row">
                    <template v-if="fieldIndex == fieldToSortIndex">
                        <ChevronUp v-if="sortDirection == 'asc'" />
                        <ChevronDown v-else-if="sortDirection == 'desc'" />
                    </template>
                    {{ field.title }}
                </div>
            </th>
        </tr>
        <tr v-for="process in sortedProcesses" :key="process.info.pid" class="odd:bg-white even:bg-slate-200"
            @click="() => processToShow = process">
            <td v-for="field in fields"
                :class="field.classes + ' p-1 first:border-l last:border-r border-y border-solid border-slate-700'">
                {{ field.accessor(process) }}</td>
        </tr>
    </table>

    <Modal :show="!!processToShow" @close="() => processToShow = undefined">
        <ProcessDetails v-if="processToShow" :process="processToShow" />
    </Modal>
</template>