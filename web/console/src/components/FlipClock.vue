<!--
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
-->
<template>
  <div ref="tick" class="text-6xl">
    <div data-repeat="true" aria-hidden="true">
      <span data-view="flip"></span>
    </div>
  </div>
</template>

<script setup lang="ts">
import Tick from "@pqina/flip";
import "@pqina/flip/dist/flip.min.css";
import { ref, defineProps, onMounted, onUnmounted, watch } from "vue";

const props = defineProps<{
  value: string
}>()
const tick = ref<HTMLDivElement>(null)
let _tick: { value: string; } //has many more properties but we use only this one

watch(() => props.value, (newValue) => { _tick.value = newValue })

onMounted(() => {
  _tick = Tick.DOM.create(tick.value, {
    value: props.value,
  });
})

onUnmounted(() => {
  Tick.DOM.destroy(tick.value);
})

</script>
