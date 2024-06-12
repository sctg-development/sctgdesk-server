<!--
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
-->
<template>
    <div class="flex">
        <div class="w-1/3 rounded-3xl bg-white text-slate-600 shadow-xl mt-10 p-10">
            <h2 class="text-center text-2xl font-bold leading-9 tracking-tight text-gray-500">Os</h2>
            <canvas id="peersChart"></canvas>
        </div>
        <div class="w-1/3 rounded-3xl bg-white text-slate-600 shadow-xl ml-10 mt-10 p-10">
            <h2 class="text-center text-2xl font-bold leading-9 tracking-tight text-gray-500">Cpus</h2>
            <canvas id="cpusChart"></canvas>
        </div>
    </div>

</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useUserStore } from '@/stores/sctgDeskStore';
import { PeerApi } from '@/api';
import { Chart, registerables, ChartItem, ChartConfiguration } from 'chart.js';

Chart.register(...registerables);

const CHART_COLORS = {
    red: 'rgb(255, 99, 132)',
    orange: 'rgb(255, 159, 64)',
    yellow: 'rgb(255, 205, 86)',
    green: 'rgb(75, 192, 192)',
    blue: 'rgb(54, 162, 235)',
    purple: 'rgb(153, 102, 255)',
    grey: 'rgb(201, 203, 207)'
};

const NAMED_COLORS = [
    CHART_COLORS.red,
    CHART_COLORS.orange,
    CHART_COLORS.yellow,
    CHART_COLORS.green,
    CHART_COLORS.blue,
    CHART_COLORS.purple,
    CHART_COLORS.grey,
];

const windowsPeersCount = ref(0);
const linuxPeersCount = ref(0);
const macPeersCount = ref(0);
const androidPeersCount = ref(0);
const allPeersCount = ref(0);
const othersPeersCount = ref(0);

function createPeersChart() {
    const peerApi = new PeerApi(useUserStore().api_configuration);
    Promise.all([
        peerApi.peersCount("windows"),
        peerApi.peersCount("linux"),
        peerApi.peersCount("mac"),
        peerApi.peersCount("android"),
        peerApi.peersCount("all")
    ]).then((responses) => {
        windowsPeersCount.value = responses[0].data.total;
        linuxPeersCount.value = responses[1].data.total;
        macPeersCount.value = responses[2].data.total;
        androidPeersCount.value = responses[3].data.total;
        allPeersCount.value = responses[4].data.total;
        othersPeersCount.value = allPeersCount.value - windowsPeersCount.value - linuxPeersCount.value - macPeersCount.value - androidPeersCount.value;
        const data = {
            labels: ['Windows', 'Linux', 'Mac', 'Android', 'Others'],
            datasets: [{
                data: [windowsPeersCount.value,
                linuxPeersCount.value,
                macPeersCount.value,
                androidPeersCount.value,
                othersPeersCount.value],
                backgroundColor: Object.values(CHART_COLORS),
            }]
        };

        const config = {
            type: 'pie',
            data: data,
        } as ChartConfiguration;

        let myChart = new Chart(
            document.getElementById('peersChart') as ChartItem,
            config
        );
    }).catch((error) => {
        console.error(error);
    });
}

function createCpusChart() {
    const peerApi = new PeerApi(useUserStore().api_configuration);
    peerApi.peersCpus().then((response) => {
        const cpus = response.data;
        const data = {
            labels: cpus.map((cpu) => cpu.cpu),
            datasets: [{
                data: cpus.map((cpu) => cpu.total),
                backgroundColor: Object.values(CHART_COLORS),
            }]
        };

        const config = {
            type: 'polarArea',
            data: data,
            options: {
                plugins: {
                    legend: {
                        display: false
                    }
                }
            }
        } as ChartConfiguration;

        let cpuChart = new Chart(
            document.getElementById('cpusChart') as ChartItem,
            config
        );
    }).catch((error) => {
        console.error(error);
    });
}
onMounted(() => {
    createPeersChart();
    createCpusChart();
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped></style>