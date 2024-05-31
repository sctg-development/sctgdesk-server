<template>
    <Modal @modalOk="updateUser()" @modalCancel="closeModal()">
        <div>
            <label for="name" class="block text-sm font-medium leading-6 text-gray-900">Username</label>
            <div class="mt-2">
                <input v-model="name" id="name" name="name" type="text" required readonly
                    class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
            </div>
        </div>
        <div>
            <label for="password" class="block text-sm font-medium leading-6 text-gray-900">Password</label>
            <div class="mt-2">
                <input v-model="password" id="passwors" name="password" type="password" required
                    class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
            </div>
        </div>
        <div>
            <label for="confirm-password" class="block text-sm font-medium leading-6 text-gray-900">Confirm</label>
            <div class="mt-2">
                <input v-model="confirm_password" id="confirm_password" name="confirm_password" type="password" required
                    class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
            </div>
        </div>
        <div>
            <label for="email" class="block text-sm font-medium leading-6 text-gray-900">Email</label>
            <div class="mt-2">
                <input v-model="email" id="email" name="email" type="text" required
                    class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
            </div>
        </div>
        <div>
            <label for="note" class="block text-sm font-medium leading-6 text-gray-900">Note</label>
            <div class="mt-2">
                <input v-model="note" id="note" name="note" type="text"
                    class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
            </div>
        </div>
        <div>
            <label for="is_admin" class="block text-sm font-medium leading-6 text-gray-900">Administrator</label>
            <div class="mt-2">
                <input v-model="is_admin" id="is_admin" name="is_admon" type="checkbox" required
                    class="block rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
            </div>
        </div>
        <div>
            <label for="grp" class="block text-sm font-medium leading-6 text-gray-900">Group</label>
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
import {
    TransitionRoot,
    TransitionChild,
    Dialog,
    DialogPanel,
    DialogTitle,
} from '@headlessui/vue'
import { onMounted } from 'vue';
import { ref } from 'vue';
import { UpdateUserRequest, UserApi, GroupApi, Group, UserListResponse } from '@/api';
import Modal from '@/components/Modal.vue';
import { useUserStore } from '@/stores/sctgDeskStore';

const userStore = useUserStore();
const name = ref("");
const password = ref("");
const confirm_password = ref("");
const email = ref("");
const note = ref("");
const is_admin = ref(false);
const grp = ref("");
const groups = ref([] as Group[]);
const groupApi = new GroupApi(userStore.api_configuration);

/**
 * Props
 * @prop {string} username: username to edit
 * @prop {string} uuid: uuid of the user to edit
 *
 * Emits
 * @event update_user_close: asks parent to close the modal
 * @event user_updated: emits when user is updated
 */
const emit = defineEmits(['update_user_close', 'user_updated'])
const props = withDefaults(defineProps<{
    username: string;
    uuid: string;
}>(),
    {
        username: '',
        uuid: ''
    })

onMounted(() => {
    groupApi.groups(1, 4294967295).then((response) => {
        groups.value = response.data.data;
        grp.value = groups.value[0].name;
    });
    getUser(props.username).then((data) => {
        console.log(data);
        if (data.length > 0) {
            name.value = data[0].name;
            email.value = data[0].email;
            is_admin.value = data[0].is_admin;
            note.value = data[0].note;
            grp.value = data[0].group_name;
        }
    });
});
function updateUser() {
    if (password.value != confirm_password.value) {
        alert("Password and confirm password do not match");
        return;
    }
    // Add user
    const user_request: UpdateUserRequest = {
        "uuid": props.uuid,
        "name": name.value,
        "password": password.value,
        "confirm-password": confirm_password.value,
        "email": email.value,
        "is_admin": is_admin.value,
        "note": note.value,
        "group_name": grp.value
    }
    const userApi = new UserApi(userStore.api_configuration);
    userApi.userUpdate(user_request).then((response) => {
        if (response.status == 200 && response.data.msg == "success") {
            alert("User update successfully");
            closeModal();
        }
        else {
            alert("Failed to update user");
        }
    });
}

/**
 * Retrieves the list of users from the API.
 *
 * @return {Promise<UserListResponse[]>} A promise that resolves to the list of users.
 */
function getUser(username: string): Promise<UserListResponse[]> {
    console.log(`getUser: ${username}`);
    const userApi = new UserApi(userStore.api_configuration);
    return new Promise<UserListResponse[]>((resolve, reject) => {
        userApi.users(1, 4294967295, undefined, username).then((response) => {
            if (response.status == 200 && response.data.msg == "success") {
                resolve(response.data.data);
            }
            else {
                resolve([] as UserListResponse[]);
            }
        }).catch((error) => {
            console.error(error);
            resolve([] as UserListResponse[]);
        });
    });
}
function closeModal() {
    emit('update_user_close')
}
</script>