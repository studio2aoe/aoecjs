const pkgver = require('./package.json').version

const path = require('path');
module.exports = {
    entry: './src/index.ts',
    devtool: 'inline-source-map',
    module: {
        rules: [{
            test: /\.ts$/,
            use: 'ts-loader',
            exclude: /node_modules/
        }]
    },
    resolve: {
        extensions: ['.ts', '.js']
    },
    output: {
        filename: `aoecjs.${pkgver}.js`,
        path: path.resolve(__dirname, 'dist')
    }
}