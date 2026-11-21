<!--
=========================================================
* © 2024-2026 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
-->
<template>
        <!-- ====== Table Section Start -->
        <section class="bg-white dark:bg-dark">
          <div class="container mx-auto">
            <div v-if="webClientAvailable()" class="mb-4 flex justify-end">
              <button type="button" @click="openWebClient()"
                class="rounded-md bg-indigo-600 px-4 py-2 text-sm font-medium text-white hover:bg-indigo-500">
                {{ t('devices.openWebClient') }}
              </button>
            </div>
            <div class="flex flex-wrap -mx-4">
              <div class="w-full px-4">
                <div class="max-w-full overflow-x-auto">
                  <table class="w-full table-auto">
                    <thead class="bg-slate-400">
                      <tr class="text-center bg-primary">
                        <th
                          class="w-1/6 min-w-[160px] border-l border-transparent py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                          {{ t('devices.id') }}
                        </th>
                        <th class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                          {{ t('devices.status') }}
                        </th>
                        <th class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                          {{ t('devices.username') }}
                        </th>
                        <th class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                          {{ t('devices.os') }}
                        </th>
                        <th class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                          {{ t('devices.ip') }}
                        </th>
                        <th
                          class="w-1/6 min-w-[160px] border-r border-transparent py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                          {{ t('devices.cpu') }}
                        </th>
                        <th></th>
                      </tr>
                    </thead>
                    <tbody>
                      <tr v-for="peer in peers" :key="peer.guid">
                        <td
                          class="text-dark border-b border-l border-[#E8E8E8] bg-[#F3F6FF] dark:bg-dark-3 dark:border-dark dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                          <ClipboardButton>{{ peer.id }}</ClipboardButton>
                        </td>
                        <td
                          class="text-dark border-b border-[#E8E8E8] bg-white dark:border-dark dark:bg-dark-2 dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                          {{ peer.status }}<br/>{{ peer.last_online }}
                        </td>
                        <td
                          class="text-dark border-b border-[#E8E8E8] bg-[#F3F6FF] dark:bg-dark-3 dark:border-dark dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                          {{ peer.info.username }}
                        </td>
                        <td
                          class="text-dark border-b border-[#E8E8E8] bg-white dark:border-dark dark:bg-dark-2 dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                          {{ peer.info.os }}
                        </td>
                        <td
                          class="text-dark border-b border-[#E8E8E8] bg-[#F3F6FF] dark:bg-dark-3 dark:border-dark dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                          {{ peer.info.ip }}
                        </td>
                        <td
                          class="text-dark border-b border-[#E8E8E8] bg-white dark:bg-dark-3 dark:border-dark dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                          {{ peer.info.cpu }}
                        </td>
                        <td
                          class="text-dark border-b border-r border-[#E8E8E8] bg-[#F3F6FF] dark:border-dark dark:bg-dark-2 dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                          <button v-if="webClientAvailable()" type="button" @click="openWebClient(peer.id)"
                            :title="t('devices.connectTitle', { device: peer.info.hostname || peer.id })"
                            class="inline-block mr-2 px-6 py-2.5 rounded-md bg-indigo-600 text-white hover:bg-indigo-500 font-medium">
                            {{ t('devices.connect') }}
                          </button>
                          <a href="javascript:alert('todo')"
                            class="inline-block px-6 py-2.5 border rounded-md border-primary text-primary hover:bg-primary hover:text-white font-medium">
                            {{ t('devices.delete') }}
                          </a>
                        </td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </div>
            </div>
          </div>
        </section>
        <!-- ====== Table Section End -->
</template>
<script setup lang="ts">
import { Peer, PeerApi } from '@/api';
import { useUserStore } from '@/stores/sctgDeskStore';
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';
import ClipboardButton from './ClipboardButton.vue';
import { t } from '@/i18n';
import { openWebClient, webClientAvailable } from '@/utilities/webclient';

const userStore = useUserStore();
const router = useRouter();

const peers = ref([] as Peer[]);

onMounted(() => {
  getPeers().then((data) => {
    peers.value = data;
  });
});

/**
 * Retrieves a list of peers from the PeerApi.
 *
 * @return {Promise<Peer[]>} A promise that resolves to an array of Peer objects.
 */
function getPeers(): Promise<Peer[]> {
  const peerApi = new PeerApi(userStore.api_configuration);
  return new Promise<Peer[]>((resolve, reject) => {
    peerApi.peers().then((response) => {
      if (response.status == 200 && response.data.msg == "success") {
        resolve(response.data.data);
      }
      else {
        resolve([] as Peer[]);
      }
    }).catch((error) => {
      console.error(error);
      resolve([] as Peer[]);
    });
  });
}
</script>