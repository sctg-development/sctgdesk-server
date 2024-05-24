import { Token, UserInfo,Configuration } from "@/api";
import { defineStore } from "pinia";
export const useUserStore = defineStore('user', {
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
  