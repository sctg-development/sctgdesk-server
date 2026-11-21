<!--
=========================================================
* © 2024-2026 Ronan LE MEILLAT for SCTG Development
=========================================================
Server settings page (administrators only): the S3 download configuration and the OAuth2 providers.
The server refuses these endpoints to non-administrators; the check here only avoids showing a page that
cannot work.
-->
<template>
  <section class="rounded-3xl bg-white p-6 text-slate-600 shadow-xl">
    <p v-if="!isAdmin" class="text-sm text-gray-600">{{ t('settings.adminOnly') }}</p>
    <template v-else>
      <div class="mb-6 border-b border-gray-200" role="tablist">
        <button v-for="tab in tabs" :key="tab.key" type="button" role="tab" :aria-selected="current === tab.key"
          @click="current = tab.key"
          :class="current === tab.key ? 'border-indigo-600 text-indigo-600' : 'border-transparent text-gray-500 hover:text-gray-700'"
          class="-mb-px mr-4 border-b-2 px-1 pb-3 text-sm font-medium">{{ t(tab.label) }}</button>
      </div>
      <!-- keyed by tab: switching tabs reloads a fresh editor instead of mixing two files' state -->
      <ConfigEditor :key="current" :kind="current" />
    </template>
  </section>
</template>
<script setup lang="ts">
import { computed, ref } from 'vue'
import { t } from '@/i18n'
import { useUserStore } from '@/stores/sctgDeskStore'
import ConfigEditor from '@/components/settings/ConfigEditor.vue'

const isAdmin = computed(() => !!(useUserStore().user as any)?.admin)
const tabs = [
  { key: 's3' as const, label: 'settings.tabS3' },
  { key: 'oauth2' as const, label: 'settings.tabOauth2' },
]
const current = ref<'s3' | 'oauth2'>('s3')
</script>
