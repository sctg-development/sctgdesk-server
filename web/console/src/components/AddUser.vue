<!--
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
-->
<template>
    <Modal @modalOk="addUser()" @modalCancel="closeModal()" :okLabel="t('common.add')" :title="t('admin.users.add')">
        <div>
            <label for="name" class="block text-sm font-medium leading-6 text-gray-900">{{ t('admin.fields.username') }}</label>
            <div class="mt-2">
                <input v-model="name" id="name" name="name" type="text" required
                    class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
            </div>
        </div>
        <div>
            <label for="password" class="block text-sm font-medium leading-6 text-gray-900">{{ t('admin.fields.password') }}</label>
            <div class="mt-2">
                <input v-model="password" id="passwors" name="password" type="password" required
                    class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
            </div>
        </div>
        <div>
            <label for="confirm-password" class="block text-sm font-medium leading-6 text-gray-900">{{ t('admin.fields.confirm') }}</label>
            <div class="mt-2">
                <input v-model="confirm_password" id="confirm_password" name="confirm_password" type="password" required
                    class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
            </div>
        </div>
        <div>
            <label for="email" class="block text-sm font-medium leading-6 text-gray-900">{{ t('admin.fields.email') }}</label>
            <div class="mt-2">
                <input v-model="email" id="email" name="email" type="text" required
                    class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
            </div>
        </div>
        <div>
            <label for="is_admin" class="block text-sm font-medium leading-6 text-gray-900">{{ t('admin.fields.administrator') }}</label>
            <div class="mt-2">
                <input v-model="is_admin" id="is_admin" name="is_admon" type="checkbox" required
                    class="block rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
            </div>
        </div>
        <div>
            <label for="grp" class="block text-sm font-medium leading-6 text-gray-900">{{ t('admin.fields.group') }}</label>
            <div class="mt-2">
                <select v-model="grp" id="grp" name="grp" required
                    class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6">
                    <option v-for="group in groups" :key="group.guid" :value="group.name">
                        {{ group.name }}
                    </option>
                </select>
            </div>
        </div>
    </Modal>
</template>
<script setup lang="ts">
import { t } from '@/i18n';
import Modal from '@/components/Modal.vue';
import { onMounted } from 'vue';
import { ref } from 'vue';
import { AddUserRequest, UserApi, GroupApi, Group } from '@/api';
import { useUserStore } from '@/stores/sctgDeskStore';

const userStore = useUserStore();
const name = ref("");
const password = ref("");
const confirm_password = ref("");
const email = ref("");
const is_admin = ref(false);
const grp = ref("");
const groups = ref([] as Group[]);
const groupApi = new GroupApi(userStore.api_configuration);

const emit = defineEmits(['add_user_close', 'user_added'])

onMounted(() => {
    groupApi.groups(1, 4294967295).then((response) => {
        groups.value = response.data.data;
        grp.value = groups.value[0].name;
    });
});

/**
 * Adds a new user to the system.
 *
 * @return {void} This function does not return anything. It displays an alert if the user does not fill in all fields,
 * or if the password and confirm password do not match. If the user is successfully added, it logs a message to the console
 * and closes the modal. Otherwise, it displays an alert indicating that the user failed to be added.
 */
function addUser(): void {
    if (name.value == "" || password.value == "" || confirm_password.value == "" || email.value == "") {
        alert(t('admin.users.fillAll'));
        return;
    }
    if (password.value != confirm_password.value) {
        alert(t('admin.users.mismatch'));
        return;
    }
    // Add user
    const user_request: AddUserRequest = {
        "name": name.value,
        "password": password.value,
        "confirm-password": confirm_password.value,
        "email": email.value,
        "is_admin": is_admin.value,
        "group_name": grp.value
    }
    const userApi = new UserApi(userStore.api_configuration);
    userApi.userAdd(user_request).then((response) => {
        if (response.status == 200 && response.data.msg == "success") {
            console.log("User added successfully");
            closeModal();
        }
        else {
            alert(t('admin.users.addFailed'));
        }
    });
}

/**
 * Closes the modal by emitting the 'add_user_close' event.
 *
 * @return {void} This function does not return anything.
 */
function closeModal(): void {
    emit('add_user_close')
}
</script>