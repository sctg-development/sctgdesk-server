/*!
=========================================================
* © 2024-2026 Ronan LE MEILLAT for SCTG Development
=========================================================
Helpers of the "new server version available" banner: the GitHub lookup, the version comparison and the cookie
that remembers that the administrator does not want to see the banner again.
*/

/** The repository whose releases announce a new server version. */
export const REPO_URL = 'https://github.com/sctg-development/sctgdesk-server'
const LATEST_RELEASE_API = 'https://api.github.com/repos/sctg-development/sctgdesk-server/releases/latest'
/** Name of the cookie set by "don't show again". */
export const HIDE_COOKIE = 'sctgdesk_hide_update_banner'

/** What the banner needs to know about the newest release. */
export interface LatestRelease { version: string; url: string }

/**
 * Turns a version or a release tag into the list of its numbers, so that `1.1.16-sctg.10` is newer than
 * `1.1.16-sctg.9` (a plain string comparison would get that wrong). The `sctg-v` prefix of our tags is ignored.
 */
export function versionParts(version: string): number[] {
  return (version.match(/\d+/g) ?? []).map(Number)
}

/**
 * Tells whether `candidate` is strictly newer than `current`.
 * Missing trailing numbers count as 0. Returns false when either side has no number at all, so an unreadable
 * version never produces a banner.
 */
export function isNewer(candidate: string, current: string): boolean {
  const a = versionParts(candidate)
  const b = versionParts(current)
  if (a.length === 0 || b.length === 0) {
    return false
  }
  for (let i = 0; i < Math.max(a.length, b.length); i++) {
    const diff = (a[i] ?? 0) - (b[i] ?? 0)
    if (diff !== 0) {
      return diff > 0
    }
  }
  return false
}

/**
 * Asks GitHub for the latest published release (drafts and pre-releases are never returned by this endpoint).
 * The call is made by the browser: the server may have no internet access, and the anonymous limit of GitHub
 * (60 requests per hour and address) is far above one request per console visit.
 *
 * @returns The release, or null on any failure: the banner is a convenience and must never show an error.
 */
export async function fetchLatestRelease(): Promise<LatestRelease | null> {
  try {
    const response = await fetch(LATEST_RELEASE_API, { headers: { Accept: 'application/vnd.github+json' } })
    if (!response.ok) {
      return null
    }
    const data = await response.json()
    const tag = String(data.tag_name ?? '')
    // only ever link to GitHub, whatever the answer contains
    const url = String(data.html_url ?? '')
    return tag ? { version: tag.replace(/^sctg-v/, ''), url: url.startsWith(REPO_URL) ? url : `${REPO_URL}/releases` } : null
  } catch {
    return null
  }
}

/** True when the administrator chose "don't show again". */
export function bannerHidden(): boolean {
  return document.cookie.split('; ').some((c) => c.startsWith(`${HIDE_COOKIE}=`))
}

/** Remembers "don't show again" for a year. `SameSite=Lax` and `path=/` keep it for the whole console. */
export function hideBannerForever(): void {
  document.cookie = `${HIDE_COOKIE}=1; max-age=${365 * 24 * 3600}; path=/; SameSite=Lax`
}
