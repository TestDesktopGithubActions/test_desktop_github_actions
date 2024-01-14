<script lang="ts" setup>
    const emit = defineEmits<{
        (event: 'end'): void;
    }>();

    const seconds = ref(6);

    function startCountdown() {
        const timer = setInterval(() => {
            if (seconds.value > 0) {
                seconds.value--;
            } else {
                clearInterval(timer);
                // 倒计时结束后执行你想要的操作
                emit('end');
            }
        }, 1000); // 每秒执行一次

        onBeforeUnmount(() => {
            clearInterval(timer);
        });
    }

    watchEffect(() => {
        startCountdown();
    });
</script>

<template>
    <div text="64px white" font-bold>
        {{ seconds }}
    </div>
</template>
