import { Token, UserInfo } from "@/api";
import { defineStore } from "pinia";
export const useUserStore = defineStore('user', {
    state: () => {
      return {
        user: null as UserInfo | null,
        accessToken: null as Token | null,
      }
    },
  })
  