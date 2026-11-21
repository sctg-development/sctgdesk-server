/*!
=========================================================
* © 2024-2026 Ronan LE MEILLAT for SCTG Development
=========================================================
Tailwind 4 runs as a PostCSS plugin and brings its own vendor prefixing and removal of unused CSS, which
replaces autoprefixer and purgecss of the previous setup.
*/
import tailwindcss from '@tailwindcss/postcss'

export default {
  plugins: [tailwindcss],
}
