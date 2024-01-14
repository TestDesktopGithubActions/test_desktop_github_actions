<script lang="ts" setup>
    import { exit } from '@tauri-apps/api/process';
    import { router } from '~/modules/router';

    const accountInfoState = useAccountInfoState();
    const routeLineState = useRouteLineState();
    const { invoke: logout } = useInvoke<LogoutCommand>('logout');

    const logoutModalElementRef = ref<ModalElement | null>(null);
    const menuDetailsElementRef = ref<HTMLDetailsElement | null>(null);
    const toastVisible = ref(false);

    onClickOutside(menuDetailsElementRef, () => {
        menuDetailsElementRef.value?.removeAttribute('open');
    });

    const { invoke: disconnectInvoke } = useInvoke<NodeEndCommand>('node_end');
    const { invoke: uploadLogInvoke } = useInvoke<UploadLogCommand>('upload_log');

    const menus = ref([
        /* {
            i18nKey: 'purchasePackage',
            action: () => {

            },
        },
        {
            i18nKey: 'visitWebsite',
            action: () => {

            },
        },
        {
            i18nKey: 'contactCustomerService',
            action: () => {

            },
        }, */
        {
            i18nKey: 'preferences',
            action: () => {
                router.push('/preferences');
            },
        },
        {
            i18nKey: 'uploadLog',
            action: async () => {
                await uploadLogInvoke({
                    email: accountInfoState.value.email,
                    token: accountInfoState.value.token,
                });
                toastVisible.value = true;
            },
            isShow: () => {
                return accountInfoState.value?.isAuthenticated;
            },
        },
        {
            i18nKey: 'versionCheck',
            action: () => {
                router.push('/version-check');
            },
        },
        {
            i18nKey: 'logout',
            action: async () => {
                logoutModalElementRef.value?.showModal();
                if (routeLineState.value.isConnected) {
                    await disconnectInvoke({
                        guid: routeLineState.value.node!.nodes[routeLineState.value.node!.minDelayNodeIndex].guid,
                        token: accountInfoState.value.token,
                    });
                }
                await logout({
                    token: accountInfoState.value.token,
                });
                accountInfoState.value = {
                    isAuthenticated: false,
                    token: '',
                    email: '',
                    exp: 0,
                    surplus_day: 0,
                    is_card: 0,
                };
                routeLineState.value = {
                    isConnected: false,
                    node: null,
                };
                logoutModalElementRef.value?.close();
                router?.replace('/auth/login');
            },
            isShow: () => {
                return accountInfoState.value?.isAuthenticated;
            },
        },
        {
            i18nKey: 'exit',
            action: async () => {
                await exit(1);
            },
            isShow: () => {
                return !accountInfoState.value?.isAuthenticated;
            },
        },
    ]);

    watch(toastVisible, (value) => {
        if (value) {
            setTimeout(() => {
                toastVisible.value = false;
            }, 2000);
        }
    });
</script>

<template>
    <details ref="menuDetailsElementRef" class="dropdown dropdown-bottom dropdown-end">
        <summary tabindex="0" class="inline-block !w-27px !h-22px cursor-pointer dark:i-sr-menu-dark light:i-sr-menu-light" />
        <ul tabindex="0" class="dropdown-content p-0 overflow-hidden z-[1] menu rd-8px shadow-md bg-base-100 w-max b-1px b-#E4E4E4 dark:b-#4D4D4D">
            <template v-for="menu in menus" :key="menu.i18nKey">
                <li
                    v-if="menu.isShow ? menu.isShow() : true"
                    class="text-#5D6574 dark:text-white text-10px font-400 p-4px px-10px text-center cursor-pointer hover:bg-primary hover:text-#FFF"
                    @click="() => {
                        menuDetailsElementRef?.removeAttribute('open');
                        menu?.action();
                    }"
                >
                    {{ $t(`menu.${menu.i18nKey}`) }}
                </li>
            </template>
        </ul>
    </details>

    <dialog ref="logoutModalElementRef" class="modal">
        <div class="f-c-c">
            <span class="loading loading-dots loading-lg" />
        </div>
    </dialog>

    <div v-show="toastVisible" class="toast toast-center toast-middle z-9">
        <div class="alert dark:bg-#3D424D dark:border-#3D424D">
            <span>{{ $t('menu.uploadLogSuccess') }}</span>
        </div>
    </div>
</template>
