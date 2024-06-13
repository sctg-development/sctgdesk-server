/*!
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
*/

import { Configuration, SoftwareApi, UserApi, UserListResponse } from "@/api";
import { GroupApi, Group } from '@/api';
import { useUserStore } from "@/stores/sctgDeskStore";

/**
 * The base path for the API.
 * This is the origin of the current window if it is localhost, otherwise it is the origin of the current window.
 * This is need for development purposes.
 */
export const basePath = window.location.origin == "http://localhost:5173" ? "http://127.0.0.1:21114" : window.location.origin;

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

/**
 * Retrieves the server version.
 * 
 * @return {Promise<string>} A promise that resolves with the server version.
 */
export function getServerVersion(): Promise<string> {
    const configuration = new Configuration({ basePath: basePath });
    const softwareApi = new SoftwareApi(configuration);
    return new Promise<string>((resolve, reject) => {
        softwareApi.softwareVersion().then((response) => {
            if (response.status == 200) {
                resolve(response.data.server);
            }
            else {
                resolve("");
            }
        }).catch((error) => {
            console.error(error);
            resolve("");
        });
    });
}

/**
 * Retrieves the client version.
 * 
 * @return {Promise<string>} A promise that resolves with the client version.
 */
export function getClientVersion(): Promise<string> {
    const configuration = new Configuration({ basePath: basePath });
    const softwareApi = new SoftwareApi(configuration);
    return new Promise<string>((resolve, reject) => {
        softwareApi.softwareVersion().then((response) => {
            if (response.status == 200) {
                resolve(response.data.client);
            }
            else {
                resolve("");
            }
        }).catch((error) => {
            console.error(error);
            resolve("");
        });
    });
}