import { router } from '~/modules/router';

// invoke 返回的统一格式
interface BaseInvokeResponse<T> {
    code: number;
    message: string;
    result: T | null;
}

interface Options {
    noCatchError?: boolean;
}

export function useInvoke<T extends Command>(command: T['name'], options?: Options): {
    result: Ref<any | null>;
    error: Ref<string | null>;
    invoking: Ref<boolean>;
    invoke: <R = Recordable>(args: T['input']) => Promise<BaseInvokeResponse<R> | void>;
} {
    const { showModal } = useModalState();
    const accountInfoState = useAccountInfoState();

    const result = ref<any | null>(null);
    const error = ref<string | null>(null);
    const invoking = ref(false);

    async function invokeCommand<R = Recordable>(args: T['input']): Promise<BaseInvokeResponse<R> | void> {
        try {
            invoking.value = true;

            // 判断 token 过期时间是否小于 30 分钟，小于则刷新 token
            if (accountInfoState.value.isAuthenticated && ((accountInfoState.value.exp * 1000 - Date.now()) < 30 * 60 * 1000)) {
                await refreshToken();
            }

            if (import.meta.env.DEV) {
                console.log('REQUEST', command, args);
            }

            const originalResponse = await invoke<string>(command, args);
            const response: BaseInvokeResponse<R> = JSON.parse(originalResponse);
            if (import.meta.env.DEV) {
                console.log('RESPONSE', response);
            }

            // 不捕获错误，直接返回
            if (options?.noCatchError) {
                invoking.value = false;
                return response;
            }

            // token 失效
            if (response.code === 2001) {
                invoking.value = false;
                router.replace('/auth/login');
                return response;
            }

            // token 过期，刷新 token
            if (response.code === 2002) {
                await refreshToken();
                if ('token' in args) {
                    args.token = accountInfoState.value.token;
                }
                return invokeCommand(args);
            }

            if (response.code !== 200) {
                result.value = null;
                error.value = response.message ?? 'unknown error';
                showModal(error.value);
            } else {
                result.value = response.result;
                error.value = null;
            }
            invoking.value = false;

            return response;
        } catch (err) {
            console.error(err);
            result.value = null;
            error.value = err!.toString();
            showModal(err!.toString());
        }
    }

    async function refreshToken() {
        const { code, result } = JSON.parse(await invoke('account_update_token', {
            token: accountInfoState.value.token,
        }));

        if (code !== 200) {
            return router?.replace('/auth/login');
        }
        accountInfoState.value = {
            isAuthenticated: true,
            ...result,
        };
        return result;
    }

    return {
        result,
        error,
        invoking,
        invoke: invokeCommand,
    };
}
