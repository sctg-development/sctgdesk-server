<!--
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
-->
<template>
  <Modal @modalOk="addSharedAddressBook()" @modalCancel="closeModal()" okLabel="Add" title="Add shared address book">
    <div>
      <label for="name" class="block text-sm font-medium leading-6 text-gray-900">Shared address book name</label>
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
  </Modal>
</template>

<script setup lang="ts">
import Modal from '@/components/Modal.vue';
import { ref } from 'vue';
import { useUserStore } from '@/stores/sctgDeskStore';
import { AddressBookApi,AbSharedAddRequest } from '@/api';
const name = ref("");
const note = ref("");

const emit = defineEmits(['add_sharedab_close', 'shared_ab_added'])

/**
 * Closes the modal by emitting the 'add_sharedab_close' event.
 *
 * @return {void}
 */
function closeModal(): void {
  emit('add_sharedab_close')
}

/**
 * Adds a new group using the GroupApi service.
 *
 * @return {void} Emits 'group_added' event upon successful addition of the group.
 */
function addSharedAddressBook(): void {
  const addressApi = new AddressBookApi(useUserStore().api_configuration);
  const request: AbSharedAddRequest = {
    name: name.value,
    note: note.value,
  }
  addressApi.abSharedAdd(request).then(() => {
    emit('shared_ab_added')
  })
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped></style>
