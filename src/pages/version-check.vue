<script lang="ts" setup>
    const appVersion = ref('');

    onMounted(async () => {
        appVersion.value = await getVersion();
    });
</script>

<template>
    <div mt-120px flex-col items-center justify-center gap-20px>
        <div text="16px dark:#FFF">
            {{ $t('versionCheck.latest') }}
        </div>
        <div text="16px dark:#FFF">
            {{ $t('versionCheck.version') }}{{ appVersion }}
        </div>
        <button class="btn btn-primary w-280px h-44px mt-115px">
            {{ $t('versionCheck.update') }}
        </button>
    </div>
</template>

<route lang="json">
    {
        "meta": {
            "hasBack": true
        }
    }
</route>
