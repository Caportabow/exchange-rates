import { createApp } from 'vue'
import './style.css'
import 'fontawesome-4.7/css/font-awesome.min.css';
import Vue3Toastify, { type ToastContainerOptions } from 'vue3-toastify';
import 'vue3-toastify/dist/index.css';
import App from './App.vue'

createApp(App).use(
    Vue3Toastify,
    {
      autoClose: 3000,
      // ...Vue3Toasity,
      limit: 2,
    } as ToastContainerOptions,
  ).mount('#app');
