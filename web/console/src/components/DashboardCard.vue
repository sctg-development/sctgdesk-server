<!--
=========================================================
* © 2024-2026 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
-->
<template>
  <div>
    <div class="flex">
        <div class="w-1/3 rounded-3xl bg-white text-slate-600 shadow-xl mt-10 p-10">
            <h2 class="text-center text-2xl font-bold leading-9 tracking-tight text-gray-500">{{ t('dashboard.os') }}</h2>
            <canvas id="peersChart"></canvas>
        </div>
        <div class="w-1/3 rounded-3xl bg-white text-slate-600 shadow-xl ml-10 mt-10 p-10">
            <h2 class="text-center text-2xl font-bold leading-9 tracking-tight text-gray-500">{{ t('dashboard.cpus') }}</h2>
            <canvas id="cpusChart"></canvas>
        </div>
    </div>

    <!-- The ten devices seen most recently, each with a link that opens the web client on it -->
    <div class="rounded-3xl bg-white text-slate-600 shadow-xl mt-10 p-10">
      <h2 class="text-center text-2xl font-bold leading-9 tracking-tight text-gray-500">{{ t('dashboard.recentTitle') }}</h2>
      <p v-if="recentPeers.length === 0" class="mt-4 text-center text-gray-500">{{ t('dashboard.recentEmpty') }}</p>
      <div v-else class="mt-4 max-w-full overflow-x-auto">
        <table class="w-full table-auto text-sm">
          <thead class="bg-slate-400 text-white">
            <tr>
              <th class="py-3 px-3 text-left font-medium">{{ t('dashboard.colDevice') }}</th>
              <th class="py-3 px-3 text-left font-medium">{{ t('dashboard.colUser') }}</th>
              <th class="py-3 px-3 text-left font-medium">{{ t('dashboard.colStatus') }}</th>
              <th class="py-3 px-3 text-left font-medium">{{ t('dashboard.colLastSeen') }}</th>
              <th class="py-3 px-3"></th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(peer, i) in recentPeers" :key="peer.guid" :class="i % 2 ? 'bg-white' : 'bg-[#F3F6FF]'">
              <td class="py-3 px-3">
                <span class="font-medium">{{ peer.info.hostname || peer.id }}</span>
                <span class="ml-2 text-gray-500">{{ peer.id }}</span>
              </td>
              <td class="py-3 px-3">{{ peer.info.username }}</td>
              <td class="py-3 px-3">{{ peer.status ? t('devices.online') : t('devices.offline') }}</td>
              <td class="py-3 px-3">{{ peer.last_online }}</td>
              <td class="py-3 px-3 text-right">
                <button v-if="webClientAvailable()" type="button" @click="openWebClient(peer.id)"
                  :title="t('dashboard.connectTitle', { device: peer.info.hostname || peer.id })"
                  class="rounded-md bg-indigo-600 px-4 py-1.5 text-sm font-medium text-white hover:bg-indigo-500">
                  {{ t('dashboard.connect') }}
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useUserStore } from '@/stores/sctgDeskStore';
import { Peer, PeerApi } from '@/api';
import { t } from '@/i18n';
import { openWebClient, webClientAvailable } from '@/utilities/webclient';
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

/** How many devices the "last connected" list shows. */
const RECENT_COUNT = 10;
const recentPeers = ref([] as Peer[]);

/**
 * Loads the devices and keeps the most recently seen ones.
 *
 * `last_online` is the only "last connection" information the server keeps per device (it is refreshed by every
 * heartbeat), so sorting on it puts the devices connected right now first, then the latest ones to have been.
 * Dates that do not parse sort last instead of breaking the list.
 */
function loadRecentPeers(): void {
    new PeerApi(useUserStore().api_configuration).peers().then((response) => {
        if (response.status == 200 && response.data.msg == "success") {
            const time = (peer: Peer) => Date.parse(peer.last_online.replace(' ', 'T') + (/[zZ+]/.test(peer.last_online) ? '' : 'Z')) || 0;
            recentPeers.value = [...response.data.data].sort((a, b) => time(b) - time(a)).slice(0, RECENT_COUNT);
        }
    }).catch((error) => {
        console.error(error);
    });
}

const windowsPeersCount = ref(0);
const linuxPeersCount = ref(0);
const macPeersCount = ref(0);
const androidPeersCount = ref(0);
const allPeersCount = ref(0);
const othersPeersCount = ref(0);

/**
 * Creates a pie chart displaying the count of peers per platform.
 *
 * @return {void} the chart is created asynchronousy.
 */
function createPeersChart(): void {
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
            labels: ['Windows', 'Linux', 'Mac', 'Android', t('dashboard.others')],
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

/**
 * Creates a polar area chart displaying the total CPU usage for each CPU.
 *
 * @return {void} .
 */
function createCpusChart(): void {
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
    loadRecentPeers();
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped></style>