<script lang="ts">
  import Processes from "./components/Processes.svelte";
  import Resources from "./components/Resources.svelte";
  import Sidebar from "./components/Sidebar.svelte";

  let active: string = "processes";
</script>

<main class="container mx-auto">
  <Sidebar on:active={(event) => (active = event.detail)} />
  <div class="ml-20 p-4">
    <!-- <h1 class="test-3xl font-bold underline">Sysinfo UI</h1> -->
    {#if active == "processes"}
      <Processes />
    {:else if active == "resources"}
      <Resources />
    {/if}
  </div>
</main>
