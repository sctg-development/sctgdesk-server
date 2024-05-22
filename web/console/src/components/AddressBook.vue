<template>

</template>
<script setup lang="ts">
import {onMounted, ref} from 'vue';
import {useUserStore} from '@/stores/sctgDeskStore';
import { AbPeer, AddressBookApi } from '@/api';
const userStore = useUserStore();
const configuration = userStore.api_configuration;

const peers = ref<AbPeer[]>([]);

function getAddressBooks() {
    return new Promise<AbPeer[]>((resolve, reject) => {
        const addressBookApi = new AddressBookApi(configuration);
        addressBookApi.abPersonal().then((response) => {
            const personalAddressBook = response.data.guid;
            addressBookApi.abPeers(1,4294967295,personalAddressBook).then((response) => {
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

onMounted(() => {
    console.log("Fetching address books...");
    getAddressBooks().then((_peers) => {
        peers.value = _peers;
        console.log(peers.value);
    }).catch((error) => {
        console.error(error);
    });
});

</script>