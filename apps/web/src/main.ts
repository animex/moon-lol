import { createPinia } from "pinia";
import Persist from "pinia-plugin-persistedstate";
import { createApp } from "vue";
import { RouterView } from "vue-router";
import { router } from "./router";
import "./style.css";

const app = createApp(RouterView);

const pinia = createPinia();
pinia.use(Persist);

app.use(router);
app.use(pinia);

app.mount("#app");
