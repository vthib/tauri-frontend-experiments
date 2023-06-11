<script setup lang="ts">
import { globalCpuUsage } from "@/stores/resources";
import CpuChart from "@/components/CpuChart.vue";

const cpuUsage = globalCpuUsage();
</script>

<template>
    <CpuChart :values="cpuUsage" />
</template>
