<template>
    <div flex-col items-center pt-60px gap-80px>
        <div class="dark:i-sr-logo-dark light:i-sr-logo-light !w-174px !h-38px" />
        <div w-full flex-1 flex-col gap-16px px-40px>
            <router-link to="/auth/login" w-full class="btn btn-primary flex">
                <div class="!w-20px !-h-20px" i-sr-login-field-dark />
                <span>{{ $t('auth.login') }}</span>
            </router-link>
            <router-link to="/auth/signup" class="btn btn-secondary flex">
                <div class="!w-20px !-h-20px" i-sr-signup-field-light dark:i-sr-signup-field-dark />
                <span>{{ $t('auth.signup') }}</span>
            </router-link>
        </div>
        <a class="link link-hover text-12px">{{ $t('auth.forgotPassword') }}</a>
    </div>
</template>
