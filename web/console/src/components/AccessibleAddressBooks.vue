<!--
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
-->
<template>
    <AddressBook class="mb-4" name="Personal address book" :peers="ab_personal_peers" :isPersonal="true"/>
    <AddressBook class="mb-4" v-for="sharedAddressBook in ab_shared_address_books" :key="sharedAddressBook.name" :name="sharedAddressBook.name" :peers="sharedAddressBook.peers" :isPersonal="false"/>
    <div class="h-24"></div>
</template>
<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useUserStore } from '@/stores/sctgDeskStore';
import { AbPeer, AbProfile, AddressBookApi } from '@/api';
import AddressBook from '@/components/AddressBook.vue';

type sharedAddressBooks = {
    name: string;
    peers: AbPeer[];
}[];

const userStore = useUserStore();
const configuration = userStore.api_configuration;

const ab_personal_peers = ref<AbPeer[]>([]);
const ab_shared_address_books = ref<sharedAddressBooks>([]);

function getPersonalAddressBooks() {
    return new Promise<AbPeer[]>((resolve, reject) => {
        const addressBookApi = new AddressBookApi(configuration);
        addressBookApi.abPersonal().then((response) => {
            const personalAddressBook = response.data.guid;
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
});

</script>