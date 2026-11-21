/*!
=========================================================
* © 2024-2026 Ronan LE MEILLAT for SCTG Development
=========================================================
Opens the web client (the browser build of the SCTGDesk client) signed in as the console user, optionally
connected to a device.

Hand-off contract with the web client, since it lives on another origin and cannot read our storage:
- the device goes in the query string: `?id=<device id>`;
- the session goes in the URL *fragment*: `#access_token=<token>&user_name=<name>`. A fragment is never sent to
  a server nor written to an access log, unlike a query string.
An administrator can override the shape with placeholders in WEB_CLIENT_URL: `{id}`, `{token}` and `{user}`
are replaced (URL-encoded) and nothing is appended.
*/
import { useProfileStore } from '@/stores/profileStore'
import { useUserStore } from '@/stores/sctgDeskStore'

/** True when the server has a web client configured: the "connect" buttons are only shown then. */
export function webClientAvailable(): boolean {
  return !!useProfileStore().webClientUrl
}

/**
 * Builds the address of the web client for the current session.
 *
 * @param deviceId The device to connect to, or undefined to just open the client.
 * @returns The URL, or null when no web client is configured.
 */
export function webClientUrl(deviceId?: string): string | null {
  const base = useProfileStore().webClientUrl
  if (!base) {
    return null
  }
  const userStore = useUserStore()
  const token = String(userStore.api_configuration?.accessToken ?? '')
  const user = userStore.user?.name ?? ''
  if (/\{(id|token|user)\}/.test(base)) {
    return base
      .replace('{id}', encodeURIComponent(deviceId ?? ''))
      .replace('{token}', encodeURIComponent(token))
      .replace('{user}', encodeURIComponent(user))
  }
  const url = new URL(base, window.location.href)
  if (deviceId) {
    url.searchParams.set('id', deviceId)
  }
  url.hash = `access_token=${encodeURIComponent(token)}&user_name=${encodeURIComponent(user)}`
  return url.toString()
}

/**
 * Builds the link that the operating system hands to the installed native client.
 *
 * The scheme `rustdesk://` is the one the client registers (upstream's, kept by our branding): the browser asks
 * the user to open the application, and does nothing visible when the client is not installed.
 *
 * @param deviceId The device to connect to.
 */
export function nativeClientUrl(deviceId: string): string {
  return `rustdesk://${encodeURIComponent(deviceId)}`
}

/**
 * Opens the web client in a new tab. `noopener` keeps the web client from reaching back into the console
 * (it receives the session token, so it must not be able to script this window).
 */
export function openWebClient(deviceId?: string): void {
  const url = webClientUrl(deviceId)
  if (url) {
    window.open(url, '_blank', 'noopener')
  }
}
