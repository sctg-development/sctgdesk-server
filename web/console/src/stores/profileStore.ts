/*!
=========================================================
* © 2024-2026 Ronan LE MEILLAT for SCTG Development
=========================================================
The profile of the signed-in user that the server stores for them: avatar and language (the other fields of
the account stay in `useUserStore`). Kept in its own store so that a change of avatar updates the header
immediately, without reloading the page.
*/
import { defineStore } from 'pinia'
import { apiRequest } from '@/utilities/http'
import { generateAvatar } from '@/utilities/avatar'
import { isLocale, setLocale } from '@/i18n'
import { useUserStore } from '@/stores/sctgDeskStore'

/** What `GET/PUT /api/user/profile` returns. */
export interface Profile {
  name: string
  email?: string | null
  admin: boolean
  /** A `data:image/...;base64,` URL, or null when the user has none (a generated identicon is shown). */
  avatar: string | null
  language: string | null
}

export const useProfileStore = defineStore('profile', {
  state: () => ({
    avatar: null as string | null,
    /** The URL of the web client, null when the server has none configured (the connect buttons are hidden). */
    webClientUrl: null as string | null,
  }),
  getters: {
    /** The image to show for the user: their own avatar, else an identicon of their name. */
    avatarUrl(state): string {
      return state.avatar || generateAvatar(useUserStore().user?.name ?? 'unknown')
    },
  },
  actions: {
    /**
     * Loads the profile (and the web client address) after a login. Failures are logged and ignored: an old
     * server without these endpoints must still give a working console.
     */
    async load(): Promise<void> {
      try {
        const profile = (await apiRequest<Profile>('get', '/api/user/profile')).data
        this.avatar = profile.avatar ?? null
        // The language saved on the server wins over the browser's one: it is the user's explicit choice.
        if (isLocale(profile.language)) {
          setLocale(profile.language)
        }
      } catch (error) {
        console.warn('profile not available', error)
      }
      try {
        const reply = (await apiRequest<{ url: string | null }>('get', '/api/webclient')).data
        this.webClientUrl = reply.url || null
      } catch (error) {
        this.webClientUrl = null
      }
    },
    /** Saves a new avatar (data URL) or removes it (null). Throws when the server refuses it. */
    async saveAvatar(avatar: string | null): Promise<void> {
      const profile = (await apiRequest<Profile>('put', '/api/user/profile', { data: { avatar } })).data
      this.avatar = profile.avatar ?? null
    },
    /** Saves the language on the server, and applies it at once. */
    async saveLanguage(language: string): Promise<void> {
      setLocale(language)
      await apiRequest('put', '/api/user/profile', { data: { language } })
    },
    /** Forgets everything on logout. */
    reset(): void {
      this.avatar = null
      this.webClientUrl = null
    },
  },
})
