import { setupLayouts } from 'virtual:generated-layouts';
import { createRouter, createWebHistory } from 'vue-router/auto';

import { type UserModule } from '~/types';

export const router = createRouter({
    history: createWebHistory(),
    extendRoutes: (routes) => {
        return setupLayouts(routes);
    },
});

router.beforeEach((to, _, next) => {
    const accountInfoState = useAccountInfoState();
    const appState = useAppState();

    if (!appState.isInitialized.value) {
        next(false);
    } else {
        if (to.meta.requiresAuth && !accountInfoState.value.isAuthenticated) {
            next({ path: '/auth/login' });
        } else {
            next();
        }
    }
});

export const install: UserModule = (app) => {
    app.use(router);
};
