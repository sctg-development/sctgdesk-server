<!--
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
-->
<template>
    <button
        class="text-white h-10 bg-gray-800 hover:bg-gray-900 focus:outline-none focus:ring-4 focus:ring-gray-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700"
        @click="bModalAddSharedAb = !bModalAddSharedAb">Create a shared address book</button>
    <AddressBook class="mb-4" name="Personal address book" :peers="ab_personal_peers" :isPersonal="true"
        :ab="ab_personal_guid" />
    <div v-for="sharedAddressBook in ab_shared_address_books" :key="sharedAddressBook.name">
        <AddressBook class="mb-4" 
            :name="sharedAddressBook.name" :peers="sharedAddressBook.peers" :isPersonal="false"
            :ab="sharedAddressBook.guid"
            @need-refresh="need_refresh"/>
    </div>
    <div class="h-24"></div>
    <AddSharedAddressBook v-if="bModalAddSharedAb" @shared_ab_added="shared_ab_added()"
        @add_sharedab_close="bModalAddSharedAb = false" />
</template>
<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import { useUserStore } from '@/stores/sctgDeskStore';
import { AbPeer, AddressBookApi } from '@/api';
import AddressBook from '@/components/AddressBook.vue';
import AddSharedAddressBook from '@/components/AddSharedAddressBook.vue';

type sharedAddressBooks = {
    guid: string;
    name: string;
    peers: AbPeer[];
}[];

const userStore = useUserStore();
const configuration = userStore.api_configuration;

const bModalAddSharedAb = ref(false);

const ab_personal_peers = ref<AbPeer[]>([]);
const ab_personal_guid = ref<string>("");
const ab_shared_address_books = ref<sharedAddressBooks>([]);

function getPersonalAddressBooks() {
    return new Promise<AbPeer[]>((resolve, reject) => {
        const addressBookApi = new AddressBookApi(configuration);
        addressBookApi.abPersonal().then((response) => {
            const personalAddressBook = response.data.guid;
            ab_personal_guid.value = personalAddressBook;
            addressBookApi.abPeers(1, 4294967295, personalAddressBook).then((response) => {
                const peers = response.data.data;
                resolve(peers);
            }).catch((error) => {
                reject(error);
            });
        }).catch((error) => {
            reject(error);
        });
    });
}

function getSharedAddressBooks() {
    return new Promise<sharedAddressBooks>((resolve, reject) => {
        const addressBookApi = new AddressBookApi(configuration);
        addressBookApi.abShared().then((response) => {
            const sharedAddressBooksId = response.data.data;
            const sharedAddressBooks: sharedAddressBooks = [];
            const sharedAddressBookPromises = sharedAddressBooksId.map((sharedAddressBook) => {
                return addressBookApi.abPeers(1, 4294967295, sharedAddressBook.guid).then((response) => {
                    sharedAddressBooks.push({
                        guid: sharedAddressBook.guid,
                        name: sharedAddressBook.name,
                        peers: response.data.data,
                    });
                });
            });
            Promise.all(sharedAddressBookPromises).then(() => {
                console.log("Shared address books fetched");
                resolve(sharedAddressBooks);
            }).catch((error) => {
                reject(error);
            });
        }).catch((error) => {
            reject(error);
        });
    });
}

onMounted(() => {
    console.log("Fetching address books...");
    refresh_address_books();
});


function shared_ab_added() {
    bModalAddSharedAb.value = false;
    refresh_address_books();
}
/**
 * Refreshes the personal and shared address books.
 *
 * This function retrieves the personal address books and shared address books
 * using the `getPersonalAddressBooks` and `getSharedAddressBooks` functions respectively.
 * The personal address books are stored in the `ab_personal_peers` reactive variable,
 * while the shared address books are stored in the `ab_shared_address_books` reactive variable.
 * If there is an error during the retrieval of the personal or shared address books,
 * the error is logged to the console.
 *
 * @return {void} This function does not return any value.
 */
function refresh_address_books() : void{
    getPersonalAddressBooks().then((_peers) => {
        ab_personal_peers.value = _peers;
    }).catch((error) => {
        console.error(error);
    });
    getSharedAddressBooks().then((_sharedAddressBooks) => {
        ab_shared_address_books.value = _sharedAddressBooks;
    }).catch((error) => {
        console.error(error);
    });
}

function need_refresh(event: Event) {
    console.log("Need refresh");
    refresh_address_books();
}
</script>