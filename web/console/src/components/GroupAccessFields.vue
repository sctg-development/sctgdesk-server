<!--
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
The groups a group may connect to, and the groups that may connect to it (the users of a group only see the users and
the devices of their own group and of the groups it may connect to).
-->
<template>
  <div class="grid grid-cols-2 gap-4">
    <fieldset>
      <legend class="block text-sm font-medium leading-6 text-gray-900">Can connect to</legend>
      <div v-if="others.length === 0" class="mt-2 text-sm text-gray-500">No other group</div>
      <div v-for="group in others" :key="'out-' + group.guid" class="mt-1 flex items-center">
        <input :id="'out-' + group.guid" type="checkbox" :value="group.guid" v-model="outgoing"
          class="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600" />
        <label :for="'out-' + group.guid" class="ml-2 text-sm text-gray-900">{{ group.name }}</label>
      </div>
    </fieldset>
    <fieldset>
      <legend class="block text-sm font-medium leading-6 text-gray-900">Can be reached by</legend>
      <div v-if="others.length === 0" class="mt-2 text-sm text-gray-500">No other group</div>
      <div v-for="group in others" :key="'in-' + group.guid" class="mt-1 flex items-center">
        <input :id="'in-' + group.guid" type="checkbox" :value="group.guid" v-model="incoming"
          class="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600" />
        <label :for="'in-' + group.guid" class="ml-2 text-sm text-gray-900">{{ group.name }}</label>
      </div>
    </fieldset>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useUserStore } from '@/stores/sctgDeskStore';
import { Group, GroupApi } from '@/api';

const props = withDefaults(defineProps<{
  /** guid of the group being edited, excluded from the choices; empty for a new group */
  self?: string;
}>(), { self: '' });

const outgoing = defineModel<string[]>('outgoing', { default: () => [] });
const incoming = defineModel<string[]>('incoming', { default: () => [] });

const groups = ref<Group[]>([]);
const others = computed(() => groups.value.filter((g) => g.guid !== props.self));

onMounted(() => {
  new GroupApi(useUserStore().api_configuration).groups(1, 4294967295).then((response) => {
    groups.value = response.data.data;
  });
});
</script>
