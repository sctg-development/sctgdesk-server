/*!
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
*/
import { createApp } from 'vue'
import App from '@/App.vue'
import { createWebHistory, createRouter, RouteRecordRaw, NavigationGuardNext } from "vue-router"
import { createPinia } from 'pinia'
import { addJsonLD } from "./config/StructuredLDJson"
import { useUserStore } from '@/stores/sctgDeskStore';
import '@/index.scss'
import { RouteLocationNormalized } from 'vue-router'

const baseUrl = import.meta.url;
// const useImage = (url: string) => {
//   return new URL(
//     `/src/${url.substring(0, 1) === "@" ? url.substring(2) : url}`,
//     baseUrl
//   ).href;
// };

// declare module "@vue/runtime-core" {
//   interface ComponentCustomProperties {
//     $require: typeof useImage;
//   }
// }

const routes = [
    {
        path: "/ui/index",
        component: () => import("@/views/IndexPage.vue"),
        name: 'index',
    },
    {
        path: "/ui/login",
        component: () => import("@/views/LoginPage.vue"),
        name: 'login',
    },
    {
        path: "/:pathMatch(.*)*",
        name: 'default',
        redirect: "/ui/login"
    },
] as RouteRecordRaw[]

const router = createRouter({
    scrollBehavior(to) {
        if (to.hash) {
            return {
                el: to.hash,
            }
        }
    },
    history: createWebHistory(),
    routes,
});

router.beforeEach((to: RouteLocationNormalized, from: RouteLocationNormalized, next: NavigationGuardNext) => {
    const store = useUserStore();
    if (to.name !== 'login' && !store.user && !store.accessToken) {
        next({ name: 'login' });
    } else {
        next();
    }
});

const pinia = createPinia()

createApp(App).use(router)
    .use(pinia)
    .mount('#app')
addJsonLD();
