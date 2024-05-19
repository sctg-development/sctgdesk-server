import { Token, UserInfo,Configuration } from "@/api";
import { defineStore } from "pinia";
export const useUserStore = defineStore('user', {
    state: () => {
      return {
        user: null as UserInfo | null,
        api_configuration: null as Configuration | null,
      }
    },
  })
  