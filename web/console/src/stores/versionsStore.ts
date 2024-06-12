import { getClientVersion, getServerVersion } from "@/utilities/api";
import { StoreDefinition, defineStore } from "pinia";

/**
 * The versions store.
 * 
 * @export
 * @type {StoreDefinition<"versions", { serverVersion: string; clientVersion: string; }, {}, { fetchVersion(): Promise<void>; }>}
 * @property {string} serverVersion The server version.
 * @property {string} clientVersion The client version.
 */
export const useVersionsStore: StoreDefinition<"versions", {
    serverVersion: string | null;
    clientVersion: string | null;
}, {}, {
    fetchVersions(): Promise<void>;
}> = defineStore('versions', {
    state: () => ({
        serverVersion: null as string,
        clientVersion: null as string,
    }),
    actions: {
        async fetchVersions() {
            // console.log("Fetching versions");
            if (!this.serverVersion) {
                this.serverVersion = await getServerVersion();
            }
            if (!this.clientVersion) {
                this.clientVersion = await getClientVersion();
            }
        }
    }
});