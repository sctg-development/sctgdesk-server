<!--
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
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
    <div class="flex flex-row items-center justify-center mx-auto">
      <div class="text-sm text-gray-600 dark:text-gray-400">
        © 2024 Ronan LE MEILLAT for SCTG Development - sctgdesk-api-server v{{ serverVersion }} - latest client version: v{{ clientVersion }}
      </div>
    </div>
    </footer>
</template>
<script setup lang="ts">
import { useVersionsStore } from '@/stores/versionsStore';
import { ref } from 'vue';
const serverVersion = ref(useVersionsStore().serverVersion);
const clientVersion = ref(useVersionsStore().clientVersion);
</script>