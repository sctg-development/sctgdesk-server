/*!
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
*/
import { toSvg as iconSvg} from 'jdenticon';

/**
 * Generates an avatar image for the given username.
 *
 * @param {string} username - The username to generate the avatar for.
 * @return {string} The data URL of the generated avatar image.
 */
export const generateAvatar = (username: string): string => {
    const svg = iconSvg(username, 200);
    return `data:image/svg+xml,${encodeURIComponent(svg)}`;
}