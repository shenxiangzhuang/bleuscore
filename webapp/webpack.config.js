const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
    mode: 'development',
    entry: './app.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'bundle.js',
        clean: true,
    },
    devServer: {
        static: {
            directory: path.join(__dirname),
        },
        port: 8080,
        open: true,
        hot: true,
        devMiddleware: {
            writeToDisk: false,
        },
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: './index.html',
            filename: 'index.html',
        }),
    ],
    experiments: {
        topLevelAwait: true,
        // asyncWebAssembly: true,
    },
}; 