<script setup lang="ts">
import { generateProcessStore } from "@/stores/process";
import type { Process } from "@/api";
import CpuChart from "@/components/CpuChart.vue";
import { watch, ref } from 'vue';

const props = defineProps<{
    process: Process
}>();

let values = ref<number[]>(Array(180).fill(undefined));
let processStore = generateProcessStore(props.process.info.pid);

watch(processStore, (store) => {
    if (store) {
        const [, ...cdr] = values.value;
        values.value = [...cdr, store.resources.cpu_usage];
    }
});
</script>

<template>
    <h1>Process details: {{ process.info.name }}</h1>
    <CpuChart :values="values" />
</template>