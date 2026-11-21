<!--
=========================================================
* © 2024-2026 Ronan LE MEILLAT for SCTG Development
=========================================================
A text input for a secret (key, password) hidden by default, with a button to reveal it.
-->
<template>
  <div class="relative">
    <input :id="id" :type="shown ? 'text' : 'password'" :value="modelValue" autocomplete="off" spellcheck="false"
      @input="emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      class="block w-full rounded-md border-0 py-1.5 pr-16 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 sm:text-sm" />
    <button type="button" @click="shown = !shown" :aria-label="shown ? t('settings.secretHide') : t('settings.secretShow')"
      class="absolute inset-y-0 right-0 px-3 text-xs text-gray-500 hover:text-gray-900">
      {{ shown ? t('common.hide') : t('common.show') }}
    </button>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
import { t } from '@/i18n'

defineProps<{ modelValue: string; id?: string }>()
const emit = defineEmits<{ (e: 'update:modelValue', value: string): void }>()
const shown = ref(false)
</script>
