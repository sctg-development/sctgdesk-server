<!--
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
-->
<template>
    <div class="rounded-3xl bg-white text-slate-600 shadow-xl mt-10 p-10">
        <h2 class="text-2xl font-bold leading-9 tracking-tight text-gray-500">{{ t('backup.title') }}</h2>
        <p class="mt-4">{{ t('backup.intro') }}</p>
        <p class="mt-2 font-semibold text-red-600">{{ t('backup.warning') }}</p>
        <p class="mt-2 text-sm">{{ t('backup.requires') }}</p>
        <button type="button" :disabled="busy" @click="download"
            class="mt-6 rounded-md bg-indigo-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 disabled:opacity-50">
            {{ busy ? t('backup.preparing') : t('backup.download') }}
        </button>
        <p v-if="message" class="mt-4 text-sm" :class="failed ? 'text-red-600' : 'text-green-700'">{{ message }}</p>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { t } from '@/i18n';
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
            throw new Error(t('backup.disabled'));
        }
        if (response.status === 401) {
            throw new Error(t('backup.adminOnly'));
        }
        if (!response.ok) {
            throw new Error(t('backup.failed', { status: response.status }));
        }
        const disposition = response.headers.get('Content-Disposition') ?? '';
        const filename = /filename="([^"]+)"/.exec(disposition)?.[1] ?? 'sctgdesk-backup.tar.gz';
        const url = URL.createObjectURL(await response.blob());
        const link = document.createElement('a');
        link.href = url;
        link.download = filename;
        link.click();
        URL.revokeObjectURL(url);
        message.value = t('backup.saved', { filename });
    } catch (e) {
        failed.value = true;
        message.value = e instanceof Error ? e.message : String(e);
    } finally {
        busy.value = false;
    }
}
</script>
