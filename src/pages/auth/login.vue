<script lang="ts" setup>
    import {
        email,
        helpers,
        required,
    } from '@vuelidate/validators';
    import { type SharpFormField } from '~/types';

    const { t } = useI18n();
    const router = useRouter();
    const loginFormValuesState = useStorage('vueuse-login-form-values', { email: '', password: '' });

    const accountInfoState = useAccountInfoState();
    const { result, invoke, invoking } = useInvoke<LoginCommand>('login');

    const formFields = ref<SharpFormField[]>([
        {
            name: 'email',
            label: 'Email',
            type: 'email',
            value: loginFormValuesState.value.email,
            placeholder: t('signup.emailPlaceholder'),
            rules: {
                email: helpers.withMessage(
                    t('signup.emailInvalid'),
                    email,
                ),
                required: helpers.withMessage(
                    t('signup.emailPlaceholder'),
                    required,
                ),
            },
        },
        {
            name: 'password',
            label: 'Password',
            type: 'password',
            value: loginFormValuesState.value.password,
            placeholder: t('signup.passwordPlaceholder'),
            rules: {
                required: helpers.withMessage(
                    t('signup.passwordPlaceholder'),
                    required,
                ),
            },
        },
    ]);

    async function handleSubmit(value: any) {
        loginFormValuesState.value = value;
        await invoke({
            email: value.email,
            passwd: value.password,
        });
    }

    watch(result, (value) => {
        if (value) {
            accountInfoState.value = {
                isAuthenticated: true,
                ...value,
            };
            router.push('/');
        }
    });
</script>

<template>
    <div>
        <div v-show="!invoking" flex-col items-center pt-60px gap-10px px-40px>
            <div class="dark:i-sr-logo-dark light:i-sr-logo-light !w-174px !h-38px" />
            <sharp-form
                w-full
                md:px-10px mt-32px
                :fields="formFields"
                :submit-text="$t('login.button')"
                :loading="invoking"
                @submit="handleSubmit"
            />
            <!--
            <a class="link link-hover text-12px">{{ $t('auth.forgotPassword') }}</a>
            <i18n-t
                text="#3C424E 12px md:14px"
                lh-24px font-400 text-center mt-20px
                keypath="login.hint"
                tag="p"
                for="urlText"
                scope="global"
            >
                <router-link to="/auth/signup" c-primary>
                    {{ $t('login.urlText') }}
                </router-link>
            </i18n-t>
            -->
        </div>
        <div v-if="invoking" mt-140px flex-col items-center gap-20px>
            <span class="loading loading-dots loading-lg" />
            <span text="16px dark:white light:#3C424E" font-400>{{ $t('login.loadingText') }}</span>
        </div>
    </div>
</template>
