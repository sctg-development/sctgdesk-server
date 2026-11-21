<!--
=========================================================
* © 2024-2026 Ronan LE MEILLAT for SCTG Development
=========================================================
Settings of the signed-in user: avatar, password and language. They are stored on the server with the account,
so they follow the user from one browser to another.
-->
<template>
  <div class="space-y-8">
    <!-- Avatar -->
    <section class="rounded-3xl bg-white p-6 shadow-xl">
      <h2 class="text-lg font-semibold text-gray-900">{{ t('userSettings.avatarTitle') }}</h2>
      <div class="mt-4 flex flex-wrap items-center gap-6">
        <img :src="profile.avatarUrl" alt="" class="h-24 w-24 rounded-full object-cover ring-1 ring-gray-200" />
        <div>
          <p class="max-w-md text-sm text-gray-600">{{ t('userSettings.avatarHelp', { size: AVATAR_SIZE }) }}</p>
          <div class="mt-3 flex flex-wrap gap-3">
            <input ref="fileInput" type="file" accept="image/png,image/jpeg,image/webp,image/gif" class="hidden"
              @change="onFile" />
            <button type="button" @click="fileInput?.click()"
              class="rounded-md bg-indigo-600 px-4 py-2 text-sm font-medium text-white hover:bg-indigo-500">{{ t('userSettings.avatarChoose') }}</button>
            <button v-if="profile.avatar" type="button" @click="removeAvatar()"
              class="rounded-md border border-gray-300 px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50">{{ t('userSettings.avatarRemove') }}</button>
          </div>
          <p v-if="avatarMessage" role="status" class="mt-2 text-sm" :class="avatarFailed ? 'text-red-700' : 'text-green-700'">{{ avatarMessage }}</p>
        </div>
      </div>
    </section>

    <!-- Password -->
    <section class="rounded-3xl bg-white p-6 shadow-xl">
      <h2 class="text-lg font-semibold text-gray-900">{{ t('userSettings.passwordTitle') }}</h2>
      <form class="mt-4 grid max-w-md gap-4" @submit.prevent="changePassword()">
        <div>
          <label for="pw-current" class="block text-sm font-medium text-gray-900">{{ t('userSettings.currentPassword') }}</label>
          <input id="pw-current" v-model="currentPassword" type="password" autocomplete="current-password"
            class="mt-1 block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 sm:text-sm" />
        </div>
        <div>
          <label for="pw-new" class="block text-sm font-medium text-gray-900">{{ t('userSettings.newPassword') }}</label>
          <input id="pw-new" v-model="newPassword" type="password" autocomplete="new-password" minlength="8"
            class="mt-1 block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 sm:text-sm" />
          <p class="mt-1 text-xs text-gray-500">{{ t('userSettings.passwordHelp') }}</p>
        </div>
        <div>
          <label for="pw-confirm" class="block text-sm font-medium text-gray-900">{{ t('userSettings.confirmPassword') }}</label>
          <input id="pw-confirm" v-model="confirmPassword" type="password" autocomplete="new-password"
            class="mt-1 block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 sm:text-sm" />
        </div>
        <div>
          <button type="submit"
            class="rounded-md bg-indigo-600 px-4 py-2 text-sm font-medium text-white hover:bg-indigo-500">{{ t('userSettings.passwordChange') }}</button>
          <p v-if="passwordMessage" role="status" class="mt-2 text-sm" :class="passwordFailed ? 'text-red-700' : 'text-green-700'">{{ passwordMessage }}</p>
        </div>
      </form>
    </section>

    <!-- Language -->
    <section class="rounded-3xl bg-white p-6 shadow-xl">
      <h2 class="text-lg font-semibold text-gray-900">{{ t('userSettings.languageTitle') }}</h2>
      <p class="mt-2 text-sm text-gray-600">{{ t('userSettings.languageHelp') }}</p>
      <label for="user-language" class="sr-only">{{ t('userSettings.languageTitle') }}</label>
      <select id="user-language" :value="locale" @change="changeLanguage(($event.target as HTMLSelectElement).value)"
        class="mt-3 block w-60 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 sm:text-sm">
        <option v-for="l in LOCALES" :key="l.code" :value="l.code">{{ l.label }}</option>
      </select>
      <p v-if="languageMessage" role="status" class="mt-2 text-sm" :class="languageFailed ? 'text-red-700' : 'text-green-700'">{{ languageMessage }}</p>
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { LOCALES, locale, t } from '@/i18n'
import { useProfileStore } from '@/stores/profileStore'
import { apiRequest } from '@/utilities/http'

const profile = useProfileStore()

/** Side of the stored avatar. Resizing in the browser keeps the upload tiny and the header sharp enough. */
const AVATAR_SIZE = 128
/** Pictures bigger than this are refused before even trying to decode them. */
const MAX_FILE_BYTES = 8 * 1024 * 1024

const fileInput = ref<HTMLInputElement | null>(null)
const avatarMessage = ref('')
const avatarFailed = ref(false)

const currentPassword = ref('')
const newPassword = ref('')
const confirmPassword = ref('')
const passwordMessage = ref('')
const passwordFailed = ref(false)

const languageMessage = ref('')
const languageFailed = ref(false)

/**
 * Turns a picture file into a small square data URL: centered crop, then scaled to AVATAR_SIZE.
 * WebP is preferred (smaller); browsers that cannot encode it silently give PNG, which is also accepted.
 */
async function toAvatarDataUrl(file: File): Promise<string> {
  const bitmap = await createImageBitmap(file)
  const side = Math.min(bitmap.width, bitmap.height)
  const canvas = document.createElement('canvas')
  canvas.width = canvas.height = AVATAR_SIZE
  const context = canvas.getContext('2d')!
  context.drawImage(bitmap, (bitmap.width - side) / 2, (bitmap.height - side) / 2, side, side, 0, 0, AVATAR_SIZE, AVATAR_SIZE)
  bitmap.close()
  return canvas.toDataURL('image/webp', 0.9)
}

async function onFile(event: Event): Promise<void> {
  const input = event.target as HTMLInputElement
  const file = input.files?.[0]
  input.value = '' // so that choosing the same file again fires `change` again
  if (!file) {
    return
  }
  avatarMessage.value = ''
  avatarFailed.value = true
  if (!file.type.startsWith('image/') || file.size > MAX_FILE_BYTES) {
    avatarMessage.value = t('userSettings.avatarInvalid')
    return
  }
  try {
    await profile.saveAvatar(await toAvatarDataUrl(file))
    avatarFailed.value = false
    avatarMessage.value = t('userSettings.avatarSaved')
  } catch (error) {
    console.error(error)
    avatarMessage.value = t('userSettings.avatarFailed')
  }
}

async function removeAvatar(): Promise<void> {
  try {
    await profile.saveAvatar(null)
    avatarFailed.value = false
    avatarMessage.value = t('userSettings.avatarSaved')
  } catch (error) {
    console.error(error)
    avatarFailed.value = true
    avatarMessage.value = t('userSettings.avatarFailed')
  }
}

/** Checks the form like the server will (length, confirmation), then sends it. */
async function changePassword(): Promise<void> {
  passwordFailed.value = true
  if (newPassword.value.length < 8) {
    passwordMessage.value = t('userSettings.passwordTooShort')
    return
  }
  if (newPassword.value !== confirmPassword.value) {
    passwordMessage.value = t('userSettings.passwordMismatch')
    return
  }
  try {
    await apiRequest('put', '/api/user/password', {
      data: { current_password: currentPassword.value, new_password: newPassword.value },
    })
    passwordFailed.value = false
    passwordMessage.value = t('userSettings.passwordSaved')
    currentPassword.value = newPassword.value = confirmPassword.value = ''
  } catch (error: any) {
    console.error(error)
    // 400/403 = the current password is wrong; anything else is a failure we cannot explain better
    const status = error?.response?.status
    passwordMessage.value = status === 400 || status === 403 ? t('userSettings.passwordWrong') : t('userSettings.passwordFailed')
  }
}

async function changeLanguage(code: string): Promise<void> {
  languageMessage.value = ''
  try {
    await profile.saveLanguage(code)
    languageFailed.value = false
    languageMessage.value = t('userSettings.languageSaved')
  } catch (error) {
    console.error(error)
    // the language still changed in this browser (see saveLanguage): only the server copy is missing
    languageFailed.value = true
    languageMessage.value = t('userSettings.languageFailed')
  }
}
</script>
