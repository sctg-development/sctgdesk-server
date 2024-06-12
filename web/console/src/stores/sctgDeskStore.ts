import { Token, UserInfo, Configuration } from "@/api";
import { StoreDefinition, defineStore } from "pinia";

/**
 * The user store.
 * 
 * @export
 * @type {StoreDefinition<"user", { user: UserInfo; api_configuration: Configuration; id: string; uuid_base64: string; oidc_code: string; oidc_provider: string; }, {}, {}>}
 * @property {UserInfo | null} user The user information.
 * @property {Configuration | null} api_configuration The API configuration.
 * @property {string | null} id The user ID.
 * @property {string | null} uuid_base64 The user UUID in base64.
 * @property {string | null} oidc_code The OIDC code.
 * @property {string | null} oidc_provider The OIDC provider.
 **/
export const useUserStore: StoreDefinition<"user", {
  user: UserInfo;
  api_configuration: Configuration;
  id: string;
  uuid_base64: string;
  oidc_code: string;
  oidc_provider: string;
}, {}, {}> = defineStore('user', {
  state: () => {
    return {
      user: null as UserInfo | null,
      api_configuration: null as Configuration | null,
      id: null as string | null,
      uuid_base64: null as string | null,
      oidc_code: null as string | null,
      oidc_provider: null as string | null,
    }
  },
})
