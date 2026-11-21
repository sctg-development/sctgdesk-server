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
                          <!-- gap-2 keeps the connect and delete buttons from touching -->
                          <div class="inline-flex flex-wrap items-center justify-center gap-2">
                            <ConnectButtons :device-id="peer.id" :device-name="peer.info.hostname" />
                            <button v-if="isAdmin" type="button" @click="toDelete = peer"
                              class="inline-block px-4 py-1.5 border rounded-md border-red-600 text-red-600 hover:bg-red-600 hover:text-white font-medium">
                              {{ t('devices.delete') }}
                            </button>
                          </div>
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
        <Modal v-if="toDelete" :title="t('devices.deleteTitle')" :okLabel="t('devices.delete')"
          @modalOk="deletePeer()" @modalCancel="toDelete = null">
          <p class="text-sm text-gray-700">{{ t('devices.confirmDelete', { device: toDelete.info?.hostname || toDelete.id, id: toDelete.id }) }}</p>
        </Modal>
        <p v-if="deleteMessage" role="status" class="mt-3 text-sm" :class="deleteFailed ? 'text-red-700' : 'text-green-700'">{{ deleteMessage }}</p>
</template>
<script setup lang="ts">
import { Peer, PeerApi } from '@/api';
import { useUserStore } from '@/stores/sctgDeskStore';
import { computed, onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';
import ClipboardButton from './ClipboardButton.vue';
import { t } from '@/i18n';
import Modal from '@/components/Modal.vue';
import { apiRequest } from '@/utilities/http';
import ConnectButtons from '@/components/ConnectButtons.vue';
import { openWebClient, webClientAvailable } from '@/utilities/webclient';

const userStore = useUserStore();
const router = useRouter();

const peers = ref([] as Peer[]);
/** Deleting a device is for the administrators (the server refuses the others anyway). */
const isAdmin = computed(() => !!userStore.user?.admin);
/** The device waiting for the confirmation of its deletion. */
const toDelete = ref<Peer | null>(null);
const deleteMessage = ref('');
const deleteFailed = ref(false);

/**
 * Deletes the confirmed device on the server and removes it from the table. The device registers itself
 * again if it is still running, which the confirmation text says.
 */
async function deletePeer(): Promise<void> {
  const peer = toDelete.value;
  toDelete.value = null;
  if (!peer) {
    return;
  }
  try {
    await apiRequest('delete', `/api/peers/${encodeURIComponent(peer.id)}`);
    peers.value = peers.value.filter((p) => p.id !== peer.id);
    deleteFailed.value = false;
    deleteMessage.value = t('devices.deleted', { id: peer.id });
  } catch (error) {
    console.error(error);
    deleteFailed.value = true;
    deleteMessage.value = t('devices.deleteFailed');
  }
}

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