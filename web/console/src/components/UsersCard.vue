<!--
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
-->
<template>
    <!-- ====== Table Section Start -->
    <section class="bg-white dark:bg-dark">
        <div class="container mx-auto">
            <div class="flex flex-wrap -mx-4">
                <div class="w-full px-4">
                    <div class="max-w-full overflow-x-auto">
                        <table class="w-full table-auto">
                            <thead class="bg-slate-400">
                                <tr class="text-center bg-primary">
                                    <th
                                        class="w-1/6 min-w-[160px] border-l border-transparent py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                                        Guid
                                    </th>
                                    <th
                                        class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                                        Name
                                    </th>
                                    <th
                                        class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                                        Email
                                    </th>
                                    <th
                                        class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                                        Status
                                    </th>
                                    <th
                                        class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                                        Admin
                                    </th>
                                    <th
                                        class="w-1/6 min-w-[160px] border-r border-transparent py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                                        Note
                                    </th>
                                    <th>
                                        <Menu as="div" class="relative inline-block text-left">
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
                                                    class="absolute right-0 mt-2 w-56 origin-top-right divide-y divide-gray-100 rounded-md bg-white shadow-lg ring-1 ring-black/5 focus:outline-none">
                                                    <div class="px-1 py-1">
                                                        <MenuItem v-slot="{ active }">
                                                        <button @click="toggle_add_user" :class="[
                                                            active ? 'bg-slate-400 text-white' : 'text-gray-900',
                                                            'group flex w-full items-center rounded-md px-2 py-2 text-sm',
                                                        ]">
                                                            Add user
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
                                <tr v-for="(user, index) in users" :key="user.guid">
                                    <td
                                        class="text-dark border-b border-l border-[#E8E8E8] bg-[#F3F6FF] dark:bg-dark-3 dark:border-dark dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                                        <ClipboardButton msg="Copied !">{{ user.guid }}</ClipboardButton>
                                    </td>
                                    <td
                                        class="text-dark border-b border-[#E8E8E8] bg-white dark:border-dark dark:bg-dark-2 dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                                        {{ user.name }}
                                    </td>
                                    <td
                                        class="text-dark border-b border-[#E8E8E8] bg-[#F3F6FF] dark:bg-dark-3 dark:border-dark dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                                        {{ user.email }}
                                    </td>
                                    <td
                                        class="text-dark border-b border-[#E8E8E8] bg-white dark:border-dark dark:bg-dark-2 dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                                        {{ user.status == 1 ? 'Active' : 'Inactive' }}
                                    </td>
                                    <td
                                        class="text-dark border-b border-[#E8E8E8] bg-[#F3F6FF] dark:bg-dark-3 dark:border-dark dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                                        {{ user.is_admin ? 'Yes' : 'No' }}
                                    </td>
                                    <td
                                        class="text-dark border-b border-[#E8E8E8] bg-white dark:bg-dark-3 dark:border-dark dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                                        {{ user.note }}
                                    </td>
                                    <td
                                        class="text-dark border-b border-r border-[#E8E8E8] bg-[#F3F6FF] dark:border-dark dark:bg-dark-2 dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                                        <a @click="toggle_edit_user(user.name, user.guid)"
                                            class="inline-block px-6 py-2.5 border rounded-md border-primary text-primary hover:bg-primary hover:text-white font-medium m-1">
                                            Edit
                                        </a>
                                        <a @click="toggle_user(user.name, user.guid, user.status == 1 ? false : true)"
                                            class="inline-block px-6 py-2.5 border rounded-md border-primary text-primary hover:bg-primary hover:text-white font-medium m-1">
                                            {{ user.status == 1 ? 'Deactivate' : 'Activate' }}
                                        </a>
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
    </section>
    <!-- ====== Table Section End -->
    <AddUser @add_user_close="toggle_add_user" @user_added="refresh_users" v-if="bModalAddUser" />
    <EditUser v-if="bModalEditUser" @update_user_close="toggle_edit_user" :username="editUserName"
        :uuid="editUserUuid" />
</template>
<script setup lang="ts">
import { Menu, MenuButton, MenuItem, MenuItems } from '@headlessui/vue'
import { UserApi, EnableUserRequest } from '@/api';
import { useUserStore } from '@/stores/sctgDeskStore';
import { onBeforeMount, onMounted, onUpdated, ref } from 'vue';
import AddUser from '@/components/AddUser.vue';
import EditUser from '@/components/EditUser.vue';
import { getUsers } from '@/utilities/api';
import ClipboardButton from './ClipboardButton.vue';
const userStore = useUserStore();
const users = ref([]);
const bModalAddUser = ref(false);
const bModalEditUser = ref(false);
const editUserName = ref("");
const editUserUuid = ref("");

onBeforeMount(() => {
    refresh_users();
});

onUpdated(() => {

});

onMounted(() => {

});

/**
 * Toggles the value of `bModalAddUser` between `true` and `false`.
 *
 * @return {void} This function does not return anything.
 */
function toggle_add_user(): void {
    bModalAddUser.value = !bModalAddUser.value;
    refresh_users();
}

/**
 * Toggles the value of `bModalEditUser` between `true` and `false`.
 *
 * @return {void} This function does not return anything.
 */
function toggle_edit_user(username?: string, uuid?: string): void {
    editUserName.value = username || "";
    editUserUuid.value = uuid || "";
    console.log(`Edit user: ${editUserName.value} (${editUserUuid.value})`)
    bModalEditUser.value = !bModalEditUser.value;
    refresh_users();
}

function toggle_user(username: string, uuid: string, activate: boolean): void {
    const userApi = new UserApi(userStore.api_configuration);
    const enableUserRequest = { rows: [uuid], disable: activate } as EnableUserRequest;
    userApi.userEnable(enableUserRequest).then((response) => {
        if (response.status == 200 && response.data.msg == "success") {
            // alert(`${activate ? 'Activated' : 'Deactivated'} user ${username}`);
            refresh_users();
        }
        else {
            alert(`Failed to ${activate ? 'activate' : 'deactivate'} user ${username}`);
        }
    });
}

function refresh_users() {
    getUsers().then((data) => {
        users.value = data;
    });
}

</script>