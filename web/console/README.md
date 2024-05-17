# Vue 3 + Vite + Typescript + Tailwindcss + Fontawesome

This template should help get you started developing with Vue 3 in Vite. The template uses [Vue 3](https://vuejs.org/), [Vite](https://vitejs.dev/), [Tailwind css](https://tailwindcss.com/) and [Fontawesome 6](https://fontawesome.com/).

## Vite

- @ path is defined as ./src
- ~ path is defined as ./node_modules
- § path is defined as ./fontawesome
- npm run dev : launch development environment and serve it to http://localhost:5173
- npm run build : compile, optimize and minify to dist/ directory
- npm run preview : serve dist/ directory to http://localhost:4173

## Howto

- Simply copy this repo with "Use this template" or fork it
- Clone your new repo
- issue "npm i" in your local clone 
- issue "npm run dev"
- browser http://localhost:5173

## Tailwind css

- Tailwind is embedded with my default theme in tailwindcss.config.cjs
- All classes are availables in development environment (usefull for UI debug with devtools)
- Built css is parsed by Purgecss for removing all unused classes, take a look to postcss.config.cjs 

## Fontawesome 6 using private Fontawesome 6 package

- use private Fontawesome 6 package 
create a .npmrc with 
```
@highcanfly-club:registry=https://npm.pkg.github.com
//npm.pkg.github.com/:_authToken=${NPM_GITHUB_TOKEN}
```

- install kit
request a personal access token at Github. For doing that go to your personal settings and hit "developer settings"

```sh
NPM_GITHUB_TOKEN=ghp_jX2t6JA_REAL_TOKEN_2c30tMrA0 npm i -D @highcanfly-club/fontawesome @sctg/fontminify
# if you decide to put your token in the .npmrc be sure to exclude it with .gitignore
```

Also be sure to request the read permission to the package owner.  
If you deploy it using a CI/CD workflow external to Github you must request a personal access token and register an environment variable with your token… See [instructions](https://github.com/highcanfly-club/fontawesome/blob/main/README.md)


## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar)

## License

- [MIT](https://github.com/eltorio/vue-vite-tailwindcss-fontawesome/blob/main/LICENSE.md) for my work
- others are under their own license
