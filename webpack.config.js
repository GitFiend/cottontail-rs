const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

// const mode = 'production'
const mode = 'development'

module.exports = (env, argv) => {
  const mode = argv.mode ?? 'development'

  return {
    entry: './index.js',
    output: {
      path: path.resolve(__dirname, 'dist'),
      filename: 'index.js',
    },
    watchOptions: {
      aggregateTimeout: 200,
      poll: 200,
    },
    plugins: [
      new HtmlWebpackPlugin({
        template: 'index.html',
      }),
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, '.'),
        forceMode: mode,
      }),
    ],
    mode,
    experiments: {
      asyncWebAssembly: true,
    },
  }
}
