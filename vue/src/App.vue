<script setup lang="ts">
import Processes from "./components/Processes.vue";
import Resources from "./components/Resources.vue";
import Sidebar from "./components/Sidebar.vue";
import { ref } from 'vue';

const active = ref("processes");
</script>

<template>
  <main class="container mx-auto">
    <Sidebar @active="(v: string) => active = v" />
    <div class="ml-20 p-4">
      <Processes v-if="active == 'processes'" />
      <Resources v-else-if="active == 'resources'" />
    </div>
  </main>
</template>