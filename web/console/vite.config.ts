/*!
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vite, Vue3, FontAwesome 6, TailwindCss 3
- And many others
*/
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import fs from "fs";
import path from 'path'
//import vitePluginFontawesomeminify from '@highcanfly-club/fontawesome'
import { viteStaticCopy } from 'vite-plugin-static-copy'

export default defineConfig({
  base: "/ui",
  plugins: [vue(), viteStaticCopy({
    targets: [
      {
        src: './src/assets/sctg.svg',
        dest: 'assets'
      }
    ],
    silent: false
  })],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
      "~": path.resolve(__dirname, "./node_modules"),
      "§": path.resolve(__dirname, "./"),
    },
  },
  server: {
    https: fs.existsSync("./localhost.key")
      ? {
        key: fs.readFileSync("./localhost.key"),
        cert: fs.readFileSync("./localhost.pem"),
      }
      : false,
  },
});
