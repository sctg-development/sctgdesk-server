/**
 * 
 * @param url absolute path of asset (must begin with @/assets/)
 * @returns a vite transformed url string
 */
export function $require(url: string): string {
  const baseUrl = import.meta.url;
  const correctedUrl = new URL(`../assets/${url.replace('@/assets/', '')}`, baseUrl);
  return correctedUrl.href;
}
