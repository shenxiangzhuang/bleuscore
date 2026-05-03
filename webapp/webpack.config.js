const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

const getrandomWasmBackendFlag = '--cfg getrandom_backend="wasm_js"';
if (!process.env.RUSTFLAGS?.includes('getrandom_backend="wasm_js"')) {
    process.env.RUSTFLAGS = [process.env.RUSTFLAGS, getrandomWasmBackendFlag]
        .filter(Boolean)
        .join(' ');
}

module.exports = (env, argv) => {
    const isProduction = argv.mode === 'production';
    
    return {
        mode: isProduction ? 'production' : 'development',
        target: ['web', 'es2020'],
        entry: './app.js',
        output: {
            path: path.resolve(__dirname, 'dist'),
            filename: isProduction ? '[name].[contenthash].js' : 'bundle.js',
            clean: true,
            publicPath: isProduction ? '/bleuscore/' : '/',
        },
        resolve: {
            alias: {
                'bleuscore-js': path.resolve(__dirname, '../bindings/js/pkg'),
            },
        },
        devServer: {
            client: {
                overlay: {
                    errors: true,
                    warnings: false,
                },
            },
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
            new WasmPackPlugin({
                crateDirectory: path.resolve(__dirname, '../bindings/js'),
            }),
            new HtmlWebpackPlugin({
                template: './index.html',
                filename: 'index.html',
            }),
        ],
        experiments: {
            topLevelAwait: true,
            asyncWebAssembly: true,
        },
        optimization: {
            splitChunks: isProduction ? {
                chunks: 'all',
            } : false,
        },
    };
}; 