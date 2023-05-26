<script lang="ts">
    import type { Process } from "@/api";

    export let process: Process;
</script>

<tr>
    <td>{process.info.pid}</td>
    <td>{process.info.name}</td>
    <td>{process.info.user || ""}</td>
    <td>{process.info.status}</td>
    <td>{process.resources.memory}</td>
    <td>{process.resources.cpu_usage}</td>
</tr>
