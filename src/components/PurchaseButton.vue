<script setup lang="ts">
    import { open } from '@tauri-apps/api/shell';

    const accountInfoState = useAccountInfoState();
    const loginFormValuesState = useStorage('vueuse-login-form-values', { email: '', password: '' });

    const { result: loginTemporaryResult, invoke: loginTemporaryInvoke, invoking } = useInvoke<LoginTemporaryCommand>('login_temporary');

    function formatToLocalTime(isoString: string): string {
        const utcDate = new Date(isoString); // 将传入的 ISO 时间字符串转换为 Date 对象
        const year = utcDate.getFullYear(); // 获取年份
        const month = (utcDate.getMonth() + 1).toString().padStart(2, '0'); // 获取月份（加1是因为月份从0开始，padStart用于补齐两位）
        const day = utcDate.getDate().toString().padStart(2, '0'); // 获取日期（padStart用于补齐两位）

        const formattedDate = `${year}-${month}-${day}`;
        return formattedDate;
    }

    function onClick() {
        loginTemporaryInvoke({
            email: loginFormValuesState.value.email,
            passwd: loginFormValuesState.value.password,
            proof: generateRandomString(10),
        });
    }

    watch(loginTemporaryResult, (result) => {
        if (result) {
            open(loginTemporaryResult.value.official_website);
        }
    });
</script>

<template>
    <div
        w-300px rd-18px flex items-center gap-10px p-10px
        class=" bg-#3C424E fixed bottom-20px left-1/2 ml--150px cursor-pointer"
        @click="onClick"
    >
        <span v-if="invoking" class="loading loading-dots loading-md" />
        <div i-sr:package class="!w-36px !h-36px" />
        <span text="14px white">
            {{ accountInfoState.is_card === 1 ? $t('index.renewNow', [accountInfoState.surplus_day, formatToLocalTime(accountInfoState.end_at!)]) : $t('index.renewalTips') }}
        </span>
    </div>
</template>
