<script lang="ts">
    import { globalCpuUsage } from "@/stores/resources";
    import CpuChart from "@/components/CpuChart.svelte";
</script>

<CpuChart values={$globalCpuUsage} />
