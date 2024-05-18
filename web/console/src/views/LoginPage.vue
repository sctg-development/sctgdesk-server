<template>
    <div class="flex min-h-full flex-1 flex-col justify-center px-6 py-12 lg:px-8">
        <div class="sm:mx-auto sm:w-full sm:max-w-sm">
            <img class="mx-auto h-10 w-auto" :src="$require('@/assets/sctg.svg')" alt="Your Company" />
            <h2 class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">SCTGDesk server</h2>
        </div>

        <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
            <form class="space-y-6" @submit="handleLogin">
                <div>
                    <label for="email" class="block text-sm font-medium leading-6 text-gray-900">Username</label>
                    <div class="mt-2">
                        <input v-model="name" id="email" name="email" type="text" required
                            class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                    </div>
                </div>

                <div>
                    <div class="flex items-center justify-between">
                        <label for="password" class="block text-sm font-medium leading-6 text-gray-900">Password</label>
                    </div>
                    <div class="mt-2">
                        <input v-model="password" id="password" name="password" type="password"
                            autocomplete="current-password" required
                            class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                    </div>
                </div>

                <div>
                    <button type="submit"
                        class="flex w-full h-12 items-center justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">Sign
                        in</button>
                </div>
            </form>
        </div>
    </div>
</template>
<script setup lang="ts">
import { $require } from '@/utilities/viteHelper.js'
import { useUserStore } from '@/stores/sctgDeskStore';
import { useRouter } from 'vue-router';
import { ref } from 'vue';
import { LoginApi, Configuration } from '@/api';
const userStore = useUserStore();
const router = useRouter();

const name = ref("");
const password = ref("");

function handleLogin(e: SubmitEvent) {
    e.preventDefault();
    const configuration = new Configuration({
        basePath: window.location.origin,
    });
    const loginApi = new LoginApi(configuration);
    loginApi.login({ username: name.value, password: password.value, id: "", uuid: "" }).then((response) => {
        if (response.status == 200) {
            const data = response.data;
            userStore.user = data.user;
            userStore.accessToken = data.access_token;
            router.push({ name: 'index' });
        } else {
            console.log(response.data);
        }
    }).catch((error) => {
        console.log(error);
    });

}
</script>