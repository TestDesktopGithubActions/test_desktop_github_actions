interface RouteLineState {
    isConnected: boolean;
    node: RouteNode | null;
}

export const useRouteLineState = createGlobalState(
    () => useStorage<RouteLineState>('vueuse-local-storage-route-line', {
        isConnected: false,
        node: null,
    }),
);
