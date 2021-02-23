import Vue from 'vue';
import App from './App';
import vSelect from 'vue-select';

Vue.component('v-select', vSelect);
Vue.config.productionTip = false;

/* eslint-disable no-new */
new Vue({
  el: '#app',
  render: h => h(App),
});
