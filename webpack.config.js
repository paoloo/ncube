/* eslint @typescript-eslint/no-var-requires: off */
const path = require("path");
const HtmlWebPackPlugin = require("html-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

const isProd = process.env.NODE_ENV === "production";

module.exports = {
  entry: "./src/ts/client.tsx",

  output: {
    path: path.resolve(__dirname, "target/webpack"),
    filename: "app.js",
    // FIXME: ncubed needs to serve hash based assets first
    // filename: "[name].[hash:8].js",
  },

  devtool: "source-map",

  devServer: {
    historyApiFallback: true,
  },

  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: [
          {
            loader: "babel-loader",
          },
        ],
        exclude: /node_modules/,
      },
      {
        test: /\.html$/,
        use: [
          {
            loader: "html-loader",
          },
        ],
      },
      {
        test: /\.css$/,
        use: [MiniCssExtractPlugin.loader, "css-loader", "postcss-loader"],
      },
      {
        test: /\.(jpe?g|png|gif|svg|woff|woff2|eot|ttf)$/i,
        use: [
          {
            loader: "url-loader",
            options: {
              limit: 30000,
            },
          },
        ],
      },
    ],
  },

  resolve: {
    extensions: [".ts", ".tsx", ".json", ".js", ".jsx"],
  },

  plugins: [
    new HtmlWebPackPlugin({
      template: path.resolve(__dirname, "resources/public/index.html"),
      filename: "./index.html",
      meta: isProd
        ? {
            "Content-Security-Policy": {
              "http-equiv": "Content-Security-Policy",
              content:
                "default-src http://127.0.0.1:40666 'unsafe-eval' 'unsafe-inline'; img-src 'self' data:;",
            },
            viewport: "width=device-width, initial-scale=1, shrink-to-fit=no",
          }
        : {},
    }),
    new MiniCssExtractPlugin({
      filename: "styles.css",
      chunkFilename: "styles.css",
      // FIXME: ncubed needs to serve hash based assets first
      // filename: "[name].[hash:8].css",
      // chunkFilename: "[name].[chunkhash:8].css",
    }),
  ],
};
