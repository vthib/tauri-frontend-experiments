import Chart from "chart.js/auto";
import { createEffect, createSignal, onMount } from "solid-js";

export interface Props {
    values: Array<number | undefined>
}

export default function CpuChart(props: Props) {
    let canvas: HTMLCanvasElement;
    const [getChart, setChart] = createSignal<undefined | Chart>(undefined);

    onMount(() => {
        createEffect(() => {
            const chart = getChart();
            if (chart === undefined) {
                return;
            }
            chart.data.datasets[0].data = props.values as any;
            chart.update();
        });

        setChart(new Chart(canvas, {
            type: "line",
            options: {
                animation: false,
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
                        data: props.values as any,
                        fill: true,
                    },
                ],
            },
        }));
    });

    return <canvas ref={canvas!} />
};
