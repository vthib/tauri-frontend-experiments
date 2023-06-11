<script setup lang="ts">
import Chart from "chart.js/auto";
import { onMounted, shallowRef, watchEffect } from "vue";

const props = defineProps<{
    values: number[]
}>();

const canvas = shallowRef<HTMLCanvasElement | undefined>(undefined);
const chart = shallowRef<Chart | undefined>(undefined);

watchEffect(() => {
    if (chart.value !== undefined) {
        chart.value.data.datasets[0].data = props.values;
        chart.value.update();
    }
});

onMounted(() => {
    chart.value = new Chart(canvas.value!, {
        type: "line",
        options: {
            scales: {
                x: {
                    min: 0,
                    max: 180,
                    grid: {
                        display: false,
                    },
                },
                y: {
                    min: 0,
                    max: 100,
                },
            },
            datasets: {
                line: {
                    pointStyle: false,
                },
            },
            plugins: {
                legend: {
                    display: true,
                },
            },
        },
        data: {
            labels: [...Array(180).fill("")],
            datasets: [
                {
                    label: "CPU usage",
                    data: props.values,
                    fill: true,
                },
            ],
        },
    }) as Chart;
});
</script>

<template>
    <canvas ref="canvas" />
</template>
