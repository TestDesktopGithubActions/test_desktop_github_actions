<script setup lang="ts">
    import { listen } from '@tauri-apps/api/event';

    useTheme();
    const router = useRouter();
    const { locale } = useI18n();
    const appState = useAppState();
    const routeLineState = useRouteLineState();
    const accountInfoState = useAccountInfoState();
    const { showModal } = useModalState();
    const { invoke: setLanguage } = useInvoke<SetLanguageCommand>('set_language');

    let unlistenUpdateToken: () => void;

    onMounted(async () => {
        const splashscreenResponse = JSON.parse(await invoke<string>('splashscreen'));
        if (splashscreenResponse.code !== 200) {
            return showModal(splashscreenResponse.message ?? 'unknown error');
        }
        await setLanguage({
            language: locale.value,
        });
        appState.isInitialized.value = true;
        router.push('/');
        document.getElementById('splashscreen')?.remove();
        routeLineState.value.isConnected = false;

        unlistenUpdateToken = await listen<{ data: string }>('UpdateToken', (event) => {
            console.warn('UpdateTokenEvent', event);
            accountInfoState.value.token = event.payload.data;
        });
    });

    onUnmounted(() => {
        unlistenUpdateToken && unlistenUpdateToken();
    });
</script>

<template>
    <div id="splashscreen" />
    <router-view />
    <sharp-modal />
</template>

<style>
    #splashscreen {
        height: 100vh;
        width: 100vw;
        border-radius: 24px;
        overflow: hidden;
        background: url('splashscreen.png'), linear-gradient(180deg, #FFF 0%, #FFF 100%);
        background-repeat: no-repeat;
        background-position: center;
        background-size: cover;
        position: absolute;
        top: 0;
        left: 0;
        z-index: 9;
        pointer-events: none;
    }
</style>
