<script lang="ts" setup>
    import { loadLanguageAsync } from '~/modules/i18n';

    const { locale, availableLocales } = useI18n();
    const { invoke } = useInvoke<SetLanguageCommand>('set_language');

    const languagesNameMap: Recordable = {
        zh_Hans: '简体中文',
        zh_Hant: '繁體中文',
        en: 'English',
        ka: 'ქართული',
        ru: 'Русский',
        fr: 'français',
        pt: 'Português',
        es: 'Español',
        de: 'Deutsch',
        ja: '日本語',
        ko: '한국어',
    };

    function changeLanguage(event: Event) {
        const lang = (event.target as HTMLSelectElement).value;
        locale.value = lang;
        localStorage.setItem('Lang', lang);
        loadLanguageAsync(lang);
        invoke({
            language: lang,
        });
    };
</script>

<template>
    <div class="form-control w-full">
        <label class="label">
            <span class="label-text">{{ $t('preference.selectLanguage') }}</span>
        </label>
        <select
            class="select select-bordered focus:outline-none"
            :value="locale"
            @change="changeLanguage"
        >
            <option
                v-for="language in availableLocales"
                :key="language"
                :value="language"
            >
                {{ languagesNameMap[language] }}
            </option>
        </select>
    </div>
</template>
