<!--
=========================================================
* © 2024-2026 Ronan LE MEILLAT for SCTG Development
=========================================================
Guided form of oauth2.toml: the "Sign in with..." providers of the login page.
The form model is the list of `[[provider]]` tables. `op_auth_string` is not edited: the server looks a provider
up by `oidc/<op>`, so it is always derived from the short name (see `providerToToml` in ConfigEditor).
-->
<template>
  <div>
    <p class="mb-4 text-sm text-gray-600">{{ t('settings.oauth2.intro') }}</p>
    <p v-if="model.length === 0" class="mb-4 rounded-md bg-gray-50 p-3 text-sm text-gray-600">{{ t('settings.oauth2.noProvider') }}</p>

    <fieldset v-for="(p, i) in model" :key="i" class="mb-6 rounded-lg border border-gray-200 p-4">
      <legend class="px-2 text-sm font-semibold text-gray-700">{{ t('settings.oauth2.providerN', { n: i + 1 }) }}</legend>
      <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
        <div>
          <label :for="`p${i}-type`" class="block text-sm font-medium text-gray-900">{{ t('settings.oauth2.type') }}</label>
          <select :id="`p${i}-type`" :value="p.provider" @change="changeType(p, ($event.target as HTMLSelectElement).value)"
            class="mt-1 block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 sm:text-sm">
            <option v-for="type in TYPES" :key="type" :value="type">{{ type === 'Dex' ? t('settings.oauth2.custom') : type }}</option>
          </select>
          <p class="mt-1 text-xs text-gray-500">{{ t('settings.oauth2.typeHelp') }}</p>
        </div>
        <div>
          <label :for="`p${i}-op`" class="block text-sm font-medium text-gray-900">{{ t('settings.oauth2.op') }}</label>
          <input :id="`p${i}-op`" v-model="p.op" type="text" spellcheck="false" pattern="[a-z0-9]+"
            class="mt-1 block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 sm:text-sm" />
          <p class="mt-1 text-xs text-gray-500">{{ t('settings.oauth2.opHelp') }} <code>oidc/{{ p.op }}</code></p>
        </div>
        <div>
          <label :for="`p${i}-id`" class="block text-sm font-medium text-gray-900">{{ t('settings.oauth2.appId') }}</label>
          <input :id="`p${i}-id`" v-model="p.app_id" type="text" spellcheck="false"
            class="mt-1 block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 sm:text-sm" />
        </div>
        <div>
          <label :for="`p${i}-secret`" class="block text-sm font-medium text-gray-900">{{ t('settings.oauth2.appSecret') }}</label>
          <SecretInput :id="`p${i}-secret`" v-model="p.app_secret" class="mt-1" />
        </div>
        <div class="md:col-span-2">
          <label :for="`p${i}-auth`" class="block text-sm font-medium text-gray-900">{{ t('settings.oauth2.authorizationUrl') }}</label>
          <input :id="`p${i}-auth`" v-model="p.authorization_url" type="url" spellcheck="false"
            class="mt-1 block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 sm:text-sm" />
        </div>
        <div class="md:col-span-2">
          <label :for="`p${i}-token`" class="block text-sm font-medium text-gray-900">{{ t('settings.oauth2.tokenExchangeUrl') }}</label>
          <input :id="`p${i}-token`" v-model="p.token_exchange_url" type="url" spellcheck="false"
            class="mt-1 block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 sm:text-sm" />
        </div>
        <div class="md:col-span-2">
          <label :for="`p${i}-scope`" class="block text-sm font-medium text-gray-900">{{ t('settings.oauth2.scope') }}</label>
          <input :id="`p${i}-scope`" v-model="p.scope" type="text" spellcheck="false"
            class="mt-1 block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 sm:text-sm" />
        </div>
      </div>
      <div class="mt-3 text-right">
        <button type="button" @click="remove(i)" class="text-sm font-medium text-red-600 hover:text-red-500">{{ t('settings.oauth2.removeProvider') }}</button>
      </div>
    </fieldset>

    <button type="button" @click="add()"
      class="rounded-md border border-indigo-600 px-4 py-2 text-sm font-medium text-indigo-600 hover:bg-indigo-50">{{ t('settings.oauth2.addProvider') }}</button>
  </div>
</template>
<script setup lang="ts">
import { t } from '@/i18n'
import SecretInput from './SecretInput.vue'

/** One `[[provider]]` table, as edited (without `op_auth_string`, derived on save). */
interface ProviderForm {
  provider: string
  op: string
  app_id: string
  app_secret: string
  authorization_url: string
  token_exchange_url: string
  scope: string
}

const model = defineModel<ProviderForm[]>({ required: true })

/** The provider types the server knows (the `Provider` enum of the api-oauth2 crate). `Dex` is the generic one. */
const TYPES = ['Github', 'Gitlab', 'Google', 'Facebook', 'Apple', 'Okta', 'Azure', 'Auth0', 'Dex', 'Oauth2']

/** Addresses and scope that are the same for every installation of a well-known provider. */
const PRESETS: Record<string, Partial<ProviderForm>> = {
  Github: { authorization_url: 'https://github.com/login/oauth/authorize', token_exchange_url: 'https://github.com/login/oauth/access_token', scope: 'read:user', op: 'github' },
  Gitlab: { authorization_url: 'https://gitlab.com/oauth/authorize', token_exchange_url: 'https://gitlab.com/oauth/token', scope: 'read_user', op: 'gitlab' },
  Google: { authorization_url: 'https://accounts.google.com/o/oauth2/v2/auth', token_exchange_url: 'https://oauth2.googleapis.com/token', scope: 'openid email profile', op: 'google' },
  Facebook: { authorization_url: 'https://www.facebook.com/v13.0/dialog/oauth', token_exchange_url: 'https://graph.facebook.com/v13.0/oauth/access_token', scope: 'public_profile', op: 'facebook' },
}
const GENERIC_SCOPE = 'openid email profile'

/** The preset of a type; unknown/custom ones only get the usual OIDC scope. */
function presetOf(type: string): Partial<ProviderForm> {
  return PRESETS[type] ?? { scope: GENERIC_SCOPE, op: type.toLowerCase() }
}

/**
 * Changes the type of a provider and fills its addresses from the preset of the new type.
 * A field the administrator customized (it differs from the preset of the *previous* type) is kept, so
 * switching type never throws away typed values.
 */
function changeType(p: ProviderForm, type: string): void {
  const before = presetOf(p.provider) as Record<string, string>
  const after = presetOf(type) as Record<string, string>
  for (const key of ['authorization_url', 'token_exchange_url', 'scope', 'op']) {
    const current = (p as unknown as Record<string, string>)[key]
    if (!current || current === (before[key] ?? '')) {
      ;(p as unknown as Record<string, string>)[key] = after[key] ?? ''
    }
  }
  p.provider = type
}

function add(): void {
  const type = 'Github'
  model.value = [...model.value, { provider: type, app_id: '', app_secret: '', authorization_url: '', token_exchange_url: '', scope: '', op: '', ...presetOf(type) }]
}

function remove(index: number): void {
  if (window.confirm(t('settings.oauth2.confirmRemove'))) {
    model.value = model.value.filter((_, i) => i !== index)
  }
}
</script>
