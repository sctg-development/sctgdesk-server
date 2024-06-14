<!--
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
-->
<template>
    <span ref="spanRef" :id="uniqueId">
        <slot>
        </slot>
    </span>
    <button ref="btnRef" :data-tooltip-target="`tp${uniqueId}`" data-tooltip-trigger="click" :data-clipboard-target="`#${uniqueId}`"
        :id="`btn${uniqueId}`">
        <img :id="`img${uniqueId}`" class="inline-block w-5 h-5 ml-2 cursor-pointer" src="@/assets/clippy.svg"
            :alt="props.altMsg" />
        <div :id="`tp${uniqueId}`" role="tooltip"
            class="absolute z-10 invisible inline-block px-3 py-2 text-sm font-medium text-white transition-opacity duration-300 bg-gray-900 rounded-lg shadow-sm opacity-0 tooltip dark:bg-gray-700">
            {{ props.msg }}
            <div class="tooltip-arrow" data-popper-arrow></div>
        </div>
    </button>
</template>
<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { generateUniqueId } from '@/utilities/viteHelper';
import { initFlowbite } from 'flowbite'
import ClipboardJS from 'clipboard';
export interface Props {
    msg?: string
    altMsg?: string
}
const props = withDefaults(defineProps<Props>(), {
    msg: 'Copied !',
    altMsg: 'Copy to clipboard'
})

const uniqueId = generateUniqueId();
const spanRef = ref<HTMLSpanElement | null>(null);
const btnRef = ref<HTMLButtonElement | null>(null);

onMounted(() => {
    initFlowbite();
    let clipboard = new ClipboardJS(btnRef.value);
    clipboard.on('success', function (e) {
        console.log('Copied !');
    });
});
</script>