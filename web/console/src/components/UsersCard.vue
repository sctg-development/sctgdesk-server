<template>
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
</template>
<script setup lang="ts">
import { UserApi, UserListResponse } from '@/api';
import { useUserStore } from '@/stores/sctgDeskStore';
import { onMounted, ref } from 'vue';

const userStore = useUserStore();
const users = ref([] as UserListResponse[]);

onMounted(() => {
  getUsers().then((data) => {
    users.value = data;
  });
});

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
</script>