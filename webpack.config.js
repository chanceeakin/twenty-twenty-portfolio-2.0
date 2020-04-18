const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");

const distPath = path.resolve(__dirname, "dist");
module.exports = (env, argv) => {
  return {
    devServer: {
      contentBase: distPath,
      compress: argv.mode === "production",
      port: 8000,
      historyApiFallback: true,
    },
    entry: ["./bootstrap.js", "./src/styles/index.css"],
    output: {
      path: distPath,
      filename: "todomvc.js",
      webassemblyModuleFilename: "todomvc.wasm",
    },
    module: {
      rules: [
        {
          test: /\.css$/,
          use: [MiniCssExtractPlugin.loader, "css-loader", "postcss-loader"],
        },
      ],
    },
    plugins: [
      new CopyWebpackPlugin([{ from: "./static", to: distPath }]),
      new MiniCssExtractPlugin({
        filename: "index.css",
        chunkFilename: "index.css",
      }),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      }),
    ],
    watch: argv.mode !== "production",
  };
};
