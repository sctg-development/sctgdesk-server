<!--
=========================================================
* © 2024-2026 Ronan LE MEILLAT for SCTG Development
=========================================================
Edits one server configuration file (`s3` = s3config.toml, `oauth2` = oauth2.toml) through the settings API.

Two ways to edit, on the same data:
- guided: a form per file (what most administrators want);
- raw: the TOML text, for what the form does not cover (and to keep comments, which the form rewrites).
Nothing is written before the server validated the text: "Validate" only checks, "Save and apply" validates again
on the server and refuses (HTTP 422) an invalid file, so a typo can never break the login of the users.
-->
<template>
  <div>
    <p v-if="loadError" class="rounded-md bg-red-50 p-3 text-sm text-red-700" role="alert">{{ loadError }}</p>
    <template v-else-if="loaded">
      <div class="mb-4 flex flex-wrap items-center justify-between gap-3">
        <div class="text-xs text-gray-500">
          <div>{{ t('settings.file', { path }) }}</div>
          <div v-if="!exists">{{ t('settings.fileMissing') }}</div>
        </div>
        <div class="inline-flex rounded-md shadow-sm" role="group">
          <button v-for="m in MODES" :key="m" type="button" @click="setMode(m)"
            :class="mode === m ? 'bg-indigo-600 text-white' : 'bg-white text-gray-700 hover:bg-gray-50'"
            class="border border-gray-300 px-3 py-1.5 text-sm font-medium first:rounded-l-md last:rounded-r-md">
            {{ m === 'guided' ? t('settings.modeGuided') : t('settings.modeRaw') }}
          </button>
        </div>
      </div>

      <template v-if="mode === 'guided'">
        <S3Form v-if="kind === 's3'" v-model="s3Form" />
        <OAuth2Form v-else v-model="oauthForm" />
        <p class="mt-4 text-xs text-gray-500">{{ t('settings.guidedNote') }}</p>
      </template>
      <textarea v-else v-model="raw" rows="18" spellcheck="false" :aria-label="t('settings.modeRaw')"
        class="block w-full rounded-md border-0 py-2 font-mono text-sm text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300"></textarea>

      <p v-if="parseError" class="mt-3 text-sm text-red-700" role="alert">{{ parseError }}</p>

      <div class="mt-4 flex flex-wrap items-center gap-3">
        <button type="button" :disabled="!!busy" @click="validate()"
          class="rounded-md border border-indigo-600 px-4 py-2 text-sm font-medium text-indigo-600 hover:bg-indigo-50 disabled:opacity-40">
          {{ busy === 'validate' ? t('settings.validating') : t('settings.validate') }}</button>
        <button type="button" :disabled="!!busy" @click="check()"
          class="rounded-md border border-indigo-600 px-4 py-2 text-sm font-medium text-indigo-600 hover:bg-indigo-50 disabled:opacity-40">
          {{ busy === 'check' ? t('settings.checking') : t('settings.check') }}</button>
        <button type="button" :disabled="!!busy" @click="save()"
          class="rounded-md bg-indigo-600 px-4 py-2 text-sm font-medium text-white hover:bg-indigo-500 disabled:opacity-40">{{ t('settings.save') }}</button>
        <button type="button" :disabled="!!busy" @click="load()"
          class="rounded-md border border-gray-300 px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 disabled:opacity-40">{{ t('settings.reload') }}</button>
        <span v-if="dirty" class="text-xs text-amber-700">{{ t('settings.unsaved') }}</span>
      </div>

      <!-- The answer of the server's validation -->
      <div v-if="report" class="mt-4 space-y-2" role="status">
        <p v-if="report.valid && report.errors.length === 0" class="rounded-md bg-green-50 p-3 text-sm text-green-800">{{ reportMessage }}</p>
        <div v-if="report.errors.length" class="rounded-md bg-red-50 p-3 text-sm text-red-800">
          <p class="font-medium">{{ reportMessage || t('settings.invalid') }}</p>
          <ul class="mt-1 list-disc pl-5"><li v-for="e in report.errors" :key="e">{{ e }}</li></ul>
        </div>
        <div v-if="report.warnings.length" class="rounded-md bg-amber-50 p-3 text-sm text-amber-800">
          <p class="font-medium">{{ t('settings.warnings') }}</p>
          <ul class="mt-1 list-disc pl-5"><li v-for="w in report.warnings" :key="w">{{ w }}</li></ul>
        </div>
      </div>

      <!-- The answer of the live check: what the real service says about these values -->
      <div v-if="checks" class="mt-4 rounded-md border border-gray-200 p-3" role="status">
        <p class="mb-2 text-sm font-medium text-gray-700">{{ t('settings.checkTitle') }}</p>
        <ul class="space-y-1 text-sm">
          <li v-for="c in checks" :key="c.target + c.message" :class="LEVEL_CLASS[c.level] ?? 'text-gray-700'">
            <span aria-hidden="true">{{ LEVEL_ICON[c.level] ?? '•' }}</span>
            <span class="font-medium"> {{ c.target }}</span>: {{ c.message }}
          </li>
        </ul>
      </div>

      <!-- A real sign-in per provider: what only a person at the provider's login page can prove -->
      <div v-if="kind === 'oauth2' && savedOps.length" class="mt-6 border-t border-gray-200 pt-4">
        <p class="text-sm font-medium text-gray-700">{{ t('settings.signInSection') }}</p>
        <p class="mb-2 text-xs text-gray-500">{{ t('settings.signInHint') }}</p>
        <div class="flex flex-wrap items-center gap-2">
          <button v-for="op in savedOps" :key="op" type="button" :disabled="!!busy || dirty || !!signInState" @click="testSignIn(op)"
            :title="dirty ? t('settings.saveFirst') : ''"
            class="rounded-md border border-gray-300 px-3 py-1.5 text-sm font-medium text-gray-700 hover:bg-gray-50 disabled:opacity-40">
            {{ t('settings.testSignIn', { op }) }}</button>
        </div>
        <p v-if="signInMessage" class="mt-2 text-sm" :class="signInOk ? 'text-green-700' : 'text-gray-700'" role="status">{{ signInMessage }}</p>
      </div>
    </template>
    <p v-else class="text-sm text-gray-500">{{ t('common.loading') }}</p>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { parse, stringify } from 'smol-toml'
import { t } from '@/i18n'
import { apiRequest } from '@/utilities/http'
import S3Form from './S3Form.vue'
import OAuth2Form from './OAuth2Form.vue'

const props = defineProps<{ kind: 's3' | 'oauth2' }>()

/** What the validate / save endpoints answer. */
interface Report { valid: boolean; errors: string[]; warnings: string[] }

const MODES = ['guided', 'raw'] as const
type Mode = (typeof MODES)[number]

/** The keys of the `[s3config]` table, in the order they are written. `ListBucket` is optional. */
const S3_KEYS = ['Page', 'Endpoint', 'Region', 'AccessKey', 'SecretKey', 'Bucket', 'ListBucket',
  'Windows64Key', 'Windows32Key', 'OSXKey', 'OSXArm64Key', 'IOSKey']
const OPTIONAL_S3_KEYS = ['ListBucket']

const loaded = ref(false)
const loadError = ref('')
const path = ref('')
const exists = ref(true)
/** The text as last loaded/saved; `dirty` compares the current content to it. */
const original = ref('')
const raw = ref('')
const mode = ref<Mode>('guided')
const s3Form = ref<Record<string, string>>({})
const oauthForm = ref<any[]>([])
const parseError = ref('')
const report = ref<Report | null>(null)
const reportMessage = ref('')
const busy = ref<'' | 'validate' | 'save' | 'check'>('')

/** One line of the answer of the live check. */
interface CheckItem { target: string; level: 'ok' | 'warning' | 'error'; message: string }
const LEVEL_ICON: Record<string, string> = { ok: '✔', warning: '⚠', error: '✘' }
const LEVEL_CLASS: Record<string, string> = { ok: 'text-green-700', warning: 'text-amber-700', error: 'text-red-700' }
const checks = ref<CheckItem[] | null>(null)

/** Builds the guided form from TOML text. Throws when the text is not valid TOML. */
function fromToml(text: string): void {
  const doc = parse(text) as any
  if (props.kind === 's3') {
    const table = doc.s3config ?? {}
    s3Form.value = Object.fromEntries(S3_KEYS.map((k) => [k, String(table[k] ?? '')]))
  } else {
    // `op_auth_string` is dropped here: it is derived from `op` on save
    oauthForm.value = (doc.provider ?? []).map((p: any) => ({
      provider: String(p.provider ?? 'Dex'), op: String(p.op ?? ''), app_id: String(p.app_id ?? ''),
      app_secret: String(p.app_secret ?? ''), authorization_url: String(p.authorization_url ?? ''),
      token_exchange_url: String(p.token_exchange_url ?? ''), scope: String(p.scope ?? ''),
    }))
  }
}

/** Serializes the guided form to TOML (the file layout the server reads). Empty optional keys are left out. */
function toToml(): string {
  if (props.kind === 's3') {
    const table: Record<string, string> = {}
    for (const k of S3_KEYS) {
      const value = (s3Form.value[k] ?? '').trim()
      if (value || !OPTIONAL_S3_KEYS.includes(k)) {
        table[k] = value
      }
    }
    return stringify({ s3config: table })
  }
  if (oauthForm.value.length === 0) {
    return '# No oauth2 provider configured.\n'
  }
  return stringify({
    provider: oauthForm.value.map((p) => ({
      provider: p.provider, authorization_url: p.authorization_url.trim(), token_exchange_url: p.token_exchange_url.trim(),
      app_id: p.app_id.trim(), app_secret: p.app_secret, scope: p.scope.trim(),
      // the server looks the provider up by "oidc/<op>": keeping them tied avoids a login button that cannot work
      op_auth_string: `oidc/${p.op.trim()}`, op: p.op.trim(),
    })),
  })
}

/** The text that would be saved right now, whatever the mode. */
function content(): string {
  return mode.value === 'raw' ? raw.value : toToml()
}

const dirty = computed(() => loaded.value && content() !== original.value)

/** Switches between the form and the text, carrying the current content over. */
function setMode(next: Mode): void {
  if (next === mode.value) {
    return
  }
  parseError.value = ''
  if (next === 'raw') {
    raw.value = content()
    mode.value = 'raw'
    return
  }
  try {
    fromToml(raw.value)
    mode.value = 'guided'
  } catch (error) {
    // stay on the text so the administrator can fix it, instead of silently dropping what they typed
    parseError.value = `${t('settings.parseFailed')} (${(error as Error).message})`
  }
}

/** Loads the file from the server and resets the editor to it. */
async function load(): Promise<void> {
  loaded.value = false
  loadError.value = ''
  report.value = null
  parseError.value = ''
  try {
    const data = (await apiRequest<{ path: string; exists: boolean; content: string }>('get', `/api/settings/config/${props.kind}`)).data
    path.value = data.path
    exists.value = data.exists
    raw.value = data.content
    try {
      fromToml(data.content)
      mode.value = 'guided'
    } catch (error) {
      // a file the form cannot read still has to be editable: open it as text
      mode.value = 'raw'
      parseError.value = `${t('settings.parseFailed')} (${(error as Error).message})`
    }
    // the baseline is what the editor produces from the file (not the file itself): comments and formatting
    // alone must not count as an "unsaved change" when the form is only opened
    original.value = mode.value === 'raw' ? data.content : toToml()
    loaded.value = true
  } catch (error) {
    console.error(error)
    loadError.value = t('settings.loadFailed')
  }
}

/** Asks the server to check the current content without saving it. */
async function validate(): Promise<void> {
  busy.value = 'validate'
  reportMessage.value = ''
  try {
    const response = await apiRequest<Report>('post', `/api/settings/config/${props.kind}/validate`, { data: { content: content() } })
    report.value = response.data
    reportMessage.value = response.data.valid ? t('settings.valid') : t('settings.invalid')
  } catch (error) {
    console.error(error)
    report.value = { valid: false, errors: [t('settings.saveFailed')], warnings: [] }
  } finally {
    busy.value = ''
  }
}

/**
 * Asks the server to check the current values against the real service (network access from the server):
 * S3 connection and files, or OAuth2 application. Slow, so it is only run on demand.
 */
async function check(): Promise<void> {
  busy.value = 'check'
  checks.value = null
  try {
    const response = await apiRequest<{ checks: CheckItem[] }>('post', `/api/settings/config/${props.kind}/check`, { data: { content: content() } })
    checks.value = response.data.checks
  } catch (error) {
    console.error(error)
    checks.value = [{ target: props.kind, level: 'error', message: t('settings.saveFailed') }]
  } finally {
    busy.value = ''
  }
}

// ---- Real sign-in test (oauth2 only) -------------------------------------------------------------------------
/** Window of the test, so a second click reuses it and it can be closed at the end. */
let signInWindow: Window | null = null
const signInState = ref(false)
const signInMessage = ref('')
const signInOk = ref(false)
/** The providers of the *saved* file: the server reads the saved file for a sign-in, not the editor. */
const savedOps = computed(() => {
  try {
    return ((parse(original.value) as any).provider ?? []).map((p: any) => String(p.op)).filter(Boolean) as string[]
  } catch {
    return []
  }
})

/**
 * Signs in at the provider for real, with the same endpoints as the login page, and reports the result. The
 * token that comes back is thrown away: the administrator's own session is not touched.
 * The window is opened first, still inside the click handler, or the browser blocks it as an unwanted pop-up.
 */
async function testSignIn(op: string): Promise<void> {
  signInWindow = window.open('about:blank', 'sctgdesk-oauth-test', 'popup=yes,width=520,height=720')
  if (!signInWindow) {
    signInOk.value = false
    signInMessage.value = t('settings.signInPopup')
    return
  }
  signInState.value = true
  signInOk.value = false
  signInMessage.value = t('settings.signInWaiting', { op })
  // a throw-away identity: the server only uses it to match the answer to this test
  const id = 'console-test'
  const uuid = btoa(crypto.randomUUID())
  try {
    const started = (await apiRequest<{ code: string; url: string }>('post', '/api/oidc/auth', {
      data: { deviceInfo: { name: 'console', os: 'console', type: 'oidc' }, id, op, uuid, console: false },
    })).data
    if (!started.url) {
      throw new Error('no url')
    }
    signInWindow.location.href = started.url
    const until = Date.now() + 2 * 60 * 1000
    while (Date.now() < until) {
      await new Promise((resolve) => setTimeout(resolve, 1500))
      // the server answers an error until the person has signed in: keep polling, like the login page does
      const answer = await apiRequest<any>('get', '/api/oidc/auth-query', { params: { code: started.code, id, uuid } })
        .then((r) => r.data)
        .catch(() => null)
      if (answer?.access_token) {
        signInOk.value = true
        signInMessage.value = t('settings.signInOk', { name: answer.user?.name ?? '' })
        return
      }
    }
    signInMessage.value = t('settings.signInTimeout')
  } catch (error) {
    console.error(error)
    signInMessage.value = t('settings.signInFailed')
  } finally {
    signInWindow?.close()
    signInWindow = null
    signInState.value = false
  }
}

/** Saves and applies the content. The server answers 422 with the errors when it is not valid. */
async function save(): Promise<void> {
  busy.value = 'save'
  const text = content()
  try {
    const response = await apiRequest<Report>('put', `/api/settings/config/${props.kind}`, { data: { content: text }, accept: [422] })
    report.value = response.data
    if (response.status === 422 || !response.data.valid) {
      reportMessage.value = t('settings.saveFailed')
    } else {
      reportMessage.value = t('settings.savedOk')
      original.value = text
      exists.value = true
    }
  } catch (error) {
    console.error(error)
    report.value = { valid: false, errors: [t('settings.saveFailed')], warnings: [] }
    reportMessage.value = ''
  } finally {
    busy.value = ''
  }
}

onMounted(load)
</script>
