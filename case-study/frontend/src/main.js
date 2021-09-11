import Vue from 'vue'
import App from './App.vue'
import Vuetify from 'vuetify';
import vuetify from './plugins/vuetify'
import router from './router'
import store from './store';
import axios from 'axios'
import DatetimePicker from 'vuetify-datetime-picker'

Vue.config.productionTip = false
Vue.use(Vuetify, {
    theme: {
        primary: '#4CAF50',
    },
});
Vue.use(DatetimePicker);

new Vue({
    router,
    store,
    vuetify,
    axios,
    render: h => h(App)
}).$mount('#app')
