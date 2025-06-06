const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = (env, argv) => {
    const isProduction = argv.mode === 'production';
    
    return {
        mode: isProduction ? 'production' : 'development',
        entry: './app.js',
        output: {
            path: path.resolve(__dirname, 'dist'),
            filename: isProduction ? '[name].[contenthash].js' : 'bundle.js',
            clean: true,
            publicPath: isProduction ? '/bleuscore/' : '/',
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
        optimization: {
            splitChunks: isProduction ? {
                chunks: 'all',
            } : false,
        },
    };
}; 