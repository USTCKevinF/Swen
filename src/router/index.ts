import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import home from '../views/home.vue'
import Default from '../views/default.vue'
import settings from '../views/settings.vue'
import screenshot from '../views/screenshot.vue'
const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Default',
    component: Default
  },
  {
    path: '/home',
    name: 'Home',
    component: home
  },
  {
    path: '/settings',
    name: 'Settings',
    component: settings,
  },
  {
    path: '/screenshot',
    name: 'Screenshot',
    component: screenshot,
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router