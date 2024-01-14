<script lang="ts" setup>
    import { disable, enable, isEnabled } from 'tauri-plugin-autostart-api';

    const isEnableAutoStart = ref(false);

    async function toggleAutoStart() {
        if (await isEnabled()) {
            disable();
        } else {
            enable();
        }
    }

    watchEffect(async () => {
        isEnableAutoStart.value = await isEnabled();
    });
</script>

<template>
    <div class="form-control w-full">
        <label class="cursor-pointer label gap-8px">
            <span class="label-text">{{ $t('preference.selfStart') }}</span>
            <input type="checkbox" class="toggle toggle-primary" :checked="isEnableAutoStart" @change="toggleAutoStart">
        </label>
    </div>
</template>
