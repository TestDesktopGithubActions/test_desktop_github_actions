<script lang="ts" setup>
    const router = useRouter();
    const route = useRoute();

    const hiddenLogo = computed(() => {
        return route.path.startsWith('/auth');
    });
</script>

<template>
    <header
        data-tauri-drag-region
        select-none
        p-20px pb-10px flex justify-between items-center
    >
        <div flex items-center gap-2px class="group">
            <button @click="appWindow.hide()">
                <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                    <circle cx="8" cy="8" r="8" fill="#ED6B5F" />
                    <path class="hidden group-hover:display-inherit" d="M4.80005 4.80005L11.2 11.2" stroke="#69110A" stroke-width="2" stroke-linecap="round" />
                    <path class="hidden group-hover:display-inherit" d="M11.2 4.80005L4.79995 11.2" stroke="#69110A" stroke-width="2" stroke-linecap="round" />
                </svg>
            </button>

            <button @click="appWindow.minimize()">
                <svg width="32" height="16" viewBox="0 0 32 16" fill="none">
                    <circle cx="16" cy="8" r="8" fill="#F5BF50" />
                    <path class="hidden group-hover:display-inherit" d="M12 8H20" stroke="#8F581E" stroke-width="2" stroke-linecap="round" />
                </svg>
            </button>

            <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                <circle cx="8" cy="8" r="8" fill="#B1B1B1" />
            </svg>
        </div>

        <div v-if="!hiddenLogo" mr-auto ml-12px class="dark:i-sr-logo-dark light:i-sr-logo-light !w-134px !h-28px" />

        <layout-dropdown-menu />
    </header>
    <button
        v-if="route.meta.hasBack"
        i-carbon:chevron-left
        ml-16px
        class="!w-24px !h-24px"
        @click="router.back()"
    />
</template>
