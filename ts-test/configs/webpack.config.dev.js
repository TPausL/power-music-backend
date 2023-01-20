const Path = require("path");
const { merge } = require("webpack-merge");
const RunNodeWebpackPlugin = require("run-node-webpack-plugin");
const ESLintPlugin = require('eslint-webpack-plugin');

const common = require("./webpack.common.js");

module.exports = merge(common, {
  mode: "development",
  devtool: "eval-cheap-source-map",
  output: {
    chunkFilename: "[name].chunk.js",
  },
  plugins: [new RunNodeWebpackPlugin(), new ESLintPlugin({extensions: ['js', 'ts']})],
  module: {
    rules: [
      {
        test: /\.(js|ts)$/,
        exclude: /node_modules/,
        loader: "babel-loader",
      },
    ],
  },
});
