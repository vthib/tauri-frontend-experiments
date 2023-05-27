<script lang="ts">
    import Chart from "chart.js/auto";
    import { onMount } from "svelte";

    export let values: number[];

    let canvas;
    let chart: undefined | Chart = undefined;

    $: if (chart !== undefined) {
        chart.data.datasets[0].data = values;
        chart.update();
    }

    onMount(() => {
        chart = new Chart(canvas, {
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
                        data: values,
                        fill: true,
                    },
                ],
            },
        });
    });
</script>

<canvas bind:this={canvas} />
