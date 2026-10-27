/*!
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
*/
import tailwindcss from 'tailwindcss'
import autoprefixer from 'autoprefixer'
import purgecss from '@fullhuman/postcss-purgecss'

export default {
  plugins: [
    tailwindcss,
    autoprefixer,
    (process.env.NODE_ENV !== 'development') ? purgecss({
      content: ['./index.html', './src/**/*.ts', './src/**/*.js', './src/**/*.vue'],
      defaultExtractor(content) {
        const contentWithoutStyleBlocks = content.replace(/<style[^]+?<\/style>/gi, '')
        return contentWithoutStyleBlocks.match(/[A-Za-z0-9-_/:]*[A-Za-z0-9-_/]+/g) || []
      },
      safelist: [/-(leave|enter|appear)(|-(to|from|active))$/, /^(?!(|.*?:)cursor-move).+-move$/, /^router-link(|-exact)-active$/, /data-v-.*/,/tick.*/,/pt-1\.5/],
      fontFace: true
    }) : ()=>{},
  ]
}