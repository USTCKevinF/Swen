import { createApp } from "vue";
import ElementPlus from "element-plus";
import "element-plus/dist/index.css";
import './index.css'
import App from "./App.vue";
import router from './router' // 你的路由配置文件
import { initStore } from './utils/store'
import i18n from './i18n'
import { initDatabase } from './utils/database'

// 先初始化 store，再挂载应用
initStore().then(() => {
  // 初始化数据库
  initDatabase().then(() => {
    console.log('数据库初始化成功')
  }).catch(error => {
    console.error('数据库初始化失败:', error)
  })

  const app = createApp(App);
  app.use(ElementPlus);
  app.use(router);
  app.use(i18n);
  app.mount("#app");
});
