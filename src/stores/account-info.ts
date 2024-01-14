interface AccountInfoState {
    isAuthenticated: boolean;
    token: string;
    email: string;
    exp: number;
    surplus_day: number;
    is_card: number;
    end_at?: string;
}

export const useAccountInfoState = createGlobalState(
    () => useStorage<AccountInfoState>('vueuse-local-storage-account-info', {
        isAuthenticated: false,
        token: '',
        email: '',
        exp: 0,
        surplus_day: 0,
        is_card: 0,
        end_at: '',
    }),
);
