<!--
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
-->
<template>
    <div :class="props.class">
        <!-- ====== Table Section Start -->
        <section class="bg-white dark:bg-dark">
            <div class="container mx-auto">
                <div class="flex flex-wrap -mx-4">
                    <div class="w-full px-4">
                        <div class="max-w-full overflow-x-auto min-h-48">
                            <p>{{ name }}</p>
                            <table class="w-full table-auto">
                                <thead class="bg-slate-400">
                                    <tr class="text-center bg-primary">
                                        <th
                                            class="w-1/6 min-w-[160px] border-l border-transparent py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                                            Id
                                        </th>
                                        <th
                                            class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                                            Alias
                                        </th>
                                        <th
                                            class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                                            Hostname
                                        </th>
                                        <th
                                            class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                                            Login
                                        </th>
                                        <th
                                            class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                                            Platform
                                        </th>
                                        <th
                                            class="w-1/6 min-w-[160px] border-r border-transparent py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                                            Username
                                        </th>
                                        <th>
                                            <Menu as="div" class="relative inline-block text-left m-1">
                                                <div>
                                                    <MenuButton
                                                        class="inline-flex w-full justify-center rounded-md bg-black/20 px-4 py-2 text-sm font-medium text-white hover:bg-black/30 focus:outline-none focus-visible:ring-2 focus-visible:ring-white/75">
                                                        Actions
                                                    </MenuButton>
                                                </div>
                                                <transition enter-active-class="transition duration-100 ease-out"
                                                    enter-from-class="transform scale-95 opacity-0"
                                                    enter-to-class="transform scale-100 opacity-100"
                                                    leave-active-class="transition duration-75 ease-in"
                                                    leave-from-class="transform scale-100 opacity-100"
                                                    leave-to-class="transform scale-95 opacity-0">
                                                    <MenuItems
                                                        class="absolute right-0 mt-2 w-60 origin-top-right divide-y divide-gray-100 rounded-md bg-white shadow-lg ring-1 ring-black/5 focus:outline-none">
                                                        <div class="px-1 py-1">
                                                            <MenuItem v-slot="{ active }" v-if="!props.isPersonal">
                                                            <button :class="[
                                                                active ? 'bg-slate-400 text-white' : 'text-gray-900',
                                                                'group flex w-full items-center rounded-md px-2 py-2 text-sm',
                                                            ]" @click="isViewRulesVisible = true">
                                                                View rules
                                                            </button>
                                                            </MenuItem>
                                                            <MenuItem v-slot="{ active }" v-if="!props.isPersonal">
                                                            <button :class="[
                                                                active ? 'bg-slate-400 text-white' : 'text-gray-900',
                                                                'group flex w-full items-center rounded-md px-2 py-2 text-sm',
                                                            ]" @click="isAddRulesVisible = true">
                                                                Add rule
                                                            </button>
                                                            </MenuItem>
                                                            <MenuItem v-slot="{ active }" v-if="!props.isPersonal">
                                                            <button :class="[
                                                                active ? 'bg-slate-400 text-white' : 'text-gray-900',
                                                                'group flex w-full items-center rounded-md px-2 py-2 text-sm',
                                                            ]" @click="deleteAddressBook()">
                                                                Delete address book
                                                            </button>
                                                            </MenuItem>
                                                        </div>
                                                    </MenuItems>
                                                </transition>
                                            </Menu>
                                        </th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <tr v-for="peer in peers" :key="peer.id">
                                        <td
                                            class="text-dark border-b border-l border-[#E8E8E8] bg-[#F3F6FF] dark:bg-dark-3 dark:border-dark dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                                            <ClipboardButton>{{ peer.id }}</ClipboardButton>
                                        </td>
                                        <td
                                            class="text-dark border-b border-[#E8E8E8] bg-white dark:border-dark dark:bg-dark-2 dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                                            {{ peer.alias }}
                                        </td>
                                        <td
                                            class="text-dark border-b border-[#E8E8E8] bg-[#F3F6FF] dark:bg-dark-3 dark:border-dark dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                                            {{ peer.hostname }}
                                        </td>
                                        <td
                                            class="text-dark border-b border-[#E8E8E8] bg-white dark:border-dark dark:bg-dark-2 dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                                            {{ peer.login_name }}
                                        </td>
                                        <td
                                            class="text-dark border-b border-[#E8E8E8] bg-[#F3F6FF] dark:bg-dark-3 dark:border-dark dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                                            {{ peer.platform }}
                                        </td>
                                        <td
                                            class="text-dark border-b border-[#E8E8E8] bg-white dark:bg-dark-3 dark:border-dark dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                                            {{ peer.username }}
                                        </td>
                                        <td
                                            class="text-dark border-b border-r border-[#E8E8E8] bg-[#F3F6FF] dark:border-dark dark:bg-dark-2 dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                                        </td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    </div>
    <!-- ====== Table Section End -->
    <ViewRules v-if="isViewRulesVisible" :ab="props.ab" @viewRulesOK="isViewRulesVisible = false"
        @viewRulesCancel="isViewRulesVisible = false" />
    <AddRule v-if="isAddRulesVisible" :ab="props.ab" @addRuleOK="isAddRulesVisible = false"
        @add-rule-cancel="isAddRulesVisible = false"></AddRule>
</template>
<script setup lang="ts">
import { Menu, MenuButton, MenuItem, MenuItems } from '@headlessui/vue'
import { AbPeer } from '@/api';
import { ref } from 'vue';
import ViewRules from '@/components/ViewRules.vue';
import AddRule from '@/components/AddRule.vue';
import ClipboardButton from '@/components/ClipboardButton.vue';
import { useUserStore } from '@/stores/sctgDeskStore';
import { AddressBookApi } from '@/api';

const isViewRulesVisible = ref(false);
const isAddRulesVisible = ref(false);
const emit = defineEmits(['needRefresh'])


const props = withDefaults(defineProps<{
    name: string,
    ab: string,
    peers: AbPeer[],
    isPersonal?: boolean,
    class?: string,
}>(), {
    isPersonal: false,
    class: "",
})

function deleteAddressBook() {
    console.log("Delete address book")
    if (confirm("Are you sure you want to delete this address book ?")) {
        const addressBookApi = new AddressBookApi(useUserStore().api_configuration);
        addressBookApi.abSharedDelete([props.ab]).then(() => {
            emit('needRefresh');
            console.log("Address book was deleted");
        }).catch((error) => {
            console.error("Error deleting address book", error)
        })
    }
}
</script>