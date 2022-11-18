// The Vue build version to load with the `import` command
// (runtime-only or standalone) has been set in webpack.base.conf with an alias.
import Vue from 'vue';
import App from './App';
import router from './router';
import axios from 'axios';
import MuseUI from 'muse-ui';
import moment from 'moment';
import Loading from 'muse-ui-loading';
import Toast from 'muse-ui-toast';
import Message from 'muse-ui-message';
import 'muse-ui/dist/muse-ui.css';
import 'muse-ui-loading/dist/muse-ui-loading.css'; 
import 'muse-ui-message/dist/muse-ui-message.css';

moment.locale('zh-cn');

Vue.use(MuseUI);
Vue.use(Loading);
Vue.use(Message);
Vue.use(Toast,{
  position: 'top',                  // 弹出的位置
  time: 2000,                       // 显示的时长
  closeIcon: '',                    // 关闭的图标
  close: false,                     // 是否显示关闭按钮
  successIcon: '',                  // 成功信息图标
  infoIcon: '',                     // 信息信息图标
  warningIcon: '',                  // 提醒信息图标
  errorIcon: ''                     // 错误信息图标
});

Vue.config.productionTip = false
Vue.prototype.axios = axios;
Vue.prototype.moment = moment;
Vue.prototype.urls1 = "http://127.0.0.1:9021/";
Vue.prototype.urls2 = "http://127.0.0.1:9022/";
Vue.prototype.urls3 = "http://127.0.0.1:9023/";

/* eslint-disable no-new */
new Vue({
  el: '#app',
  router,
  components: { App },
  template: '<App/>'
})
