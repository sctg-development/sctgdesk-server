<!--
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
-->
<template>
  <Modal @modalOk="updateGroup()" @modalCancel="closeModal()" okLabel="Update" title="Update group">
    <div>
      <label for="name" class="block text-sm font-medium leading-6 text-gray-900">Group name</label>
      <div class="mt-2">
        <input v-model="name" id="name" name="name" type="text" required
          class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
      </div>
    </div>
    <div>
      <label for="note" class="block text-sm font-medium leading-6 text-gray-900">Note</label>
      <div class="mt-2">
        <input v-model="note" id="note" name="note" type="text" required
          class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
      </div>
    </div>
    <GroupAccessFields v-model:outgoing="allowedOutgoings" v-model:incoming="allowedIncomings" :self="uuid" />
  </Modal>
</template>

<script setup lang="ts">
import Modal from '@/components/Modal.vue';
import GroupAccessFields from '@/components/GroupAccessFields.vue';
import { onMounted, ref } from 'vue';
import { useUserStore } from '@/stores/sctgDeskStore';
import { GroupApi, UpdateGoupRequest } from '@/api';
const name = ref("");
const note = ref("");
const allowedOutgoings = ref<string[]>([]);
const allowedIncomings = ref<string[]>([]);

const props = withDefaults(defineProps<{
    uuid: string;
}>(),
    {
        uuid: ''
    })

const emit = defineEmits(['update_group_close', 'group_updated'])

onMounted(() => {
  const groupApi = new GroupApi(useUserStore().api_configuration);
  groupApi.groupGet(props.uuid).then((response) => {
    name.value = response.data.name;
    note.value = response.data.note;
    allowedOutgoings.value = response.data.access_to;
    allowedIncomings.value = response.data.accessed_from;
  })
})
/**
 * Closes the modal by emitting the 'add_group_close' event.
 *
 * @return {void}
 */
function closeModal(): void {
  emit('update_group_close')
}

/**
 * Updates a new group using the GroupApi service.
 *
 * @return {void} Emits 'group_updated' event upon successful update of the group.
 */
function updateGroup(): void {
  const groupApi = new GroupApi(useUserStore().api_configuration);
  const request: UpdateGoupRequest = {
    guid: props.uuid,
    name: name.value,
    note: note.value,
    allowed_incomings: allowedIncomings.value,
    allowed_outgoings: allowedOutgoings.value,
  }
  groupApi.groupUpdate(request).then(() => {
    emit('group_updated')
  })
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped></style>
