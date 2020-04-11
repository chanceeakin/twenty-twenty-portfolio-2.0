const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");

const distPath = path.resolve(__dirname, "dist");
module.exports = (env, argv) => {
  console.log(distPath);
  return {
    devServer: {
      contentBase: distPath,
      compress: argv.mode === "production",
      port: 8000
    },
    entry: ["./bootstrap.js", "./styles/index.scss"],
    output: {
      path: distPath,
      filename: "todomvc.js",
      webassemblyModuleFilename: "todomvc.wasm"
    },
    module: {
      rules: [
        {
          test: /\.scss$/,
          use:
            argv.mode !== "production"
              ? ["style-loader", "css-loader", "sass-loader"]
              : [
                  {
                    loader: "file-loader",
                    options: {
                      name: "css/[name].css"
                    }
                  },
                  {
                    loader: "extract-loader"
                  },
                  {
                    loader: "css-loader?-url"
                  },
                  {
                    loader: "postcss-loader"
                  },
                  {
                    loader: "sass-loader"
                  }
                ]
        }
      ]
    },
    plugins: [
      new CopyWebpackPlugin([{ from: "./static", to: distPath }]),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript"
      })
    ],
    watch: argv.mode !== "production"
  };
};
