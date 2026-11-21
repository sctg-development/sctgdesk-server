<!--
=========================================================
* © 2024-2026 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
-->
<template>
    <div class="flex min-h-full flex-1 flex-col justify-center px-6 py-12 lg:px-8">
        <div class="sm:mx-auto sm:w-full sm:max-w-sm">
            <img class="mx-auto h-10 w-auto" :src="$require('@/assets/sctg.svg')" alt="Your Company" />
            <h2 class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">{{ t('login.title', { version: serverVersion }) }}</h2>
        </div>

        <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
            <form class="space-y-6" @submit="handleLogin">
                <div>
                    <label for="email" class="block text-sm font-medium leading-6 text-gray-900">{{ t('login.username') }}</label>
                    <div class="mt-2">
                        <input v-model="name" id="email" name="email" type="text" required
                            class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                    </div>
                </div>

                <div>
                    <div class="flex items-center justify-between">
                        <label for="password" class="block text-sm font-medium leading-6 text-gray-900">{{ t('login.password') }} <span
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
                        class="flex w-full h-12 items-center justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">{{ t('login.signIn') }}</button>
                </div>
            </form>
            <div>
                <p v-if="oauthError" role="alert" class="pt-1.5 text-sm text-red-700">{{ oauthError }}</p>
                <div class="pt-1.5" v-for="oauthprovider in oauthproviders" :key="oauthprovider.name">
                    <button @click="oidcAuth_step1(oauthprovider)" :disabled="oauthPending !== null"
                        class="flex w-full h-12 items-center justify-center rounded-md bg-gray-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-gray-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 disabled:opacity-60">{{ t('login.signInWith', { provider: capitalizeFirstLetter(oauthprovider.name) }) }}</button>
                </div>
            </div>
            <div class="mt-6 flex items-center justify-center gap-2 text-sm text-gray-600">
                <label for="login-language">{{ t('login.language') }}</label>
                <select id="login-language" :value="locale" @change="setLocale(($event.target as HTMLSelectElement).value)"
                    class="rounded-md border-0 py-1 pl-2 pr-8 text-sm text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300">
                    <option v-for="l in LOCALES" :key="l.code" :value="l.code">{{ l.label }}</option>
                </select>
            </div>
        </div>

        <!-- The sign-in of an external provider happens in a real browser window (providers such as GitHub or
             Google forbid being shown in an iframe). This panel keeps the user on the console meanwhile. -->
        <div v-if="oauthPending" class="fixed inset-0 z-20 flex items-center justify-center bg-black/40 p-4" role="dialog"
            aria-modal="true" aria-labelledby="oauth-title">
            <div class="w-full max-w-sm rounded-2xl bg-white p-6 text-center shadow-xl">
                <div class="mx-auto h-10 w-10 animate-spin rounded-full border-4 border-indigo-200 border-t-indigo-600"></div>
                <h3 id="oauth-title" class="mt-4 text-lg font-medium text-gray-900">{{ t('login.oauthWaiting', { provider: oauthPending }) }}</h3>
                <p class="mt-2 text-sm text-gray-600">{{ t('login.oauthHint', { provider: oauthPending }) }}</p>
                <a v-if="oidcUrl" href="#" @click.prevent="openOauthWindow()"
                    class="mt-3 inline-block text-sm font-medium text-indigo-600 hover:text-indigo-500">{{ t('login.oauthReopen') }}</a>
                <div class="mt-5">
                    <button type="button" @click="cancelOauth()"
                        class="rounded-md bg-gray-100 px-4 py-2 text-sm font-medium text-gray-900 hover:bg-gray-300">{{ t('common.cancel') }}</button>
                </div>
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">
import { $require, generateUUIDBase64Encoded } from '@/utilities/viteHelper.js'
import { useUserStore } from '@/stores/sctgDeskStore';
import { useRouter } from 'vue-router';
import { onBeforeUnmount, onMounted, ref } from 'vue';
import { LOCALES, locale, setLocale, t } from '@/i18n';
import { useProfileStore } from '@/stores/profileStore';
import { LoginApi, Configuration } from '@/api';
import { useVersionsStore } from '@/stores/versionsStore';
import { basePath } from '@/utilities/api';

const serverVersion = ref("");

const userStore = useUserStore();
const router = useRouter();

const name = ref("");
const password = ref("");

/** Name of the provider being signed in with (shows the waiting panel), null when idle. */
const oauthPending = ref(null as string | null);
const oauthError = ref("");
/** The authorization URL of the provider, kept so the user can reopen the window if they lost it. */
const oidcUrl = ref("");
let oauthWindow: Window | null = null;
/** Set by `cancelOauth()` / unmount to stop the polling loop of `oidcAuth_step1`. */
let oauthCancelled = false;
/** The sign-in must finish within this time (the user may need to type a password and a 2FA code). */
const OAUTH_TIMEOUT_MS = 180000;

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
function handleLogin(e: SubmitEvent): void {
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
            useProfileStore().load().finally(() => router.push({ name: 'index' }));
        } else {
            console.log(response.data);
            setLoginResult(t('login.wrongCredentials'));
        }
    }).catch((error) => {
        console.log(error);
        setLoginResult(t('login.wrongCredentials'));
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
 * Opens the sign-in window of the provider, centered over the console.
 *
 * The window is a real browser window, not an iframe: GitHub, Google and most providers send
 * `X-Frame-Options: deny` / a frame-ancestors policy on their login page, so an iframe would stay blank.
 *
 * @param url The page to load; omit it to open a blank window (it is navigated once the server gave the URL).
 * @return The window, or null when the browser blocked it.
 */
function openOauthWindow(url?: string): Window | null {
    const width = 520, height = 700;
    const left = Math.max(0, window.screenX + (window.outerWidth - width) / 2);
    const top = Math.max(0, window.screenY + (window.outerHeight - height) / 2);
    const features = `popup=yes,width=${width},height=${height},left=${left},top=${top}`;
    // Reusing the name reuses the window if the user clicks "open it again" while it is still open.
    oauthWindow = window.open(url ?? oidcUrl.value ?? "about:blank", "sctgdesk-oauth", features);
    oauthWindow?.focus();
    return oauthWindow;
}

/** Closes the sign-in window if we still can (a provider can detach it from us, then it is left to the user). */
function closeOauthWindow(): void {
    try {
        oauthWindow?.close();
    } catch {
        // detached window: nothing to do
    }
    oauthWindow = null;
}

/** Stops waiting for the provider and goes back to the form. */
function cancelOauth(): void {
    oauthCancelled = true;
    oauthPending.value = null;
    closeOauthWindow();
}

/** Where the same-tab sign-in keeps what it needs to resume after the round trip to the provider. */
const PENDING_KEY = 'sctgdesk.oidc.pending';

/**
 * Same-tab sign-in, used when the browser blocked the popup: remember the session, then leave for the provider.
 * The server sends the browser back to `/ui/login?oidc_code=...` (see `resumeOauth`). `sessionStorage` is per
 * tab and dies with it; it holds no secret, only the ids the polling needs.
 */
function redirectOauth(providerName: string, code: string, url: string): void {
    try {
        sessionStorage.setItem(PENDING_KEY, JSON.stringify({
            code, provider: providerName, id: userStore.id, uuid: userStore.uuid_base64,
        }));
    } catch {
        oauthError.value = t('login.oauthPopupBlocked');
        oauthPending.value = null;
        return;
    }
    window.location.assign(url);
}

/**
 * Finishes a same-tab sign-in when the provider sent the browser back with `?oidc_code=`.
 * The code must match the one saved before leaving: a link crafted by someone else cannot log the user in
 * with another session.
 */
async function resumeOauth(code: string): Promise<void> {
    let pending: { code: string; provider: string; id: string; uuid: string } | null = null;
    try {
        pending = JSON.parse(sessionStorage.getItem(PENDING_KEY) ?? 'null');
        sessionStorage.removeItem(PENDING_KEY);
    } catch {
        pending = null;
    }
    // drop the code from the address bar whatever happens
    window.history.replaceState({}, '', window.location.pathname);
    if (!pending || pending.code !== code) {
        oauthError.value = t('login.oauthFailed', { provider: pending?.provider ?? '' });
        return;
    }
    userStore.id = pending.id;
    userStore.uuid_base64 = pending.uuid;
    userStore.oidc_code = pending.code;
    oauthPending.value = pending.provider;
    oauthCancelled = false;
    // the provider already answered: a few tries are enough, the session is ready or it never will be
    for (let i = 0; i < 10 && !oauthCancelled; i++) {
        if (await oidcAuth_step2()) {
            oauthPending.value = null;
            await useProfileStore().load();
            router.push({ name: 'index' });
            return;
        }
        await new Promise(resolve => setTimeout(resolve, 1000));
    }
    oauthPending.value = null;
    oauthError.value = t('login.oauthFailed', { provider: pending.provider });
}

/**
 * Performs the first step of the OIDC authentication process, then waits for the second one.
 *
 * Everything happens from this page: the provider's window is opened right here on the click (it must be, or
 * the browser takes it for an unwanted pop-up), a panel tells the user to finish there, and the window closes
 * itself as soon as the server has the session.
 *
 * @param {OauthProvider} provider - The OauthProvider object representing the chosen provider.
 * @return {Promise<void>} - A promise that resolves when the authentication process is complete.
 */
async function oidcAuth_step1(provider: OauthProvider): Promise<void> {
    const providerName = capitalizeFirstLetter(provider.name);
    oauthError.value = "";
    oidcUrl.value = "";
    oauthCancelled = false;
    // Must be synchronous with the click: after an `await`, browsers block the window.
    // When it is blocked anyway, fall back to signing in in this very tab (see `redirectOauth`).
    const popupOpened = openOauthWindow("about:blank") !== null;
    oauthPending.value = providerName;
    userStore.oidc_provider = providerName;
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
        uuid: userStore.uuid_base64,
        // tells the server to send the browser back to the console after the provider (same-tab mode only)
        console: !popupOpened,
    }
    try {
        const response = await loginApi.oidcAuth(oidcAuthRequest as any);
        userStore.oidc_code = response.data.code;
        oidcUrl.value = response.data.url;
        if (!popupOpened) {
            redirectOauth(providerName, response.data.code, response.data.url);
            return;
        }
        if (oauthWindow) {
            oauthWindow.location.href = oidcUrl.value;
        }
        const timeout = Date.now() + OAUTH_TIMEOUT_MS;
        while (!oauthCancelled && Date.now() < timeout) {
            await new Promise(resolve => setTimeout(resolve, 1500));
            if (oauthCancelled) {
                return;
            }
            if (await oidcAuth_step2()) {
                closeOauthWindow();
                oauthPending.value = null;
                await useProfileStore().load();
                router.push({ name: 'index' });
                return;
            }
        }
        if (!oauthCancelled) {
            oauthError.value = t('login.oauthTimeout');
        }
    } catch (error) {
        console.log(error);
        oauthError.value = t('login.oauthFailed', { provider: providerName });
    }
    cancelOauth();
}

/**
 * Performs the second step of the OIDC authentication process.
 *
 * @return {Promise<boolean>} A promise that resolves to true if the authentication is successful,
 *                           or false otherwise.
 */
function oidcAuth_step2(): Promise<boolean> {
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
function capitalizeFirstLetter(string: string): string {
    return string.charAt(0).toUpperCase() + string.slice(1);
}

onBeforeUnmount(() => {
    // leaving the page (e.g. after the sign-in) must stop the polling loop and close the provider's window
    oauthCancelled = true;
    closeOauthWindow();
});

onMounted(() => {
    const resumeCode = new URLSearchParams(window.location.search).get('oidc_code');
    if (resumeCode) {
        resumeOauth(resumeCode);
    }
    useVersionsStore().fetchVersions().then(() => {
        serverVersion.value = useVersionsStore().serverVersion;
    })
    if (!resumeCode) {
        // a resumed sign-in keeps the ids it started with: the server matches the session on them
        userStore.uuid_base64 = generateUUIDBase64Encoded();
        userStore.id = Math.random().toString(36).substring(2, 15);
    }
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