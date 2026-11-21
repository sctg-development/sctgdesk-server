<!--
=========================================================
* © 2024-2026 Ronan LE MEILLAT for SCTG Development
=========================================================
The "connect to this device" buttons, shared by the dashboard, the devices and the connections pages so that
they all behave the same way:
- the SCTGDesk logo opens the installed native client (a `rustdesk://<id>` link, handled by the operating system:
  always offered, since the console cannot know whether the client is installed);
- the globe opens the web client, and is only shown when the server has a valid WEB_CLIENT_URL.
Icons only, with a tooltip and an accessible name: the buttons sit in table cells where a text would be too long.
-->
<template>
  <span class="inline-flex items-center gap-1.5 align-middle m-0.5">
    <a :href="nativeClientUrl(deviceId)" :title="nativeTitle" :aria-label="nativeTitle" data-test="connect-native"
      class="inline-flex h-8 w-8 items-center justify-center rounded-md border border-gray-300 bg-white hover:bg-gray-50">
      <img :src="logo" alt="" class="h-5 w-5" />
    </a>
    <button v-if="webClientAvailable()" type="button" :title="webTitle" :aria-label="webTitle" data-test="connect-web"
      @click="openWebClient(deviceId)"
      class="inline-flex h-8 w-8 items-center justify-center rounded-md bg-indigo-600 text-white hover:bg-indigo-500">
      <!-- a globe: circle, meridian ellipse and equator -->
      <svg viewBox="0 0 24 24" class="h-5 w-5" fill="none" stroke="currentColor" stroke-width="1.8"
        stroke-linecap="round" aria-hidden="true">
        <circle cx="12" cy="12" r="9" />
        <ellipse cx="12" cy="12" rx="4" ry="9" />
        <path d="M3 12h18" />
      </svg>
    </button>
  </span>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { t } from '@/i18n'
import logo from '@/assets/sctg.svg'
import { nativeClientUrl, openWebClient, webClientAvailable } from '@/utilities/webclient'

const props = defineProps<{
  /** The device to connect to. */
  deviceId: string
  /** The name shown in the tooltips; the id is used when the device has no name. */
  deviceName?: string
}>()

const label = computed(() => props.deviceName || props.deviceId)
const nativeTitle = computed(() => t('connect.nativeTitle', { device: label.value }))
const webTitle = computed(() => t('connect.webTitle', { device: label.value }))
</script>
