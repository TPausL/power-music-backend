const Path = require("path");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const nodeExternals = require("webpack-node-externals");

module.exports = {
  externals: [nodeExternals()],
  target: "node",
  entry: {
    index: Path.resolve(__dirname, "../src/index.ts"),
  },
  output: {
    path: Path.join(__dirname, "../dist"),
    filename: "[name].js",
  },
  optimization: {
    splitChunks: {
      chunks: "all",
      name: false,
    },
  },
  plugins: [new CleanWebpackPlugin()],
  resolve: {
    alias: {
      "~": Path.resolve(__dirname, "../src"),
    },
      extensions: ['.js',  '.json', '.ts'],
      modules: [Path.join(__dirname, '..', 'src'), 'node_modules'],
  },
  module: {
    rules: [
      {
        test: /\.mjs$/,
        include: /node_modules/,
        type: "javascript/auto",
      },
      {
        test: /\.(ico|jpg|jpeg|png|gif|eot|otf|webp|svg|ttf|woff|woff2)(\?.*)?$/,
        use: {
          loader: "file-loader",
          options: {
            name: "[path][name].[ext]",
          },
        },
      },
    ],
  },
};
