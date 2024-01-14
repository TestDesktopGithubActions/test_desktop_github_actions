import {
    defineConfig,
    presetAttributify,
    presetIcons,
    presetTypography,
    presetUno,
    presetWebFonts,
    transformerDirectives,
    transformerVariantGroup,
} from 'unocss';
import { presetDaisy } from 'unocss-preset-daisy';
import themes from 'daisyui/src/theming/themes';
import { FileSystemIconLoader } from '@iconify/utils/lib/loader/node-loaders';

const flagIcons = [
    'i-flag-hk',
    'i-flag-kr',
    'i-flag-tw',
    'i-flag-us',
    'i-flag-jp',
    'i-flag-cn',
    'i-flag-fr',
    'i-flag-de',
    'i-flag-it',
    'i-flag-es',
    'i-flag-ru',
    'i-flag-in',
    'i-flag-id',
    'i-flag-th',
    'i-flag-vn',
    'i-flag-ph',
    'i-flag-sg',
].map(flag => [`${flag}-1x1`, `${flag}-4x3`]).flat();

export default defineConfig({
    theme: {
        colors: {
            primary: 'hsl(var(--p))',
        },
    },
    shortcuts: [
        ['wh-full', 'w-full h-full'],
        ['f-c-c', 'flex justify-center items-center'],
        ['flex-col', 'flex flex-col'],
    ],
    presets: [
        presetUno(),
        presetAttributify(),
        presetIcons({
            scale: 1.2,
            warn: true,
            collections: {
                carbon: () => import('@iconify-json/carbon/icons.json').then(i => i.default),
                flag: () => import('@iconify-json/flag/icons.json').then(i => i.default),

                // a helper to load icons from the file system
                // files under `./assets/icons` with `.svg` extension will be loaded as it's file name
                // you can also provide a transform callback to change each icon (optional)
                sr: FileSystemIconLoader(
                    './src/assets/icons',
                    svg => svg.replace(/#fff/, 'currentColor'),
                ),
            },
            customizations: {
                iconCustomizer(collection, _, props) {
                    if (collection === 'sr') {
                        props.width = '100%';
                        props.height = '100%';
                    }
                },
            },
        }),
        presetTypography(),
        presetWebFonts({
            provider: 'bunny',
            fonts: {
                rubik: 'Rubik',
                serif: ['DM Serif Display', 'Source Serif Pro'],
                mono: 'DM Mono',
                helvetica: 'Helvetica',
            },
        }),
        presetDaisy({
            themes: [
                {
                    light: {
                        ...themes['[data-theme=light]'],
                        'base-100': '#fff',
                        'primary': '#0966D4',
                        'primary-content': '#FFF',
                        'secondary': '#F4F6F9',
                        'secondary-content': '#5D6574',
                        'error': '#F86161',
                        '--btn-text-case': 'none',
                    },
                },
                {
                    dark: {
                        ...themes['[data-theme=dark]'],
                        'base-100': '#1A1D24',
                        'primary': '#0966D4',
                        'error': '#F86161',
                        'secondary': '#353940',
                        '--btn-text-case': 'none',
                    },
                },
            ],
        }),
    ],
    transformers: [
        transformerDirectives(),
        transformerVariantGroup(),
    ],
    safelist: [
        ...flagIcons,
    ],
});
