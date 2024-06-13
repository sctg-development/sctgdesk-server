/*!
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
*/
import { toSvg as iconSvg} from 'jdenticon';

export const generateAvatar = (username: string): string => {
    const svg = iconSvg(username, 200);
    return `data:image/svg+xml,${encodeURIComponent(svg)}`;
}