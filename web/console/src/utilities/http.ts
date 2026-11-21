/*!
=========================================================
* © 2024-2026 Ronan LE MEILLAT for SCTG Development
=========================================================
Authenticated calls to the endpoints that are newer than the generated client in `src/api` (profile, settings,
purge of the connections, ...). The generated client is not regenerated for each of them: it would add a huge,
noisy diff for a few JSON shapes, which are declared next to their use instead.
*/
import axios, { AxiosResponse } from 'axios'
import { useUserStore } from '@/stores/sctgDeskStore'
import { basePath } from '@/utilities/api'

/**
 * Calls the API server with the access token of the current session.
 *
 * @param method HTTP verb.
 * @param path Path starting with `/api/...`.
 * @param options `params` (query string) and `data` (JSON body).
 * @returns The axios response. Non-2xx statuses reject, except those listed in `accept` (validation errors
 * such as 422 carry a body the UI wants to show).
 */
export function apiRequest<T = any>(
  method: 'get' | 'post' | 'put' | 'delete',
  path: string,
  options: { params?: object; data?: object; accept?: number[] } = {},
): Promise<AxiosResponse<T>> {
  const configuration = useUserStore().api_configuration
  return axios.request<T>({
    url: `${configuration?.basePath || basePath}${path}`,
    method,
    params: options.params,
    data: options.data,
    headers: { Authorization: `Bearer ${configuration?.accessToken}` },
    validateStatus: (status) => (status >= 200 && status < 300) || (options.accept ?? []).includes(status),
  })
}
