import Chart from "chart.js/auto";

export function buildChart(canvas, values) {
    return new Chart(canvas, {
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
                    data: values,
                    fill: true,
                },
            ],
        },
    });
}

export function updateChart(chart, values) {
    chart.data.datasets[0].data = values;
    chart.update();
}