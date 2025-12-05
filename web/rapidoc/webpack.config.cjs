// webpack.config.cjs
const path = require("path");
const webpack = require("webpack");

const commonConfig = {
  entry: "./index.ts", // Point d'entrée de votre application
  output: {
    filename: "openapisnippet.min.js", // Nom du fichier de sortie
    path: path.resolve(__dirname, "dist"), // Répertoire de sortie
  },
  resolve: {
    alias: { process: "process/browser" },
    fallback: {
      crypto: require.resolve("crypto-browserify"),
      vm: require.resolve("vm-browserify"),
      stream: require.resolve("stream-browserify"),
      buffer: require.resolve("buffer/"),
      url: require.resolve("url/"),
      querystring: require.resolve("querystring-es3"),
      path: require.resolve("path-browserify"),
      util: require.resolve("util/"),
      https: require.resolve("https-browserify"),
      http: require.resolve("stream-http"),
      fs: false,
    },
  },
  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: /node_modules/,
        use: {
          loader: "babel-loader", // Utiliser Babel pour transpiler le code
          options: {
            targets: "defaults",
            presets: [["@babel/preset-env"]],
          },
        },
      },
      {
        test: /\.ts$/,
        exclude: /node_modules/,
        use: {
          loader: "babel-loader",
          options: {
            presets: ["@babel/preset-typescript"],
            plugins: ["@babel/plugin-syntax-import-attributes"],
          },
        },
      },
    ],
  },
  plugins: [
    new webpack.ProvidePlugin({
      OpenAPISnippets: "./index.js", // Exposer OpenAPISnippets en tant que variable globale
      process: "process/browser",
      Buffer: ["buffer", "Buffer"],
    }),
  ],
  // mode: 'production', // Mode production pour minifier le code
  devtool: "source-map",
};

const minifiedConfig = {
  ...commonConfig,
  output: {
    filename: "sctgdesk-server.min.js", // Nom du fichier minifié
    path: path.resolve(__dirname, "dist"), // Répertoire de sortie
  },
  optimization: {
    usedExports: true, // <- remove unused function
  },
  mode: "production", // Mode production pour minifier le code
};

const nonMinifiedConfig = {
  ...commonConfig,
  output: {
    filename: "sctgdesk-server.js", // Nom du fichier non minifié
    path: path.resolve(__dirname, "dist"), // Répertoire de sortie
  },
  mode: "development", // Mode développement pour ne pas minifier le code
  optimization: {
    minimize: false, // Désactiver la minification
  },
};

module.exports = [minifiedConfig, nonMinifiedConfig];
