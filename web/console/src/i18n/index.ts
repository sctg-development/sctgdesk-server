/*!
=========================================================
* © 2024-2026 Ronan LE MEILLAT for SCTG Development
=========================================================
Minimal i18n layer of the web console (no external dependency).

Design notes for maintainers:
- Messages are plain nested objects, one folder per locale under `locales/`. A locale folder merges several
  "namespaces" (`core.ts`, `admin.ts`, ...) so that different features can be translated independently.
- `en-US` is the reference: every key must exist there, and it is the fallback for a key missing elsewhere
  (so a half-translated locale never shows a raw key).
- The active locale is a module-level `ref`, hence reactive for every component calling `t()` in its template.
- The choice is persisted in `localStorage` (works before login) and, once logged in, mirrored on the server
  profile (see `useProfileStore`) so that it follows the user across browsers.
*/
import { ref } from 'vue'
import enUS from './locales/en-US'
import frFR from './locales/fr-FR'
import deDE from './locales/de-DE'
import esES from './locales/es-ES'
import ptPT from './locales/pt-PT'
import itIT from './locales/it-IT'

/** A tree of messages: leaves are strings, `{name}` placeholders are replaced by `t()`. */
export type Messages = { [key: string]: string | Messages }

/** The locales offered by the console, with the name each one is shown with in the selectors. */
export const LOCALES = [
  { code: 'en-US', label: 'English' },
  { code: 'fr-FR', label: 'Français' },
  { code: 'de-DE', label: 'Deutsch' },
  { code: 'es-ES', label: 'Español' },
  { code: 'pt-PT', label: 'Português' },
  { code: 'it-IT', label: 'Italiano' },
] as const

export type LocaleCode = (typeof LOCALES)[number]['code']

const MESSAGES: Record<LocaleCode, Messages> = {
  'en-US': enUS,
  'fr-FR': frFR,
  'de-DE': deDE,
  'es-ES': esES,
  'pt-PT': ptPT,
  'it-IT': itIT,
}

const STORAGE_KEY = 'sctgdesk.locale'
const DEFAULT_LOCALE: LocaleCode = 'en-US'

/** True when `code` is one of the supported locales (the server uses the same list). */
export function isLocale(code: unknown): code is LocaleCode {
  return typeof code === 'string' && code in MESSAGES
}

/**
 * Picks the first locale to use: the one saved in the browser, else the browser language (an exact match, or
 * the same language in another region, e.g. `fr-CA` -> `fr-FR`), else English.
 */
function detectLocale(): LocaleCode {
  try {
    const saved = localStorage.getItem(STORAGE_KEY)
    if (isLocale(saved)) {
      return saved
    }
  } catch {
    // localStorage can be blocked (private mode): fall through to the browser language
  }
  for (const language of navigator.languages ?? [navigator.language]) {
    if (isLocale(language)) {
      return language
    }
    const base = language.split('-')[0].toLowerCase()
    const match = LOCALES.find((l) => l.code.toLowerCase().startsWith(base + '-'))
    if (match) {
      return match.code
    }
  }
  return DEFAULT_LOCALE
}

/** The active locale. Read it in a template to re-render when the user changes the language. */
export const locale = ref<LocaleCode>(detectLocale())

/** Changes the active locale (ignored when unknown) and remembers it in the browser. */
export function setLocale(code: string): void {
  if (!isLocale(code)) {
    return
  }
  locale.value = code
  try {
    localStorage.setItem(STORAGE_KEY, code)
  } catch {
    // not persisted, the choice still applies to this session
  }
  document.documentElement.lang = code
}

/** Walks a dotted path (`"nav.devices"`) in a message tree; undefined when it is not a string leaf. */
function lookup(tree: Messages, key: string): string | undefined {
  let node: string | Messages | undefined = tree
  for (const part of key.split('.')) {
    if (typeof node !== 'object' || node === null) {
      return undefined
    }
    node = node[part]
  }
  return typeof node === 'string' ? node : undefined
}

/**
 * Translates `key` in the active locale, falling back to English, then to the key itself (visible in the UI,
 * so a missing key is easy to spot). `{name}` placeholders are replaced from `params`.
 */
export function t(key: string, params?: Record<string, string | number>): string {
  const message = lookup(MESSAGES[locale.value], key) ?? lookup(MESSAGES[DEFAULT_LOCALE], key) ?? key
  if (!params) {
    return message
  }
  return message.replace(/\{(\w+)\}/g, (whole, name) => (name in params ? String(params[name]) : whole))
}

/** Composable form, for `<script setup>` code that prefers destructuring. */
export function useI18n() {
  return { t, locale, setLocale, locales: LOCALES }
}

document.documentElement.lang = locale.value
