<template>
    <footer
    id="footer"
    class="lg:fixed mt-4 p-4 bg-white sm:p-6 dark:bg-ambblue-800 font-neutra-demi inline-block w-full bottom-0 right-0"
  >
    <div class="flex flex-col items-center justify-center">
      <p class="text-sm text-gray-600 dark:text-gray-400">
        © 2024 Ronan LE MEILLAT for SCTG Development - sctgdesk-api-server v{{ serverVersion }} - latest client version: v{{ clientVersion }}
      </p>
    </div>
    </footer>
</template>
<script setup lang="ts">
import { useVersionsStore } from '@/stores/versionsStore';
import { ref } from 'vue';
const serverVersion = ref(useVersionsStore().serverVersion);
const clientVersion = ref(useVersionsStore().clientVersion);
</script>