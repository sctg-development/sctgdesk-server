import { toSvg as iconSvg} from 'jdenticon';

export const generateAvatar = (username: string): string => {
    const svg = iconSvg(username, 200);
    return `data:image/svg+xml,${encodeURIComponent(svg)}`;
}