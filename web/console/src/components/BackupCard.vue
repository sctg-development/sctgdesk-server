<!--
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
-->
<template>
    <div class="rounded-3xl bg-white text-slate-600 shadow-xl mt-10 p-10">
        <h2 class="text-2xl font-bold leading-9 tracking-tight text-gray-500">Backup of the server</h2>
        <p class="mt-4">
            Downloads an archive (<code>.tar.gz</code>) with a consistent copy of the database, the configuration
            files (<code>oauth2.toml</code>, <code>s3config.toml</code>) and the key pair of the server.
        </p>
        <p class="mt-2 font-semibold text-red-600">
            The archive holds the private key and the secrets of the server: keep it encrypted and private. Each
            download is recorded in the log of the server.
        </p>
        <p class="mt-2 text-sm">
            It works only if <code>hbbs</code> was started with <code>--allow-remote-backup</code> (or
            <code>ALLOW_REMOTE_BACKUP=Y</code>).
        </p>
        <button type="button" :disabled="busy" @click="download"
            class="mt-6 rounded-md bg-indigo-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 disabled:opacity-50">
            {{ busy ? 'Preparing the backup…' : 'Download a backup' }}
        </button>
        <p v-if="message" class="mt-4 text-sm" :class="failed ? 'text-red-600' : 'text-green-700'">{{ message }}</p>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useUserStore } from '@/stores/sctgDeskStore';

const userStore = useUserStore();
const busy = ref(false);
const failed = ref(false);
const message = ref('');

/**
 * Downloads the backup with the token of the administrator and hands it to the browser as a file.
 *
 * @return {Promise<void>} resolves when the file is saved or the error is displayed.
 */
async function download(): Promise<void> {
    busy.value = true;
    failed.value = false;
    message.value = '';
    try {
        const configuration = userStore.api_configuration;
        const response = await fetch(`${configuration?.basePath ?? ''}/api/backup`, {
            headers: { Authorization: `Bearer ${configuration?.accessToken}` },
        });
        if (response.status === 404) {
            throw new Error('The backup is disabled on this server: start hbbs with --allow-remote-backup.');
        }
        if (response.status === 401) {
            throw new Error('Only an administrator can download a backup.');
        }
        if (!response.ok) {
            throw new Error(`The server could not make the backup (HTTP ${response.status}).`);
        }
        const disposition = response.headers.get('Content-Disposition') ?? '';
        const filename = /filename="([^"]+)"/.exec(disposition)?.[1] ?? 'sctgdesk-backup.tar.gz';
        const url = URL.createObjectURL(await response.blob());
        const link = document.createElement('a');
        link.href = url;
        link.download = filename;
        link.click();
        URL.revokeObjectURL(url);
        message.value = `Saved ${filename}.`;
    } catch (e) {
        failed.value = true;
        message.value = e instanceof Error ? e.message : String(e);
    } finally {
        busy.value = false;
    }
}
</script>
