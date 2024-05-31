import { UserApi, UserListResponse } from "@/api";
import { GroupApi, Group } from '@/api';
import { useUserStore } from "@/stores/sctgDeskStore";

/**
 * Retrieves the list of users from the API.
 *
 * @return {Promise<UserListResponse[]>} A promise that resolves to the list of users.
 */
export function getUsers(): Promise<UserListResponse[]> {
    const userStore = useUserStore();
    const userApi = new UserApi(userStore.api_configuration);
    return new Promise<UserListResponse[]>((resolve, reject) => {
        userApi.usersClient(1, 4294967295).then((response) => {
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

/**
 * Retrieves a list of groups from the server.
 *
 * @return {Promise<Group[]>} A promise that resolves with an array of Group objects representing the groups.
 */
export function getGroups(): Promise<Group[]> {
    const userStore = useUserStore();
    const groupApi = new GroupApi(userStore.api_configuration);
    return new Promise<Group[]>((resolve, reject) => {
        groupApi.groups(1, 4294967295).then((response) => {
            if (response.status == 200 && response.data.msg == "success") {
                resolve(response.data.data);
            }
            else {
                resolve([] as Group[]);
            }
        }).catch((error) => {
            console.error(error);
            resolve([] as Group[]);
        });
    }
    );
}