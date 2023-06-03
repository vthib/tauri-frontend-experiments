<script lang="ts">
    import { generateProcessStore } from "@/stores/process";
    import type { Process } from "@/api";
    import CpuChart from "@/components/CpuChart.svelte";

    export let process: Process;

    let values = Array(180).fill(undefined);

    let processStore = generateProcessStore(process.info.pid);
    $: {
        if ($processStore) {
            const [, ...cdr] = values;
            values = [...cdr, $processStore.resources.cpu_usage];
        }
    }
</script>

<h1>Process details: {process.info.name}</h1>
<CpuChart {values} />
