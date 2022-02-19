import {createApp} from 'vue';
import App from './App.vue';
import {router} from "./routes";
import './index.scss';
import {createPinia} from "pinia";
import {useUserStore} from "./stores/userStore";

async function bootstrap() {
    const app = createApp(App)
        .use(createPinia());
    await useUserStore().updateInfo();
    app
        .use(router)
        .mount('#app');
}

bootstrap().then(void 0)
