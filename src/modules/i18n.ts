import { type Locale, createI18n } from 'vue-i18n';

/*
 * All i18n resources specified in the plugin `include` option can be loaded
 * at once using the import syntax
 */
import messages from '@intlify/unplugin-vue-i18n/messages';

import { type UserModule } from '~/types';

// Import i18n resources
const i18n = createI18n({
    globalInjection: true,
    legacy: false,
    locale: '',
    messages,
    fallbackLocale: 'en',
});

// 获取浏览器语言设置
const userLanguage = navigator.language;

// 将语言值规范化为小写，并截取前两个字符
let languageCode = userLanguage.toLowerCase().slice(0, 2);

// 如果语言值是中文，检查是否有区域代码（例如，zh_Hans 或 zh_Hant）
if (languageCode === 'zh') {
    // 检查是否为繁体中文
    if (userLanguage.toLowerCase() === 'zh_hant' || userLanguage.toLowerCase() === 'zh_hant_tw') {
        // 繁体中文
        languageCode = 'zh_Hant';
    } else {
        // 简体中文
        languageCode = 'zh_Hans';
    }
}

const historyLang = localStorage.getItem('Lang');
const localLang = historyLang || languageCode;
const loadedLanguages: string[] = [];

function setI18nLanguage(lang: Locale) {
    i18n.global.locale.value = lang as any;
    if (typeof document !== 'undefined') {
        document.querySelector('html')?.setAttribute('lang', lang);
    };
    return lang;
}

export async function loadLanguageAsync(lang: string): Promise<Locale> {
    // If the same language
    if (i18n.global.locale.value === lang) {
        return setI18nLanguage(lang);
    };

    // If the language was already loaded
    if (loadedLanguages.includes(lang)) {
        return setI18nLanguage(lang);
    };

    // If the language hasn't been loaded yet
    i18n.global.setLocaleMessage(lang, messages![lang]);
    loadedLanguages.push(lang);
    return setI18nLanguage(lang);
}

export const install: UserModule = (app) => {
    app.use(i18n);
    loadLanguageAsync(localLang ?? 'en');
};
