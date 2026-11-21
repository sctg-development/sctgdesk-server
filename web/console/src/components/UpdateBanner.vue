<!--
=========================================================
* © 2024-2026 Ronan LE MEILLAT for SCTG Development
=========================================================
Banner shown at the top of the console when GitHub has a server release newer than the one that runs.
- clicking the text opens the release on GitHub;
- "Close" hides it until the page is reloaded;
- "Do not show again" hides it for good (a cookie, see utilities/updateCheck.ts).
Nothing is shown while the check is running or when it fails.
-->
<template>
  <div v-if="release && !closed" role="status"
    class="flex flex-wrap items-center justify-center gap-x-4 gap-y-1 bg-indigo-600 px-4 py-2 text-sm text-white">
    <a :href="release.url" target="_blank" rel="noopener noreferrer" class="font-medium hover:underline">
      {{ t('update.available', { latest: release.version, current: versions.serverVersion ?? '' }) }}
    </a>
    <span class="flex items-center gap-3">
      <button type="button" class="underline hover:no-underline" @click="hideForever">{{ t('update.dontShowAgain') }}</button>
      <button type="button" :aria-label="t('common.close')" :title="t('common.close')" class="text-lg leading-none hover:opacity-80"
        @click="closed = true">&times;</button>
    </span>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { t } from '@/i18n'
import { useVersionsStore } from '@/stores/versionsStore'
import { bannerHidden, fetchLatestRelease, hideBannerForever, isNewer, type LatestRelease } from '@/utilities/updateCheck'

const versions = useVersionsStore()
/** Set only when a newer release exists and the user did not opt out. */
const release = ref<LatestRelease | null>(null)
const closed = ref(false)

function hideForever(): void {
  hideBannerForever()
  closed.value = true
}

onMounted(async () => {
  if (bannerHidden()) {
    return // no need to call GitHub for a banner that will not be shown
  }
  await versions.fetchVersions()
  const latest = await fetchLatestRelease()
  if (latest && versions.serverVersion && isNewer(latest.version, versions.serverVersion)) {
    release.value = latest
  }
})
</script>
