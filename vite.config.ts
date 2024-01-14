import { resolve } from 'node:path';
import process from 'node:process';

import { defineConfig } from 'vite';
import Vue from '@vitejs/plugin-vue';
import UnoCSS from 'unocss/vite';
import VueRouter from 'unplugin-vue-router/vite';
import { VueRouterAutoImports } from 'unplugin-vue-router';
import Layouts from 'vite-plugin-vue-layouts';
import AutoImport from 'unplugin-auto-import/vite';
import Components from 'unplugin-vue-components/vite';
import VueI18n from '@intlify/unplugin-vue-i18n/vite';

// https://vitejs.dev/config/
export default defineConfig({
    resolve: {
        alias: {
            '~/': `${resolve(__dirname, 'src')}/`,
        },
    },
    plugins: [
        // https://github.com/posva/unplugin-vue-router
        // ⚠️ VueRouter() must be placed before Vue()
        VueRouter(),

        Vue(),

        // https://github.com/JohnCampionJr/vite-plugin-vue-layouts
        Layouts(),

        // https://github.com/unplugin/unplugin-auto-import
        AutoImport({
            imports: [
                VueRouterAutoImports,
                'vue',
                'vue-i18n',
                '@vueuse/head',
                '@vueuse/core',
                {
                    '@tauri-apps/api/tauri': ['invoke', 'tauri'],
                    '@tauri-apps/api/shell': ['open'],
                    '@tauri-apps/api/window': ['appWindow', 'setTitle'],
                    '@tauri-apps/api/app': ['getVersion'],
                },
                {
                    from: '~/types',
                    imports: ['UserModule', 'SharpFormField', 'RouteNode', 'ModalElement'],
                    type: true,
                },
            ],
            dirs: [
                './src/utils',
                './src/composables',
                './src/stores',
            ],
            // Filepath to generate corresponding .d.ts file.
            // Defaults to './auto-imports.d.ts' when `typescript` is installed locally.
            // Set `false` to disable.
            dts: './auto-imports.d.ts',

            // Generate corresponding .eslintrc-auto-import.json file.
            // eslint globals Docs - https://eslint.org/docs/user-guide/configuring/language-options#specifying-globals
            eslintrc: {
                enabled: true, // Default `false`
                filepath: './.eslintrc-auto-import.json', // Default `./.eslintrc-auto-import.json`
                globalsPropValue: true, // Default `true`, (true | false | 'readonly' | 'readable' | 'writable' | 'writeable')
            },
            vueTemplate: true,
        }),

        // https://github.com/unplugin/unplugin-vue-components
        Components({
            dts: true, // enabled by default if `typescript` is installed

            // Allow subdirectories as namespace prefix for components.
            directoryAsNamespace: true,
        }),

        // https://github.com/intlify/bundle-tools/tree/main/packages/unplugin-vue-i18n
        VueI18n({
            runtimeOnly: true,
            compositionOnly: true,
            fullInstall: true,
            strictMessage: false,
            include: [resolve(__dirname, 'locales/**')],
        }),

        // https://github.com/antfu/unocss
        // see uno.config.ts for config
        UnoCSS(),
    ],

    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    //
    // 1. prevent vite from obscuring rust errors
    clearScreen: false,
    // 2. tauri expects a fixed port, fail if that port is not available
    server: {
        port: 1420,
        strictPort: true,
    },
    // 3. to make use of `TAURI_DEBUG` and other env variables
    // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
    envPrefix: ['VITE_', 'TAURI_'],
    build: {
        // Tauri uses Chromium on Windows and WebKit on macOS and Linux
        target: process.env.TAURI_PLATFORM === 'windows' ? 'chrome105' : 'safari13',
        // don't minify for debug builds
        minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
        // 为调试构建生成源代码映射 (sourcemap)
        sourcemap: !!process.env.TAURI_DEBUG,
    },
});
