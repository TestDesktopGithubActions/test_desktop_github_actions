<script lang="ts" setup>
    import {
        email,
        helpers,
        required,
        sameAs,
    } from '@vuelidate/validators';
    import { type SharpFormField } from '~/types';

    const { t } = useI18n();
    const router = useRouter();
    const { invoke: bindDeviceInvoke } = useInvoke<BindDeviceCommand>('bind_device');
    const { result, invoke, invoking } = useInvoke<RegisterCommand>('register');
    const { result: activatingResult, invoke: activatingInvoke, invoking: activatingInvoking } = useInvoke<ActivatingCommand>('activating');

    const formFields = ref<SharpFormField[]>([
        {
            name: 'email',
            label: 'Email',
            type: 'email',
            value: '',
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
            value: '',
            placeholder: t('signup.passwordPlaceholder'),
            mask: true,
            rules: {
                required: helpers.withMessage(
                    t('signup.passwordPlaceholder'),
                    required,
                ),
            },
        },
        {
            name: 'confirmPassword',
            label: 'Confirm Password',
            type: 'password',
            value: '',
            placeholder: t('signup.confirmPasswordPlaceholder'),
            mask: true,
            rules: {
                required: helpers.withMessage(
                    t('signup.confirmPasswordPlaceholder'),
                    required,
                ),
                sameAsField: helpers.withMessage(
                    t('signup.confirmPasswordInvalid'),
                    sameAs('password'),
                ),
            },
        },
    ]);
    const activateFormFields = ref<SharpFormField[]>([
        {
            name: 'accountCode',
            label: 'Account Code',
            type: 'text',
            value: '',
            placeholder: t('signup.activationCodePlaceholder'),
            rules: {
                required: helpers.withMessage(
                    t('signup.activationCodePlaceholder'),
                    required,
                ),
            },
        },
    ]);

    watch(activatingResult, (value) => {
        if (value) {
            console.warn('activatingResult', value);
            setTimeout(() => {
                router.push('/auth/login');
            }, 2000);
        }
    });

    async function handleSubmit(value: any) {
        await bindDeviceInvoke({});
        await invoke({
            email: value.email,
            passwd: value.password,
            repeatPassword: value.confirmPassword,
        });
    }

    async function handleActivateSubmit(value: any) {
        await activatingInvoke({
            accountCode: value.accountCode,
            code: result.value,
        });
    }
</script>

<template>
    <div flex-col items-center pt-60px gap-10px px-40px>
        <div class="dark:i-sr-logo-dark light:i-sr-logo-light !w-174px !h-38px" />
        <template v-if="!result">
            <sharp-form
                w-full
                md:px-10px mt-32px
                :fields="formFields"
                :submit-text="$t('signup.button')"
                :loading="invoking"
                @submit="handleSubmit"
            />
            <i18n-t
                text="#3C424E 12px md:14px"
                lh-24px font-400 text-center mt-20px
                keypath="signup.hint"
                tag="p"
                for="urlText"
                scope="global"
            >
                <router-link to="/auth/login" c-primary>
                    {{ $t('signup.urlText') }}
                </router-link>
            </i18n-t>
        </template>
        <template v-else>
            <span v-if="activatingResult">{{ $t('signup.activateSuccessMessage') }}</span>
            <span mt-30px text="12px">{{ $t('signup.activateMessage') }}</span>
            <sharp-form
                w-full
                md:px-10px
                :fields="activateFormFields"
                :submit-text="$t('signup.activateButton')"
                :loading="activatingInvoking"
                @submit="handleActivateSubmit"
            />
        </template>
    </div>
</template>
