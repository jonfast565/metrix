window.chart = null;

function initializeChart() {
    Chart.overrides.line.spanGaps = true;
    const chartElement = document.getElementById('chart-body');
    chartElement.height = 200;
    const ctx = chartElement.getContext('2d');
    window.chart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: [],
            datasets: [
                {
                    label: '',
                    data: []
                }
            ]
        },
        options: {
            scales: {
                x: {
                    type: 'time'
                }
            },
            animation: {
                duration: 0
            }
        }
    });
}

function drawChart(chartValues) {
    const dataType = chartValues[0].dataType;
    const values = chartValues.map(v => v.dataValueNumeric);
    const labels = chartValues.map(v => new Date(v.createdDatetime).toISOString());

    window.chart.data.labels.length = 0;
    window.chart.data.labels.push(...labels);

    window.chart.data.datasets[0].label = dataType;
    window.chart.data.datasets[0].data.length = 0;
    window.chart.data.datasets[0].data.push(...values);

    window.chart.update();
}

export { initializeChart, drawChart }