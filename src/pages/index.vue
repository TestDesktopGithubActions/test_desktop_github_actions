<script lang="ts" setup>
    import { listen } from '@tauri-apps/api/event';

    const { t } = useI18n();
    const accountInfoState = useAccountInfoState();
    const routeLineState = useRouteLineState();
    const { showModal } = useModalState();

    const routeLine: Ref<RouteNode | null> = ref(routeLineState.value.node);
    const countdownEnd = ref(false);
    const { result, invoking, invoke: connectInvoke } = useInvoke<NodeStartCommand>('node_start');
    const { result: disconnectResult, invoking: disconnectInvoking, invoke: disconnectInvoke } = useInvoke<NodeEndCommand>('node_end', { noCatchError: true });

    let unlistenDisconnected: () => void;

    const stateText = computed(() => {
        if (invoking.value) {
            return t('index.connecting');
        }
        if (disconnectInvoking.value) {
            return t('index.disconnecting');
        }
        if (routeLineState.value.isConnected) {
            return t('index.connect');
        }
        if (!routeLineState.value.isConnected) {
            return t('index.disconnect');
        }
    });

    async function onRouteLineSelected(node: RouteNode) {
        if (!!routeLine.value && (node.countryCode === routeLine.value.countryCode)) {
            return;
        }
        if (routeLineState.value.isConnected) {
            await disconnect();
            routeLine.value = node;
            routeLineState.value.node = node;
            await connect();
        } else {
            routeLine.value = node;
            routeLineState.value.node = node;
        }
    }

    function onRouteLineUpdate(node: any) {
        routeLine.value = node;
        routeLineState.value.node = node;
    }

    async function connect() {
        if (routeLine.value && !invoking.value) {
            const connectResponse = await connectInvoke({
                guid: routeLine.value.nodes[routeLine.value.minDelayNodeIndex].guid,
                token: accountInfoState.value.token,
            }).catch(() => {
                countdownEnd.value = true;
                routeLineState.value.isConnected = false;
            });

            if (!connectResponse) {
                countdownEnd.value = true;
                routeLineState.value.isConnected = false;
            }
        }
    }

    async function disconnect() {
        if (routeLineState.value.isConnected && !invoking.value) {
            routeLineState.value.isConnected = false;
            await disconnectInvoke({
                guid: routeLine.value!.nodes[routeLine.value!.minDelayNodeIndex].guid,
                token: accountInfoState.value.token,
            });
        }
    }

    watch([result, invoking], ([value]) => {
        if (value) {
            routeLineState.value.isConnected = true;
        } else {
            routeLineState.value.isConnected = false;
        }
        countdownEnd.value = true;
    });

    watch([disconnectResult, disconnectInvoking], ([value]) => {
        if (value) {
            routeLineState.value.isConnected = false;
        }
    });

    onMounted(async () => {
        unlistenDisconnected = await listen<{ data: Recordable<string> }>('Disconnected', (event) => {
            console.warn('DisconnectedEvent', event);
            routeLineState.value.isConnected = false;
            if (event?.payload.data.code === '9001') {
                showModal(t('index.packageExpired'));
                return;
            }
            if (event?.payload.data.code === '9002') {
                showModal(t('index.forceDisconnect'));
                return;
            }
            connect();
        });
    });

    onUnmounted(() => {
        unlistenDisconnected && unlistenDisconnected();
    });
</script>

<template>
    <div px-20px pt-10px flex-col items-center>
        <route-line-selector
            :node="routeLine"
            @selected="onRouteLineSelected"
            @update="onRouteLineUpdate"
        />

        <button
            flex-col items-center justify-center mt-45px relative w-176px h-176px select-none
            type="button"
            @click="routeLineState.isConnected ? disconnect() : connect()"
        >
            <svg v-if="invoking || disconnectInvoking" class="absolute z--1 animate-pulse-alt" width="225" height="225" viewBox="0 0 225 225" fill="none">
                <circle opacity="0.1" cx="112.5" cy="112.5" r="112.5" fill="url(#paint0_linear_1378_2042)" />
                <defs>
                    <linearGradient id="paint0_linear_1378_2042" x1="112.5" y1="0" x2="112.5" y2="225" gradientUnits="userSpaceOnUse">
                        <stop stop-color="#97A1B2" />
                        <stop offset="0.9999" stop-color="#3C424E" />
                    </linearGradient>
                </defs>
            </svg>
            <svg class="absolute z--1" width="190" height="190" viewBox="0 0 190 190" fill="none">
                <g opacity="0.3">
                    <circle cx="95" cy="95" r="95" fill="url(#paint1_linear_1402_9399)" />
                </g>
                <defs>
                    <linearGradient id="paint1_linear_1402_9399" x1="95" y1="0" x2="95" y2="190" gradientUnits="userSpaceOnUse">
                        <stop stop-color="#97A1B2" />
                        <stop offset="0.9999" stop-color="#3C424E" />
                    </linearGradient>
                </defs>
            </svg>
            <svg class="absolute z-0" width="150" height="150" viewBox="0 0 150 150" fill="none">
                <circle cx="75" cy="75" r="75" fill="url(#paint0_linear_1366_843)" />
                <defs>
                    <linearGradient id="paint0_linear_1366_843" x1="75" y1="0" x2="75" y2="150" gradientUnits="userSpaceOnUse">
                        <stop stop-color="#97A1B2" />
                        <stop offset="0.9999" stop-color="#3C424E" />
                    </linearGradient>
                </defs>
            </svg>
            <div relative z-1>
                <template v-if="!invoking && !disconnectInvoking">
                    <div
                        v-if="!routeLineState.isConnected"
                        i-sr:lightning
                        class="!w-64px !h-66px"
                    />
                    <div
                        v-else
                        i-sr:power
                        class="!w-64px !h-66px"
                    />
                </template>
                <template v-if="invoking">
                    <countdown-timer
                        v-if="!countdownEnd"
                        @end="() => {
                            countdownEnd = true;
                        }"
                    />
                    <span v-else text="14px white">{{ $t('index.buttonTextConnecting') }}</span>
                </template>
                <template v-if="disconnectInvoking">
                    <span text="14px white">{{ $t('index.buttonTextDisconnecting') }}</span>
                </template>
            </div>
        </button>
        <span mt-40px text="14px dark:#C9C9C9">{{ $t('index.state') }}{{ stateText }}</span>
        <!-- <span v-if="!invoking && !disconnectInvoking" mt-16px text="14px text-#849199">
            {{ !routeLineState.isConnected ? '' : $t('index.disconnectHint') }}
        </span> -->
        <span v-if="routeLine?.defaultCountryCode" text="mt-10px 14px dark:#C9C9C9">
            {{ $t('index.defaultNodeCountry', [routeLine?.defaultCountryCode.toUpperCase()]) }}
        </span>

        <purchase-button />
    </div>
</template>

<route lang="json">
    {
        "meta": {
            "requiresAuth": true
        }
    }
</route>
