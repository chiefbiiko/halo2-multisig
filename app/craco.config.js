const webpack = require('webpack')

// required even for "prod" builds bc of some nasty nested js bigint math
// error that occurs when building with webpack prod but not dev config
process.env.NODE_ENV = 'development'

module.exports = {
  webpack: {
    plugins: {
      add: [
        new webpack.ProvidePlugin({
          Buffer: ['buffer', 'Buffer']
        }),
        new webpack.ProvidePlugin({
          process: 'process/browser.js'
        })
      ]
    }
  }
}