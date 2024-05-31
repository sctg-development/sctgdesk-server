<template>
    <Modal @modalOk="emit('viewRulesOK')" @modalCancel="emit('viewRulesCancel')" title="Rules">
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
                                        guid
                                    </th>
                                    <th
                                        class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                                        Group
                                    </th>
                                    <th
                                        class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                                        User
                                    </th>
                                    <th
                                        class="w-1/6 min-w-[160px] py-4 px-3 text-lg font-medium text-white lg:py-7 lg:px-4">
                                        Rights
                                    </th>
                                    <th>
                                        &nbsp;
                                    </th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr v-for="rule in rules" :key="rule.guid">
                                    <td
                                        class="text-dark border-b border-l border-[#E8E8E8] bg-[#F3F6FF] dark:bg-dark-3 dark:border-dark dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                                        {{ rule.guid }}
                                    </td>
                                    <td
                                        class="text-dark border-b border-[#E8E8E8] bg-white dark:border-dark dark:bg-dark-2 dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                                        {{ rule.group }}
                                    </td>
                                    <td
                                        class="text-dark border-b border-[#E8E8E8] bg-[#F3F6FF] dark:bg-dark-3 dark:border-dark dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                                        {{ rule.user }}
                                    </td>
                                    <td
                                        class="text-dark border-b border-[#E8E8E8] bg-white dark:border-dark dark:bg-dark-2 dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                                        {{ decodeRights(rule.rule )}}
                                    </td>
                                    <td
                                        class="text-dark border-b border-r border-[#E8E8E8] bg-[#F3F6FF] dark:border-dark dark:bg-dark-2 dark:text-dark-7 py-5 px-2 text-center text-base font-medium">
                                        <a href="javascript:alert('todo')" class="text-blue-500">Edit</a>
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
    </section>
    </Modal>
</template>
<script setup lang="ts">
import { AbRule, AddressBookApi } from '@/api';
import Modal from '@/components/Modal.vue';
import { useUserStore } from '@/stores/sctgDeskStore';
import { onMounted, ref } from 'vue';
const emit = defineEmits(['viewRulesOK', 'viewRulesCancel'])
const userStore = useUserStore();
const configuration = userStore.api_configuration;

const props = defineProps<{
    ab: string,
}>()

const rules = ref<AbRule[]>([]);
function getRules(ab: string) {
    return new Promise<AbRule[]>((resolve, reject) => {
        console.log("Fetching rules");
        const addressBookApi = new AddressBookApi(configuration);
        addressBookApi.abRules(1, 4294967295, ab).then((response) => {
            resolve(response.data.data);
        }).catch((error) => {
            console.error(error);
            reject();
        });
    });
}
onMounted(() => {
    getRules(props.ab).then((data) => {
        console.log("Rules fetched");
        rules.value = data;
    });
});

function decodeRights(rights: number): string {
    switch (rights) {
        case 0:
            return "None";
        case 1:
            return "Read";
        case 2:
            return "Read/Write";
        case 3:
            return "Total Control";
        default:
            return "Unknown";
    }
}
</script>