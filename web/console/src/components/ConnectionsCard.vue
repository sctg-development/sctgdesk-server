<!--
=========================================================
* © 2024-2026 Ronan LE MEILLAT for SCTG Development
=========================================================
The history of the connections to the devices, with a note on each (what was done), to search and to export.
Administrators can also purge it (a selection, or everything). A click on a device opens the web client on it.
-->
<template>
  <section class="bg-white dark:bg-dark">
    <div class="container mx-auto">
      <form class="mb-4 flex flex-wrap items-end gap-3" @submit.prevent="search()">
        <div>
          <label for="conn-q" class="block text-sm font-medium text-gray-900">{{ t('connections.search') }}</label>
          <input id="conn-q" v-model="q" type="search" :placeholder="t('connections.searchPlaceholder')"
            class="mt-1 block w-72 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 sm:text-sm" />
        </div>
        <div>
          <label for="conn-from" class="block text-sm font-medium text-gray-900">{{ t('connections.from') }}</label>
          <input id="conn-from" v-model="from" type="date"
            class="mt-1 block rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 sm:text-sm" />
        </div>
        <div>
          <label for="conn-to" class="block text-sm font-medium text-gray-900">{{ t('connections.to') }}</label>
          <input id="conn-to" v-model="to" type="date"
            class="mt-1 block rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 sm:text-sm" />
        </div>
        <button type="submit"
          class="rounded-md bg-indigo-600 px-4 py-2 text-sm font-medium text-white hover:bg-indigo-500">{{ t('connections.search') }}</button>
        <button type="button" @click="exportCsv()"
          class="rounded-md border border-indigo-600 px-4 py-2 text-sm font-medium text-indigo-600 hover:bg-indigo-50">{{ t('connections.exportCsv') }}</button>
        <!-- Purge: administrators only (the server refuses it to anybody else anyway) -->
        <template v-if="isAdmin">
          <button type="button" :disabled="selected.size === 0" @click="askPurge('selected')"
            class="rounded-md border border-red-600 px-4 py-2 text-sm font-medium text-red-600 hover:bg-red-50 disabled:opacity-40">{{
              t('connections.purgeSelected', { count: selected.size }) }}</button>
          <button type="button" :disabled="total === 0" @click="askPurge('all')"
            class="rounded-md bg-red-600 px-4 py-2 text-sm font-medium text-white hover:bg-red-500 disabled:opacity-40">{{
              t('connections.purgeAll') }}</button>
        </template>
      </form>

      <div class="max-w-full overflow-x-auto">
        <table class="w-full table-auto">
          <thead class="bg-slate-400">
            <tr class="text-center">
              <th v-if="isAdmin" class="py-4 px-3">
                <input type="checkbox" :checked="allSelected" @change="toggleAll(($event.target as HTMLInputElement).checked)"
                  :aria-label="t('connections.selectAll')" />
              </th>
              <th class="py-4 px-3 text-lg font-medium text-white">{{ t('connections.started') }}</th>
              <th class="py-4 px-3 text-lg font-medium text-white">{{ t('connections.duration') }}</th>
              <th class="py-4 px-3 text-lg font-medium text-white">{{ t('connections.device') }}</th>
              <th class="py-4 px-3 text-lg font-medium text-white">{{ t('connections.controlledFrom') }}</th>
              <th class="py-4 px-3 text-lg font-medium text-white">{{ t('connections.user') }}</th>
              <th class="py-4 px-3 text-lg font-medium text-white">{{ t('connections.type') }}</th>
              <th class="py-4 px-3 text-lg font-medium text-white">{{ t('connections.address') }}</th>
              <th class="min-w-[260px] py-4 px-3 text-lg font-medium text-white">{{ t('connections.note') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="records.length === 0">
              <td :colspan="isAdmin ? 9 : 8" class="py-6 text-center text-gray-500">{{ t('connections.none') }}</td>
            </tr>
            <tr v-for="(r, i) in records" :key="r.guid" :class="i % 2 ? 'bg-white' : 'bg-[#F3F6FF]'">
              <td v-if="isAdmin" class="border-b border-[#E8E8E8] py-3 px-2 text-center">
                <input type="checkbox" :checked="selected.has(r.guid)" @change="toggle(r.guid)"
                  :aria-label="t('connections.select', { id: r.device_id })" />
              </td>
              <td class="border-b border-[#E8E8E8] py-3 px-2 text-center text-sm">{{ r.started_at }} UTC</td>
              <td class="border-b border-[#E8E8E8] py-3 px-2 text-center text-sm">{{ duration(r) }}</td>
              <td class="border-b border-[#E8E8E8] py-3 px-2 text-center text-sm">
                <!-- the name and the id open the native client; the globe next to them opens the web client -->
                <div class="inline-flex items-center gap-2">
                  <a :href="nativeClientUrl(r.device_id)"
                    :title="t('connect.nativeTitle', { device: r.device_name || r.device_id })"
                    class="text-indigo-600 hover:underline">{{ r.device_name || '' }}<br /><span>{{ r.device_id }}</span></a>
                  <button v-if="webClientAvailable()" type="button" @click="openWebClient(r.device_id)"
                    :title="t('connect.webTitle', { device: r.device_name || r.device_id })"
                    :aria-label="t('connect.webTitle', { device: r.device_name || r.device_id })"
                    class="inline-flex h-8 w-8 items-center justify-center rounded-md bg-indigo-600 text-white hover:bg-indigo-500">
                    <svg viewBox="0 0 24 24" class="h-5 w-5" fill="none" stroke="currentColor" stroke-width="1.8"
                      stroke-linecap="round" aria-hidden="true"><circle cx="12" cy="12" r="9" /><ellipse cx="12" cy="12" rx="4" ry="9" /><path d="M3 12h18" /></svg>
                  </button>
                </div>
              </td>
              <td class="border-b border-[#E8E8E8] py-3 px-2 text-center text-sm">
                {{ r.controller_name || '' }}<br /><span class="text-gray-500">{{ r.controller_id || '' }}</span>
              </td>
              <td class="border-b border-[#E8E8E8] py-3 px-2 text-center text-sm">{{ r.user || '' }}</td>
              <td class="border-b border-[#E8E8E8] py-3 px-2 text-center text-sm">{{ typeName(r.conn_type) }}</td>
              <td class="border-b border-[#E8E8E8] py-3 px-2 text-center text-sm">{{ r.ip || '' }}</td>
              <td class="border-b border-[#E8E8E8] py-3 px-2 text-sm">
                <textarea v-model="r.note" rows="2" @blur="saveNote(r)" :aria-label="t('connections.noteOf', { id: r.device_id })"
                  class="block w-full rounded-md border-0 py-1 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 sm:text-sm"></textarea>
                <span v-if="saved === r.guid" class="text-xs text-green-700">{{ t('connections.saved') }}</span>
                <span v-if="failed === r.guid" class="text-xs text-red-700">{{ t('connections.notSaved') }}</span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="mt-4 flex items-center justify-between text-sm">
        <span>{{ t('connections.count', { count: total }) }}</span>
        <div class="flex items-center gap-3">
          <button type="button" :disabled="page <= 1" @click="go(page - 1)"
            class="rounded-md border px-3 py-1 disabled:opacity-40">{{ t('common.previous') }}</button>
          <span>{{ t('connections.page', { page, pages }) }}</span>
          <button type="button" :disabled="page >= pages" @click="go(page + 1)"
            class="rounded-md border px-3 py-1 disabled:opacity-40">{{ t('common.next') }}</button>
        </div>
      </div>

      <!-- Confirmation of a purge: it deletes history for good -->
      <Modal v-if="purgeMode" :title="t('connections.purgeTitle')" :okLabel="t('common.delete')"
        @modalOk="purge()" @modalCancel="purgeMode = null">
        <p class="text-sm text-gray-700">{{ purgeMode === 'all' ? t('connections.confirmAll', { total }) :
          t('connections.confirmSelected', { count: selected.size }) }}</p>
      </Modal>
      <p v-if="purgeMessage" role="status" class="mt-3 text-sm" :class="purgeFailed ? 'text-red-700' : 'text-green-700'">{{ purgeMessage }}</p>
    </div>
  </section>
</template>

<script setup lang="ts">
import axios from 'axios';
import { computed, onMounted, ref } from 'vue';
import { useUserStore } from '@/stores/sctgDeskStore';
import Modal from '@/components/Modal.vue';
import { t } from '@/i18n';
import { nativeClientUrl, openWebClient, webClientAvailable } from '@/utilities/webclient';

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
const isAdmin = computed(() => !!(userStore.user as any)?.admin);
/** The guids ticked in the table (kept across searches, so a selection can span several pages). */
const selected = ref(new Set<string>());
/** Which purge the confirmation dialog is about; null when it is closed. */
const purgeMode = ref<'selected' | 'all' | null>(null);
const purgeMessage = ref('');
const purgeFailed = ref(false);
const allSelected = computed(() => records.value.length > 0 && records.value.every((r) => selected.value.has(r.guid)));
const pages = computed(() => Math.max(1, Math.ceil(total.value / PAGE_SIZE)));

/** The connection to the API: the base path and the token of the session of the console. */
function request(path: string, method: 'get' | 'put' | 'delete', params?: object, data?: object) {
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

function toggle(guid: string) {
  // a new Set each time: Vue does not track in-place mutations of a Set held in a ref reliably across versions
  const next = new Set(selected.value);
  next.has(guid) ? next.delete(guid) : next.add(guid);
  selected.value = next;
}

/** Ticks (or unticks) the rows of the current page only. */
function toggleAll(checked: boolean) {
  const next = new Set(selected.value);
  for (const r of records.value) {
    checked ? next.add(r.guid) : next.delete(r.guid);
  }
  selected.value = next;
}

function askPurge(mode: 'selected' | 'all') {
  purgeMessage.value = '';
  purgeMode.value = mode;
}

/**
 * Deletes the selected connections, or all of them, once the administrator confirmed.
 * The server decides who may do it (403 for the others), so the buttons being hidden is only a convenience.
 */
async function purge() {
  const mode = purgeMode.value;
  purgeMode.value = null;
  try {
    const body = mode === 'all' ? { all: true, guids: [] } : { all: false, guids: [...selected.value] };
    const response = await request('/api/audit/conn', 'delete', undefined, body);
    purgeFailed.value = false;
    purgeMessage.value = t('connections.purgeDone', { count: response.data.deleted ?? 0 });
    selected.value = new Set();
    page.value = 1;
    await load();
  } catch (error) {
    console.error(error);
    purgeFailed.value = true;
    purgeMessage.value = t('connections.purgeFailed');
  }
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

/** Translation keys of the connection types, indexed by the type number the server stores. */
const TYPES = ['remote', 'file', 'port', 'camera', 'terminal'];
function typeName(type: number): string {
  return TYPES[type] ? t(`connections.types.${TYPES[type]}`) : String(type);
}

/** The server gives UTC times as `YYYY-MM-DD HH:MM:SS`. */
function parse(time: string): number {
  return Date.parse(time.replace(' ', 'T') + 'Z');
}

function duration(record: Connection): string {
  if (!record.ended_at) {
    return t('connections.inProgress');
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
  const h = (key: string) => t(`connections.csvHeaders.${key}`);
  const lines = [['started', 'ended', 'device', 'deviceId', 'controlledFrom', 'user', 'type', 'address', 'note']
    .map((key) => quote(h(key))).join(',')];
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
