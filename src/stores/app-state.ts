export const useAppState = createGlobalState(
    () => {
        const isInitialized = ref(false);

        return {
            isInitialized,
        };
    },
);
