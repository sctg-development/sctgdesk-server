<!--
=========================================================
* © 2024-2026 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
-->
<template>
  <div class="min-h-full">
    <Disclosure as="nav" class="bg-gray-800" v-slot="{ open }">
      <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div class="flex h-16 items-center justify-between">
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <img class="h-8 w-8" :src="$require('@/assets/sctg.svg')" alt="Your Company" />
            </div>
            <div class="hidden md:block">
              <div class="ml-10 flex items-baseline space-x-4">
                <a v-for="item in navigation" :key="item.key" :href="item.href" @click="makePageCurrent(item.key)"
                  :class="[item.current ? 'bg-gray-900 text-white' : 'text-gray-300 hover:bg-gray-700 hover:text-white', 'rounded-md px-3 py-2 text-sm font-medium']"
                  :aria-current="item.current ? 'page' : undefined">{{ t('nav.' + item.key) }}</a>
              </div>
            </div>
          </div>
          <div class="hidden md:block">
            <div class="ml-4 flex items-center md:ml-6">
              <button type="button"
                class="relative rounded-full bg-gray-800 p-1 text-gray-400 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800">
                <span class="absolute -inset-1.5" />
                <span class="sr-only">{{ t('nav.notifications') }}</span>
                <BellIcon class="h-6 w-6" aria-hidden="true" />
              </button>

              <!-- Profile dropdown -->
              <Menu as="div" class="relative ml-3">
                <div>
                  <MenuButton
                    class="relative flex max-w-xs items-center rounded-full bg-gray-800 text-sm focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800">
                    <span class="absolute -inset-1.5" />
                    <span class="sr-only">{{ t('nav.openUserMenu') }}</span>
                    <img class="h-8 w-8 rounded-full object-cover" :src="user.imageUrl" alt="" />
                  </MenuButton>
                </div>
                <transition enter-active-class="transition ease-out duration-100"
                  enter-from-class="transform opacity-0 scale-95" enter-to-class="transform opacity-100 scale-100"
                  leave-active-class="transition ease-in duration-75" leave-from-class="transform opacity-100 scale-100"
                  leave-to-class="transform opacity-0 scale-95">
                  <MenuItems
                    class="absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none">
                    <MenuItem v-for="item in userNavigation" :key="item.key" v-slot="{ active }">
                    <a :href="item.href" @click.prevent="item.action"
                      :class="[active ? 'bg-gray-100' : '', 'block px-4 py-2 text-sm text-gray-700']">{{ item.label }}</a>
                    </MenuItem>
                  </MenuItems>
                </transition>
              </Menu>
            </div>
          </div>
          <div class="-mr-2 flex md:hidden">
            <!-- Mobile menu button -->
            <DisclosureButton
              class="relative inline-flex items-center justify-center rounded-md bg-gray-800 p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800">
              <span class="absolute -inset-0.5" />
              <span class="sr-only">{{ t('nav.openMenu') }}</span>
              <Bars3Icon v-if="!open" class="block h-6 w-6" aria-hidden="true" />
              <XMarkIcon v-else class="block h-6 w-6" aria-hidden="true" />
            </DisclosureButton>
          </div>
        </div>
      </div>

      <DisclosurePanel class="md:hidden">
        <div class="space-y-1 px-2 pb-3 pt-2 sm:px-3">
          <DisclosureButton v-for="item in navigation" :key="item.key" as="a" :href="item.href"
            @click="makePageCurrent(item.key)"
            :class="[item.current ? 'bg-gray-900 text-white' : 'text-gray-300 hover:bg-gray-700 hover:text-white', 'block rounded-md px-3 py-2 text-base font-medium']"
            :aria-current="item.current ? 'page' : undefined">{{ t('nav.' + item.key) }}</DisclosureButton>
        </div>
        <div class="border-t border-gray-700 pb-3 pt-4">
          <div class="flex items-center px-5">
            <div class="flex-shrink-0">
              <img class="h-10 w-10 rounded-full object-cover" :src="user.imageUrl" alt="" />
            </div>
            <div class="ml-3">
              <div class="text-base font-medium leading-none text-white">{{ user.name }}</div>
              <div class="text-sm font-medium leading-none text-gray-400">{{ user.email }}</div>
            </div>
            <button type="button"
              class="relative ml-auto flex-shrink-0 rounded-full bg-gray-800 p-1 text-gray-400 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800">
              <span class="absolute -inset-1.5" />
              <span class="sr-only">{{ t('nav.notifications') }}</span>
              <BellIcon class="h-6 w-6" aria-hidden="true" />
            </button>
          </div>
          <div class="mt-3 space-y-1 px-2">
            <DisclosureButton v-for="item in userNavigation" :key="item.key" as="a" :href="item.href"
              @click.prevent="item.action"
              class="block rounded-md px-3 py-2 text-base font-medium text-gray-400 hover:bg-gray-700 hover:text-white">
              {{ item.label }}</DisclosureButton>
          </div>
        </div>
      </DisclosurePanel>
    </Disclosure>

    <header class="bg-white shadow">
      <div class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
        <h1 class="text-3xl font-bold tracking-tight text-gray-900">{{ pageTitle }}</h1>
      </div>
    </header>
    <main>
      <div v-if="isCurrentPage('dashboard')" class="mx-auto max-w-7xl py-6 sm:px-6 lg:px-8">
        <DashboardCard />
      </div>
      <div v-if="isCurrentPage('devices')" class="mx-auto max-w-7xl py-6 sm:px-6 lg:px-8">
        <DevicesCard />
      </div>
      <div v-if="isCurrentPage('users')" class="mx-auto max-w-7xl py-6 sm:px-6 lg:px-8">
        <UsersCard />
      </div>
      <div v-if="isCurrentPage('groups')" class="mx-auto max-w-7xl py-6 sm:px-6 lg:px-8">
        <GroupsCard />
      </div>
      <div v-if="isCurrentPage('addressBooks')" class="mx-auto max-w-7xl py-6 sm:px-6 lg:px-8">
        <AccessibleAddressBooks />
      </div>
      <div v-if="isCurrentPage('connections')" class="mx-auto max-w-7xl py-6 sm:px-6 lg:px-8">
        <ConnectionsCard />
      </div>
      <div v-if="isCurrentPage('settings') && isAdmin" class="mx-auto max-w-7xl py-6 sm:px-6 lg:px-8">
        <SettingsCard />
      </div>
      <div v-if="isCurrentPage('userSettings')" class="mx-auto max-w-3xl py-6 sm:px-6 lg:px-8">
        <UserSettings />
      </div>
    </main>
    <MainFooter />
  </div>
</template>

<script setup lang="ts">
import { $require } from '@/utilities/viteHelper.js'
import { Disclosure, DisclosureButton, DisclosurePanel, Menu, MenuButton, MenuItem, MenuItems } from '@headlessui/vue'
import { Bars3Icon, BellIcon, XMarkIcon } from '@heroicons/vue/24/outline'
import { useRouter } from 'vue-router';
import { useUserStore } from '@/stores/sctgDeskStore';
import { LoginApi } from '@/api';
import DevicesCard from '@/components/DevicesCard.vue';
import UsersCard from '@/components/UsersCard.vue';
import DashboardCard from '@/components/DashboardCard.vue';
import MainFooter from '@/footers/mainfooter.vue';
import { computed, ref } from 'vue';
import { t } from '@/i18n';
import { useProfileStore } from '@/stores/profileStore';
import SettingsCard from '@/components/SettingsCard.vue';
import UserSettings from '@/components/UserSettings.vue';
import GroupsCard from '@/components/GroupsCard.vue';
import AccessibleAddressBooks from '@/components/AccessibleAddressBooks.vue';
import ConnectionsCard from '@/components/ConnectionsCard.vue';
const userStore = useUserStore();
const router = useRouter();
const profileStore = useProfileStore();

/** The user shown in the header; computed so that a new avatar or name shows at once. */
const user = computed(() => ({
  name: userStore.user?.name || t('common.unknown'),
  email: userStore.user?.email || '',
  imageUrl: profileStore.avatarUrl,
}))

const isAdmin = computed(() => !!userStore.user?.admin)

/**
 * The pages of the main navigation. `key` is both the page id and the suffix of the `nav.*` message.
 * The settings page changes the server configuration, so it only exists for administrators.
 */
const navigation = ref([
  { key: 'dashboard', href: '#', current: true, adminOnly: false },
  { key: 'devices', href: '#', current: false, adminOnly: false },
  { key: 'users', href: '#', current: false, adminOnly: false },
  { key: 'groups', href: '#', current: false, adminOnly: false },
  { key: 'addressBooks', href: '#', current: false, adminOnly: false },
  { key: 'connections', href: '#', current: false, adminOnly: false },
  { key: 'settings', href: '#', current: false, adminOnly: true },
].filter((item) => !item.adminOnly || userStore.user?.admin))

/** Entries of the avatar menu. `userSettings` is a page too, but it is not part of the main navigation. */
const userNavigation = computed(() => [
  { key: 'who', label: `${userStore.user?.name} ${userStore.user?.email ?? ''}`.trim(), href: '#', action: nop },
  { key: 'userSettings', label: t('userMenu.settings'), href: '#', action: () => makePageCurrent('userSettings') },
  { key: 'signOut', label: t('userMenu.signOut'), href: '#', action: logout },
])

/** The page being shown; `userSettings` is none of the navigation items, so no tab is highlighted for it. */
const pageTitle = computed(() => {
  const current = navigation.value.find((item) => item.current)
  return current ? t('nav.' + current.key) : t('userMenu.settings')
})

/**
 * Checks if the given page is the current page.
 *
 * @param {string} page - The id of the page to check.
 * @return {boolean} True if the page is the current page, false otherwise.
 */
function isCurrentPage(page: string): boolean {
  return currentPage.value === page;
}

/** The id of the page being shown. */
const currentPage = ref('dashboard')

/**
 * Shows another page and highlights its entry in the navigation (none for pages outside of it).
 *
 * @param {string} page - The id of the page to show.
 * @return {void} This function does not return anything.
 */
function makePageCurrent(page: string): void {
  currentPage.value = page;
  navigation.value.forEach((item) => {
    item.current = item.key === page;
  });
}
/**
 * Logs out the user by calling the logout API endpoint.
 *
 * @return {void} This function does not return anything.
 */
function logout(): void {
  const loginApi = new LoginApi(userStore.api_configuration);
  loginApi.logout({ id: userStore.user.name, uuid: "" }).then((response) => {
    if (response.status == 200) {
      userStore.user = null;
      userStore.api_configuration = null;
      profileStore.reset();
      router.push({ name: 'login' });
    }
  }).catch((error) => {
    console.error(error);
    alert(t('userMenu.unableToLogout'));
  });
}



/**
 * A no-operation function that does nothing.
 *
 * @return {void} This function does not return anything.
 */
function nop(): void {
  // Do nothing
}

</script>