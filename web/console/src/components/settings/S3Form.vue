<!--
=========================================================
* © 2024-2026 Ronan LE MEILLAT for SCTG Development
=========================================================
Guided form of s3config.toml: where the desktop clients are downloaded from.
The form model is a flat object keyed by the TOML keys of the `[s3config]` table.
-->
<template>
  <div>
    <p class="mb-4 text-sm text-gray-600">{{ t('settings.s3.intro') }}</p>
    <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
      <div v-for="f in fields" :key="f.key" :class="f.wide ? 'md:col-span-2' : ''">
        <label :for="'s3-' + f.key" class="block text-sm font-medium text-gray-900">{{ t('settings.s3.' + f.key) }}</label>
        <SecretInput v-if="f.secret" :id="'s3-' + f.key" v-model="model[f.key]" class="mt-1" />
        <input v-else :id="'s3-' + f.key" v-model="model[f.key]" type="text" spellcheck="false"
          class="mt-1 block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 sm:text-sm" />
        <p v-if="f.help" class="mt-1 text-xs text-gray-500">{{ t(f.help) }}</p>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { t } from '@/i18n'
import SecretInput from './SecretInput.vue'

/** The form model: the keys of the `[s3config]` table of s3config.toml. */
const model = defineModel<Record<string, string>>({ required: true })

/** The fields in display order. `help` is a message key; `secret` hides the value. */
const fields = [
  { key: 'Page', wide: true, help: 'settings.s3.PageHelp' },
  { key: 'Endpoint', help: 'settings.s3.EndpointHelp' },
  { key: 'Region', help: 'settings.s3.RegionHelp' },
  { key: 'AccessKey' },
  { key: 'SecretKey', secret: true },
  { key: 'Bucket', help: 'settings.s3.BucketHelp' },
  { key: 'ListBucket', help: 'settings.s3.ListBucketHelp' },
  { key: 'Windows64Key', wide: true, help: 'settings.s3.KeyHelp' },
  { key: 'Windows32Key', wide: true },
  { key: 'OSXKey', wide: true },
  { key: 'OSXArm64Key', wide: true },
  { key: 'IOSKey', wide: true },
]
</script>
