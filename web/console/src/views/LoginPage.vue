<!--
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
-->
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
                        <label for="password" class="block text-sm font-medium leading-6 text-gray-900">Password <span
                                id="loginResult" class="text-red-700"></span></label>
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
            <div>
                <a ref="oidc_link" href="#" class="text-sm font-medium text-indigo-600 hover:text-indigo-500"></a>
                <div class="pt-1.5" v-for="oauthprovider in oauthproviders">
                    <button @click="oidcAuth_step1(oauthprovider)"
                        class="flex w-full h-12 items-center justify-center rounded-md bg-gray-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-gray-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">Sign
                        in with {{ capitalizeFirstLetter(oauthprovider.name) }}</button>
                </div>
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">
import { $require, generateUUIDBase64Encoded } from '@/utilities/viteHelper.js'
import { useUserStore } from '@/stores/sctgDeskStore';
import { useRouter } from 'vue-router';
import { onMounted, ref } from 'vue';
import { LoginApi, Configuration } from '@/api';
const userStore = useUserStore();
const router = useRouter();

const name = ref("");
const password = ref("");
const basePath = window.location.origin == "http://localhost:5173" ? "http://127.0.0.1:21114" : window.location.origin;
const oidc_link = ref(null as HTMLAnchorElement | null);

type OauthProvider = {
    name: string;
    rustdesk_name: string;
};

const oauthproviders = ref([] as OauthProvider[]);

/**
 * Handles the login form submission event.
 *
 * @param {SubmitEvent} e - The submit event object.
 * @return {void} This function does not return a value.
 */
function handleLogin(e: SubmitEvent) {
    e.preventDefault();
    const configuration = new Configuration({
        // Workaround for development environment
        basePath: basePath,
        username: name.value,
        password: password.value
    });
    const loginApi = new LoginApi(configuration);
    loginApi.login({ username: name.value, password: password.value, id: "", uuid: "" }).then((response) => {
        if (response.status == 200) {
            const data = response.data;
            userStore.user = data.user;
            userStore.api_configuration = configuration;
            userStore.api_configuration.accessToken = data.access_token as any;
            router.push({ name: 'index' });
        } else {
            console.log(response.data);
            document.getElementById("loginResult").innerText = "Wrong username or password !";
        }
    }).catch((error) => {
        console.log(error);
        setLoginResult("Wrong username or password !");
    });

}

/**
 * Sets the inner text of the element with the ID "loginResult" to the specified message.
 *
 * @param {string} message - The message to be displayed.
 * @return {void} This function does not return a value.
 */
function setLoginResult(message: string): void {
    document.getElementById("loginResult").innerText = message;
}

/**
 * Performs the first step of the OIDC authentication process.
 *
 * @param {OauthProvider} provider - The OauthProvider object representing the chosen provider.
 * @return {Promise<void>} - A promise that resolves when the authentication process is complete.
 */
function oidcAuth_step1(provider: OauthProvider) {
    const configuration = new Configuration({
        basePath: basePath,
        username: name.value,
        password: password.value
    });
    const loginApi = new LoginApi(configuration);
    const oidcAuthRequest = {
        deviceInfo: {
            name: navigator.appName,
            os: navigator.platform,
            type: "oidc",
        },
        id: userStore.id,
        op: provider.rustdesk_name,
        uuid: userStore.uuid_base64
    }
    userStore.oidc_provider = capitalizeFirstLetter(provider.name);
    loginApi.oidcAuth(oidcAuthRequest).then(async (response) => {
        console.log(response);
        userStore.oidc_code = response.data.code;
        
        oidc_link.value.href = response.data.url;
        oidc_link.value.innerText = `Please authenticate with ${userStore.oidc_provider}...`;
        oidc_link.value.target = "_blank";

        const timeout = new Date().getTime() + 30000; // 30s en millisecondes
        while (new Date().getTime() < timeout) {
            const result = await oidcAuth_step2();
            if (result) {
                router.push({ name: 'index' });
                return;
            }
            await new Promise(resolve => setTimeout(resolve, 2000));
        }
    }).catch((error) => {
        console.log(error);
    });
}

/**
 * Performs the second step of the OIDC authentication process.
 *
 * @return {Promise<boolean>} A promise that resolves to true if the authentication is successful,
 *                           or false otherwise.
 */
function oidcAuth_step2():Promise<boolean> {
    const configuration = new Configuration({
        basePath: basePath,
        username: name.value,
        password: password.value
    });
    const loginApi = new LoginApi(configuration);
    return new Promise((resolve, reject) => {
        loginApi.oidcState(userStore.oidc_code, userStore.id, userStore.uuid_base64).then((response) => {
            console.log(response);
            if (response.data.access_token !== undefined) {
                userStore.user = {
                    name: response.data.user.name,
                    admin: response.data.user.is_admin,
                    email: response.data.user.email,
                };
                userStore.api_configuration = configuration;
                userStore.api_configuration.accessToken = response.data.access_token;
                resolve(true);
            } else {
                resolve(false);
            }
        }).catch((error) => {
            console.log(error);
            resolve(false);
        });
    });
}

/**
 * Capitalizes the first letter of a given string.
 *
 * @param {string} string - The string to capitalize.
 * @return {string} The capitalized string.
 */
function capitalizeFirstLetter(string: string): string{
    return string.charAt(0).toUpperCase() + string.slice(1);
}

onMounted(() => {
    userStore.uuid_base64 = generateUUIDBase64Encoded();
    userStore.id = Math.random().toString(36).substring(2, 15);
    const configuration = new Configuration({
        basePath: basePath,
        username: name.value,
        password: password.value
    });
    const loginApi = new LoginApi(configuration);
    loginApi.loginOptions().then((_providers) => {
        for (const _provider of _providers.data) {
            oauthproviders.value.push({
                name: _provider.split("/")[1],
                rustdesk_name: _provider.split("/")[1]
            });
        }
    }).catch((error) => {
        console.log(error);
    });
})
</script>