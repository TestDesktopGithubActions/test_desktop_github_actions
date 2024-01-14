import { createApp } from 'vue';

import 'virtual:uno.css';
import './styles.scss';

import App from './App.vue';
import { type UserModule } from '~/types';

const app = createApp(App);

// install all modules under `modules/`
Object.values(import.meta.glob<{ install: UserModule }>('./modules/*.ts', { eager: true })).forEach(i => i.install?.(app));

app.mount('#app');

// disable context menu in production
if (import.meta.env.PROD) {
    document.addEventListener('contextmenu', e => e.preventDefault());
}
