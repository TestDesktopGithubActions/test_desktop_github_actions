<template>
    <main wh-full>
        <layout-title-bar />
        <router-view v-slot="{ Component, route }">
            <transition name="fade" mode="out-in">
                <component :is="Component" :key="route" />
            </transition>
        </router-view>
    </main>
</template>

<style lang="scss">
.fade-enter-from, .fade-leave-to {
    opacity: 0;
}

.fade-enter-to, .fade-leave-from {
    opacity: 1;
}

.fade-enter-active,
.fade-leave-active {
    transition: opacity .2s;
}
</style>
