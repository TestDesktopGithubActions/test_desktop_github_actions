<script lang="ts" setup>
    import type { NodeData, RouteNode } from '~/types';

    const props = defineProps<{
        node?: RouteNode | null;
        disabled?: boolean;
    }>();

    const emit = defineEmits<{
        (event: 'selected', payload: RouteNode): void;
        (event: 'update', payload: RouteNode): void;
    }>();

    const accountInfoState = useAccountInfoState();
    const selectedRouteNode = ref<RouteNode | null>(props.node || null);
    const routeNodes = ref<RouteNode[]>([]);
    const loading = ref(false);
    const toastVisible = ref(false);

    const routesDetailsElementRef = ref<HTMLDetailsElement | null>(null);
    const defaultRouteNodeCountryCode = 'DEFAULT';

    onClickOutside(routesDetailsElementRef, () => {
        closeDetails();
    });

    const { result, error, invoking, invoke } = useInvoke<NodeListCommand>('node_list');
    const { echo } = useIFetch();

    function closeDetails() {
        routesDetailsElementRef.value?.removeAttribute('open');
    }

    function onSelected(node: RouteNode) {
        selectedRouteNode.value = node;
        emit('selected', node);
        closeDetails();
    }

    function requestNodes() {
        if (invoking.value || loading.value) {
            return;
        }
        routeNodes.value = [];
        invoke({
            token: accountInfoState.value.token,
        });
    }

    function updateDelay() {
        let minDelayRouteNode: RouteNode | null = null;

        routeNodes.value.forEach((routeNode) => {
            // 遍历 node.nodes 比较 delay，选出最小的 delay 元素
            routeNode.nodes.forEach((node: NodeData, index: number) => {
                echo(`${node.ip}:${node.node_port}`).then((res) => {
                    if (routeNode.delay === 0 || res.delay < routeNode.delay) {
                        routeNode.delay = res.delay;
                        routeNode.minDelayNodeIndex = index;
                        if (
                            selectedRouteNode.value?.countryCode !== defaultRouteNodeCountryCode
                            && selectedRouteNode.value?.countryCode === routeNode.countryCode
                        ) {
                            selectedRouteNode.value = routeNode;
                            emit('update', routeNode);
                        }
                    }
                    // 获取到最小 delay 的节点后，更新 minDelayRouteNode
                    if (minDelayRouteNode === null || minDelayRouteNode.delay === 0 || routeNode.delay < minDelayRouteNode.delay) {
                        minDelayRouteNode = {
                            ...routeNode,
                            countryCode: defaultRouteNodeCountryCode,
                            defaultCountryCode: routeNode.defaultCountryCode || routeNode.countryCode,
                        };
                        routeNodes.value[0] = minDelayRouteNode;
                        if (selectedRouteNode.value?.countryCode === defaultRouteNodeCountryCode) {
                            selectedRouteNode.value = minDelayRouteNode;
                            emit('update', minDelayRouteNode);
                        }
                    }
                });
            });
        });
    }

    function handleSummaryClick() {
        if (props.disabled) {
            toastVisible.value = true;
            routesDetailsElementRef.value?.setAttribute('open', '');
            return;
        }
        updateDelay();
    }

    watch(error, (value) => {
        if (value) {
            selectedRouteNode.value = null;
            routeNodes.value = [];
        }
    });

    watch(result, async (value) => {
        if (!value) {
            return;
        }

        loading.value = true;
        routeNodes.value = [
            {
                countryCode: defaultRouteNodeCountryCode,
                minDelayNodeIndex: 0,
                delay: 0,
                nodes: [],
            },
            ...Object.entries(value).map(([countryCode, nodes]) => {
                return {
                    countryCode,
                    minDelayNodeIndex: 0,
                    delay: 0,
                    nodes: nodes as NodeData[],
                };
            }),
        ];
        selectedRouteNode.value = routeNodes.value.find(node => node.countryCode === selectedRouteNode.value?.countryCode) || routeNodes.value[0];
        if (selectedRouteNode.value) {
            emit('update', selectedRouteNode.value);
        }
        loading.value = false;
    });

    watch(routeNodes, (value) => {
        if (!value) {
            return;
        }
        updateDelay();
    });

    watch(toastVisible, (value) => {
        if (value) {
            setTimeout(() => {
                toastVisible.value = false;
            }, 2000);
        }
    });

    onMounted(() => {
        requestNodes();
    });
</script>

<template>
    <details ref="routesDetailsElementRef" class="dropdown w-280px">
        <summary
            class="flex justify-between btn mb-6px px-6px w-full !h-40px rd-24px bg-secondary hover:bg-primary hover:text-white group"
            @click="handleSummaryClick"
        >
            <div v-if="selectedRouteNode?.countryCode !== defaultRouteNodeCountryCode" :class="`i-flag-${(selectedRouteNode?.countryCode ?? '').toLowerCase()}-1x1 !w-32px !h-32px rd-full`" />
            <div v-else class="i-carbon:3d-curve-auto-colon !w-32px !h-32px" />

            <div v-if="selectedRouteNode" class="flex-1 flex items-center justify-between dark:text-white">
                <span v-if="selectedRouteNode?.countryCode !== defaultRouteNodeCountryCode">
                    {{ `${selectedRouteNode?.countryCode.toUpperCase()} ${$t('index.route')}` }}
                </span>
                <span v-else>{{ $t('index.defaultLine') }}</span>
                <span v-if="(selectedRouteNode?.delay || 0) > 0">{{ selectedRouteNode?.delay ?? '' }}ms</span>
            </div>
            <div v-else>
                {{ $t('index.selectRoute') }}
            </div>

            <div i-carbon:chevron-down w-20px h-20px mr-12px />
        </summary>
        <ul class="dropdown-content w-full max-h-380px flex-nowrap flex-col gap-8px overflow-scroll shadow z-[1] bg-white dark:bg-#353940 rd-8px py-10px px-5px relative z-9">
            <li f-c-c mb-10px>
                <div text="14px #3C424E dark:white">
                    {{ $t('index.selectLine') }}
                </div>
                <button
                    absolute right-10px top-12px ml-auto i-carbon:restart
                    @click="requestNodes"
                />
            </li>
            <li
                v-if="invoking || loading"
                flex-col items-center justify-center
            >
                <span class="loading loading-spinner loading-md" />
                <span text="14px dark:white">{{ $t('index.lineLoading') }}</span>
            </li>
            <li
                v-else-if="!routeNodes.length"
                flex justify-center
            >
                <span text="14px dark:white">{{ $t('index.noRouteNodes') }}</span>
            </li>
            <li
                v-for="node in routeNodes"
                :key="node.countryCode"
                flex flex-row items-center gap-8px w-full rd-32px
                cursor-pointer px-6px py-5px
                border="1px solid #CFD9E3"
                @click="() => { onSelected(node) }"
            >
                <div v-if="node.countryCode !== defaultRouteNodeCountryCode" :class="`i-flag-${node.countryCode.toLowerCase()}-1x1 !w-32px !h-32px rd-full`" />
                <div v-else class="i-carbon:3d-curve-auto-colon !w-32px !h-32px" />
                <div class="!flex-col items-start gap-0 p-0">
                    <span v-if="node.countryCode !== defaultRouteNodeCountryCode" text="12px dark:#FFF">{{ node.countryCode.toUpperCase() }} {{ $t('index.route') }}</span>
                    <span v-else text="12px dark:#FFF">{{ $t('index.defaultLine') }}</span>
                </div>
                <div v-if="node.delay" ml-auto flex items-center gap-8px text="16px dark:#C9C9C9">
                    <div i-carbon:skill-level-intermediate :class="(node.delay || 0) > 200 ? 'c-red' : 'c-green' " />
                    <span text="10px dark:#FFF">{{ node.delay }}ms</span>
                    <input type="radio" class="radio checked:!bg-primary" :checked="!!selectedRouteNode && (selectedRouteNode.countryCode === node.countryCode)">
                </div>
            </li>
        </ul>
    </details>

    <div v-show="toastVisible" class="toast toast-center toast-middle z-9">
        <div class="alert dark:bg-#3D424D dark:border-#3D424D">
            <span>{{ $t('index.disconnectFirst') }}</span>
        </div>
    </div>
</template>
