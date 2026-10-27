<!--
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
-->
<template>
  <Modal @modalOk="updateAb()" @modalCancel="closeModal()" okLabel="Update" title="Update address book">
    <div>
      <label for="name" class="block text-sm font-medium leading-6 text-gray-900">Address book name</label>
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
import { onMounted, ref } from 'vue';
import { useUserStore } from '@/stores/sctgDeskStore';
import { AddressBookApi, AbSharedNameRequest, AbProfile } from '@/api';
const name = ref("");
const note = ref("");

const props = withDefaults(defineProps<{
    uuid: string;
}>(),
    {
        uuid: ''
    })

const emit = defineEmits(['update_ab_edit_close', 'ab_updated'])

onMounted(() => {
  const addressBookApi = new AddressBookApi(useUserStore().api_configuration);
  addressBookApi.abShared().then((response) => {
    response.data.data.forEach((ab:AbProfile) => {
      if (ab.guid === props.uuid) {
        name.value = ab.name;
        note.value = ab.note;
      }
    })
  })
})
/**
 * Closes the modal by emitting the 'add_group_close' event.
 *
 * @return {void}
 */
function closeModal(): void {
  emit('update_ab_edit_close')
}

/**
 * Updates a new group using the GroupApi service.
 *
 * @return {void} Emits 'group_updated' event upon successful update of the group.
 */
function updateAb(): void {
  const addressBookApi = new AddressBookApi(useUserStore().api_configuration);
  const request = {
    guid: props.uuid,
    name: name.value,
    note: note.value
  } as AbSharedNameRequest;
  addressBookApi.abSharedName(request).then(() => {
    emit('ab_updated')
  })

}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped></style>
