import Vue from 'vue'
import App from './App.vue'
import router from './router'
import VueNativeSock from 'vue-native-websocket';

Vue.use(VueNativeSock, 'ws://localhost:3001', {
    format:'json',
    connectManually: true
});

Vue.config.productionTip = false

new Vue({
  router,
  render: h => h(App),
}).$mount('#app')
