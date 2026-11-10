<!--
=========================================================
* © 2026 Ronan LE MEILLAT for SCTG Development
=========================================================
The history of the connections to the devices, with a note on each (what was done), to search and to export.
-->
<template>
  <section class="bg-white dark:bg-dark">
    <div class="container mx-auto">
      <form class="mb-4 flex flex-wrap items-end gap-3" @submit.prevent="search()">
        <div>
          <label for="conn-q" class="block text-sm font-medium text-gray-900">Search</label>
          <input id="conn-q" v-model="q" type="search" placeholder="device, user, address, note"
            class="mt-1 block w-72 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 sm:text-sm" />
        </div>
        <div>
          <label for="conn-from" class="block text-sm font-medium text-gray-900">From</label>
          <input id="conn-from" v-model="from" type="date"
            class="mt-1 block rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 sm:text-sm" />
        </div>
        <div>
          <label for="conn-to" class="block text-sm font-medium text-gray-900">To</label>
          <input id="conn-to" v-model="to" type="date"
            class="mt-1 block rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 sm:text-sm" />
        </div>
        <button type="submit"
          class="rounded-md bg-indigo-600 px-4 py-2 text-sm font-medium text-white hover:bg-indigo-500">Search</button>
        <button type="button" @click="exportCsv()"
          class="rounded-md border border-indigo-600 px-4 py-2 text-sm font-medium text-indigo-600 hover:bg-indigo-50">Export
          CSV</button>
      </form>

      <div class="max-w-full overflow-x-auto">
        <table class="w-full table-auto">
          <thead class="bg-slate-400">
            <tr class="text-center">
              <th class="py-4 px-3 text-lg font-medium text-white">Started</th>
              <th class="py-4 px-3 text-lg font-medium text-white">Duration</th>
              <th class="py-4 px-3 text-lg font-medium text-white">Device</th>
              <th class="py-4 px-3 text-lg font-medium text-white">Controlled from</th>
              <th class="py-4 px-3 text-lg font-medium text-white">User</th>
              <th class="py-4 px-3 text-lg font-medium text-white">Type</th>
              <th class="py-4 px-3 text-lg font-medium text-white">Address</th>
              <th class="min-w-[260px] py-4 px-3 text-lg font-medium text-white">Note</th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="records.length === 0">
              <td colspan="8" class="py-6 text-center text-gray-500">No connection</td>
            </tr>
            <tr v-for="(r, i) in records" :key="r.guid" :class="i % 2 ? 'bg-white' : 'bg-[#F3F6FF]'">
              <td class="border-b border-[#E8E8E8] py-3 px-2 text-center text-sm">{{ r.started_at }} UTC</td>
              <td class="border-b border-[#E8E8E8] py-3 px-2 text-center text-sm">{{ duration(r) }}</td>
              <td class="border-b border-[#E8E8E8] py-3 px-2 text-center text-sm">
                {{ r.device_name || '' }}<br /><span class="text-gray-500">{{ r.device_id }}</span>
              </td>
              <td class="border-b border-[#E8E8E8] py-3 px-2 text-center text-sm">
                {{ r.controller_name || '' }}<br /><span class="text-gray-500">{{ r.controller_id || '' }}</span>
              </td>
              <td class="border-b border-[#E8E8E8] py-3 px-2 text-center text-sm">{{ r.user || '' }}</td>
              <td class="border-b border-[#E8E8E8] py-3 px-2 text-center text-sm">{{ typeName(r.conn_type) }}</td>
              <td class="border-b border-[#E8E8E8] py-3 px-2 text-center text-sm">{{ r.ip || '' }}</td>
              <td class="border-b border-[#E8E8E8] py-3 px-2 text-sm">
                <textarea v-model="r.note" rows="2" @blur="saveNote(r)" :aria-label="'Note of ' + r.device_id"
                  class="block w-full rounded-md border-0 py-1 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 sm:text-sm"></textarea>
                <span v-if="saved === r.guid" class="text-xs text-green-700">Saved</span>
                <span v-if="failed === r.guid" class="text-xs text-red-700">Not saved</span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="mt-4 flex items-center justify-between text-sm">
        <span>{{ total }} connection{{ total === 1 ? '' : 's' }}</span>
        <div class="flex items-center gap-3">
          <button type="button" :disabled="page <= 1" @click="go(page - 1)"
            class="rounded-md border px-3 py-1 disabled:opacity-40">Previous</button>
          <span>Page {{ page }} / {{ pages }}</span>
          <button type="button" :disabled="page >= pages" @click="go(page + 1)"
            class="rounded-md border px-3 py-1 disabled:opacity-40">Next</button>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import axios from 'axios';
import { computed, onMounted, ref } from 'vue';
import { useUserStore } from '@/stores/sctgDeskStore';

interface Connection {
  guid: string;
  device_id: string;
  device_name?: string | null;
  controller_id?: string | null;
  controller_name?: string | null;
  user?: string | null;
  conn_type: number;
  ip?: string | null;
  note: string;
  started_at: string;
  ended_at?: string | null;
}

const PAGE_SIZE = 25;
const userStore = useUserStore();
const records = ref<Connection[]>([]);
const total = ref(0);
const page = ref(1);
const q = ref('');
const from = ref('');
const to = ref('');
const saved = ref('');
const failed = ref('');
const pages = computed(() => Math.max(1, Math.ceil(total.value / PAGE_SIZE)));

/** The connection to the API: the base path and the token of the session of the console. */
function request(path: string, method: 'get' | 'put', params?: object, data?: object) {
  const configuration = userStore.api_configuration;
  return axios.request({
    url: `${configuration.basePath || ''}${path}`,
    method,
    params,
    data,
    headers: { Authorization: `Bearer ${configuration.accessToken}` },
  });
}

function filters(current: number, pageSize: number) {
  return { current, pageSize, q: q.value || undefined, from: from.value || undefined, to: to.value || undefined };
}

async function load() {
  try {
    const response = await request('/api/audit/conn', 'get', filters(page.value, PAGE_SIZE));
    records.value = response.data.data;
    total.value = response.data.total;
  } catch (error) {
    console.error(error);
    records.value = [];
    total.value = 0;
  }
}

function search() {
  page.value = 1;
  load();
}

function go(target: number) {
  page.value = Math.min(Math.max(1, target), pages.value);
  load();
}

async function saveNote(record: Connection) {
  saved.value = failed.value = '';
  try {
    await request('/api/audit', 'put', undefined, { guid: record.guid, note: record.note });
    saved.value = record.guid;
  } catch (error) {
    console.error(error);
    failed.value = record.guid;
  }
}

const TYPES = ['Remote', 'File transfer', 'Port forward', 'Camera', 'Terminal'];
function typeName(type: number): string {
  return TYPES[type] ?? String(type);
}

/** The server gives UTC times as `YYYY-MM-DD HH:MM:SS`. */
function parse(time: string): number {
  return Date.parse(time.replace(' ', 'T') + 'Z');
}

function duration(record: Connection): string {
  if (!record.ended_at) {
    return 'in progress';
  }
  const seconds = Math.max(0, Math.round((parse(record.ended_at) - parse(record.started_at)) / 1000));
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = seconds % 60;
  return h > 0 ? `${h} h ${m} min` : m > 0 ? `${m} min ${s} s` : `${s} s`;
}

/** The matching connections (up to 1000) as a CSV file, for a report or an invoice. */
async function exportCsv() {
  const response = await request('/api/audit/conn', 'get', filters(1, 1000));
  const quote = (value: unknown) => `"${String(value ?? '').replace(/"/g, '""')}"`;
  const lines = [['Started (UTC)', 'Ended (UTC)', 'Device', 'Device id', 'Controlled from', 'User', 'Type', 'Address', 'Note']
    .map(quote).join(',')];
  for (const r of response.data.data as Connection[]) {
    lines.push([r.started_at, r.ended_at, r.device_name, r.device_id, r.controller_name, r.user, typeName(r.conn_type), r.ip, r.note]
      .map(quote).join(','));
  }
  const url = URL.createObjectURL(new Blob([lines.join('\r\n')], { type: 'text/csv;charset=utf-8' }));
  const link = document.createElement('a');
  link.href = url;
  link.download = 'connections.csv';
  link.click();
  URL.revokeObjectURL(url);
}

onMounted(load);
</script>
