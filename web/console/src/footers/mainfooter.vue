<!--
=========================================================
* © 2024-2026 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
-->
<template>
    <footer
    id="footer"
    class="lg:fixed mt-4 p-4 bg-white sm:p-6 dark:bg-ambblue-800 font-neutra-demi inline-block w-full bottom-0 right-0"
  >
    <div class="flex flex-col items-center justify-center mx-auto">
      <div class="text-sm text-gray-600 dark:text-gray-400">
        {{ t('footer.line', { year: currentYear, serverVersion, clientVersion }) }}
      </div>
      <a :href="REPO_URL" target="_blank" rel="noopener noreferrer"
        class="text-sm text-indigo-600 hover:underline dark:text-indigo-400">{{ t('footer.github') }}</a>
      <div class="text-xs">
        <GithubButton owner="sctg-development" repo="sctgdesk-server"/>
      </div>
    </div>
    </footer>
</template>
<script setup lang="ts">
import GithubButton from '@/components/GithubButton.vue';
import { useVersionsStore } from '@/stores/versionsStore';
import { computed } from 'vue';
import { t } from '@/i18n';
import { REPO_URL } from '@/utilities/updateCheck';
const versions = useVersionsStore();
// computed, not a one-time copy: the versions are fetched asynchronously and may arrive after the first render
const serverVersion = computed(() => versions.serverVersion);
const clientVersion = computed(() => versions.clientVersion);
/** The copyright range ends with the current year, so it never needs a manual update. The project started in 2024. */
const currentYear = new Date().getFullYear();
</script>