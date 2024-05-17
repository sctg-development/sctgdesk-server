<!--
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
-->
<template>
  <div class="rounded-3xl bg-white text-slate-600 shadow-xl mt-10 p-10">
    <span>{{ msg }}</span>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  msg: string
}>()
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>

</style>
