<!--
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
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
                <a v-for="item in navigation" :key="item.name" :href="item.href" @click="makePageCurrent(item.name)"
                  :class="[item.current ? 'bg-gray-900 text-white' : 'text-gray-300 hover:bg-gray-700 hover:text-white', 'rounded-md px-3 py-2 text-sm font-medium']"
                  :aria-current="item.current ? 'page' : undefined">{{ item.name }}</a>
              </div>
            </div>
          </div>
          <div class="hidden md:block">
            <div class="ml-4 flex items-center md:ml-6">
              <button type="button"
                class="relative rounded-full bg-gray-800 p-1 text-gray-400 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800">
                <span class="absolute -inset-1.5" />
                <span class="sr-only">View notifications</span>
                <BellIcon class="h-6 w-6" aria-hidden="true" />
              </button>

              <!-- Profile dropdown -->
              <Menu as="div" class="relative ml-3">
                <div>
                  <MenuButton
                    class="relative flex max-w-xs items-center rounded-full bg-gray-800 text-sm focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800">
                    <span class="absolute -inset-1.5" />
                    <span class="sr-only">Open user menu</span>
                    <img class="h-8 w-8 rounded-full" :src="user.imageUrl" alt="" />
                  </MenuButton>
                </div>
                <transition enter-active-class="transition ease-out duration-100"
                  enter-from-class="transform opacity-0 scale-95" enter-to-class="transform opacity-100 scale-100"
                  leave-active-class="transition ease-in duration-75" leave-from-class="transform opacity-100 scale-100"
                  leave-to-class="transform opacity-0 scale-95">
                  <MenuItems
                    class="absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none">
                    <MenuItem v-for="item in userNavigation" :key="item.name" v-slot="{ active }">
                    <a :href="item.href" @click="item.action"
                      :class="[active ? 'bg-gray-100' : '', 'block px-4 py-2 text-sm text-gray-700']">{{ item.name
                      }}</a>
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
              <span class="sr-only">Open main menu</span>
              <Bars3Icon v-if="!open" class="block h-6 w-6" aria-hidden="true" />
              <XMarkIcon v-else class="block h-6 w-6" aria-hidden="true" />
            </DisclosureButton>
          </div>
        </div>
      </div>

      <DisclosurePanel class="md:hidden">
        <div class="space-y-1 px-2 pb-3 pt-2 sm:px-3">
          <DisclosureButton v-for="item in navigation" :key="item.name" as="a" :href="item.href"
            @click="makePageCurrent(item.name)"
            :class="[item.current ? 'bg-gray-900 text-white' : 'text-gray-300 hover:bg-gray-700 hover:text-white', 'block rounded-md px-3 py-2 text-base font-medium']"
            :aria-current="item.current ? 'page' : undefined">{{ item.name }}</DisclosureButton>
        </div>
        <div class="border-t border-gray-700 pb-3 pt-4">
          <div class="flex items-center px-5">
            <div class="flex-shrink-0">
              <img class="h-10 w-10 rounded-full" :src="user.imageUrl" alt="" />
            </div>
            <div class="ml-3">
              <div class="text-base font-medium leading-none text-white">{{ user.name }}</div>
              <div class="text-sm font-medium leading-none text-gray-400">{{ user.email }}</div>
            </div>
            <button type="button"
              class="relative ml-auto flex-shrink-0 rounded-full bg-gray-800 p-1 text-gray-400 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800">
              <span class="absolute -inset-1.5" />
              <span class="sr-only">View notifications</span>
              <BellIcon class="h-6 w-6" aria-hidden="true" />
            </button>
          </div>
          <div class="mt-3 space-y-1 px-2">
            <DisclosureButton v-for="item in userNavigation" :key="item.name" as="a" :href="item.href"
              @click="item.action"
              class="block rounded-md px-3 py-2 text-base font-medium text-gray-400 hover:bg-gray-700 hover:text-white">
              {{ item.name }}</DisclosureButton>
          </div>
        </div>
      </DisclosurePanel>
    </Disclosure>

    <header class="bg-white shadow">
      <div class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
        <h1 class="text-3xl font-bold tracking-tight text-gray-900">{{ getCurrentPage() }}</h1>
      </div>
    </header>
    <main>
      <div v-if="isCurrentPage('Dashboard')" class="mx-auto max-w-7xl py-6 sm:px-6 lg:px-8">
        Dashboard content
      </div>
      <div v-if="isCurrentPage('Devices')" class="mx-auto max-w-7xl py-6 sm:px-6 lg:px-8">
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
                          Id
                        </th>
                        <th class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                          Status
                        </th>
                        <th class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                          Username
                        </th>
                        <th class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                          Os
                        </th>
                        <th class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                          Ip
                        </th>
                        <th
                          class="w-1/6 min-w-[160px] border-r border-transparent py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                          Cpu
                        </th>
                        <th></th>
                      </tr>
                    </thead>
                    <tbody>
                      <tr v-for="peer in peers" :key="peer.guid">
                        <td
                          class="text-dark border-b border-l border-[#E8E8E8] bg-[#F3F6FF] dark:bg-dark-3 dark:border-dark dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                          {{ peer.id }}
                        </td>
                        <td
                          class="text-dark border-b border-[#E8E8E8] bg-white dark:border-dark dark:bg-dark-2 dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                          {{ peer.status }}
                        </td>
                        <td
                          class="text-dark border-b border-[#E8E8E8] bg-[#F3F6FF] dark:bg-dark-3 dark:border-dark dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                          {{ peer.info.username }}
                        </td>
                        <td
                          class="text-dark border-b border-[#E8E8E8] bg-white dark:border-dark dark:bg-dark-2 dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                          {{ peer.info.os }}
                        </td>
                        <td
                          class="text-dark border-b border-[#E8E8E8] bg-[#F3F6FF] dark:bg-dark-3 dark:border-dark dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                          {{ peer.info.ip }}
                        </td>
                        <td
                          class="text-dark border-b border-[#E8E8E8] bg-white dark:bg-dark-3 dark:border-dark dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                          {{ peer.info.cpu }}
                        </td>
                        <td
                          class="text-dark border-b border-r border-[#E8E8E8] bg-[#F3F6FF] dark:border-dark dark:bg-dark-2 dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                          <a href="javascript:void(0)"
                            class="inline-block px-6 py-2.5 border rounded-md border-primary text-primary hover:bg-primary hover:text-white font-medium">
                            Delete
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
      </div>
      <div v-if="isCurrentPage('Users')" class="mx-auto max-w-7xl py-6 sm:px-6 lg:px-8">
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
                        <th class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                          Name
                        </th>
                        <th class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                          Email
                        </th>
                        <th class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                          Status
                        </th>
                        <th class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                          Admin
                        </th>
                        <th
                          class="w-1/6 min-w-[160px] border-r border-transparent py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                          Note
                        </th>
                        <th></th>
                      </tr>
                    </thead>
                    <tbody>
                      <tr v-for="user in users" :key="user.guid">
                        <td
                          class="text-dark border-b border-l border-[#E8E8E8] bg-[#F3F6FF] dark:bg-dark-3 dark:border-dark dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                          {{ user.guid }}
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
                          {{ user.status }}
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
                          <a href="javascript:void(0)"
                            class="inline-block px-6 py-2.5 border rounded-md border-primary text-primary hover:bg-primary hover:text-white font-medium">
                            Edit
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
      </div>
      <div v-if="isCurrentPage('Groups')" class="mx-auto max-w-7xl py-6 sm:px-6 lg:px-8">
        Groups content
      </div>
      <div v-if="isCurrentPage('Reports')" class="mx-auto max-w-7xl py-6 sm:px-6 lg:px-8">
        Reports content
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { $require } from '@/utilities/viteHelper.js'
import { Disclosure, DisclosureButton, DisclosurePanel, Menu, MenuButton, MenuItem, MenuItems } from '@headlessui/vue'
import { Bars3Icon, BellIcon, XMarkIcon } from '@heroicons/vue/24/outline'
import { useRouter } from 'vue-router';
import { generateAvatar } from '@/utilities/avatar'
import { useUserStore } from '@/stores/sctgDeskStore';
import { LoginApi, UserApi, UserListResponse, PeersResponse, PeerApi, Peer } from '@/api';
import { onMounted, ref } from 'vue';
const userStore = useUserStore();
const router = useRouter();

const user = {
  name: userStore.user?.name || 'Unknown',
  email: userStore.user?.email || '',
  imageUrl: generateAvatar(userStore.user?.name),
}

const users = ref([] as UserListResponse[]);
const peers = ref([] as Peer[]);

const navigation = ref([
  { name: 'Dashboard', href: '#', current: true },
  { name: 'Devices', href: '#', current: false },
  { name: 'Users', href: '#', current: false },
  { name: 'Groups', href: '#', current: false },
  { name: 'Reports', href: '#', current: false },
])
const userNavigation = [
  { name: `${userStore.user?.name} ${userStore.user?.email}`, href: '#', action: nop },
  { name: 'Settings', href: '#', action: nop },
  { name: 'Sign out', href: '#', action: logout },
]

/**
 * Returns the name of the current page in the navigation menu.
 *
 * @return {string} The name of the current page, or 'Dashboard' if no page is currently selected.
 */
function getCurrentPage(): string {
  return navigation.value.find((item) => item.current)?.name || 'Dashboard';
}
/**
 * Checks if the given page is the current page.
 *
 * @param {string} page - The name of the page to check.
 * @return {boolean} True if the page is the current page, false otherwise.
 */
function isCurrentPage(page: string): boolean {
  return navigation.value.find((item) => item.name === page)?.current || false;
}

/**
 * Updates the current page in the navigation menu based on the provided page name.
 *
 * @param {string} page - The name of the page to set as the current page.
 * @return {void} This function does not return anything.
 */
function makePageCurrent(page: string): void {
  navigation.value.forEach((item) => {
    item.current = item.name === page;
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
      router.push({ name: 'login' });
    }
  }).catch((error) => {
    console.error(error);
    alert("Unable to logout");
  });
}

/**
 * Retrieves the list of users from the API.
 *
 * @return {Promise<UserListResponse[]>} A promise that resolves to the list of users.
 */
function getUsers(): Promise<UserListResponse[]> {
  const userApi = new UserApi(userStore.api_configuration);
  return new Promise<UserListResponse[]>((resolve, reject) => {
    //userApi.usersClient();
    userApi.usersClient(1, 2 ^ 32 - 1).then((response) => {
      if (response.status == 200 && response.data.msg == "success") {
        console.log(response.data);
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

function getPeers(): Promise<Peer[]> {
  const peerApi = new PeerApi(userStore.api_configuration);
  return new Promise<Peer[]>((resolve, reject) => {
    peerApi.peers().then((response) => {
      if (response.status == 200 && response.data.msg == "success") {
        console.log(response.data);
        resolve(response.data.data);
      }
      else {
        resolve([] as Peer[]);
      }
    }).catch((error) => {
      console.error(error);
      resolve([] as Peer[]);
    });
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

onMounted(() => {
  getUsers().then((data) => {
    users.value = data;
  });
  getPeers().then((data) => {
    peers.value = data;
  });
});

</script>