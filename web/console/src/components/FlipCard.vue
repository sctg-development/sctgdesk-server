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
    <flip-clock :value="counter !== undefined ? counter : '00000'" />
  </div>
</template>

<script setup lang="ts">
import FlipClock from "@/components/FlipClock.vue";
import { ref, onMounted } from "vue";
const counter = ref('000000')
onMounted(() => {
  setInterval(updateCounter, 2000);
})
const zeroPad = (num: number, places: number): string => {
  return String(num).padStart(places, "0");
}
const updateCounter = () => {
  counter.value = zeroPad(Math.floor(Math.random() * 100000), 5);
}
</script>

