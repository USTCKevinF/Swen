import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import home from '../views/home.vue'
import settings from '../views/settings.vue'
const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Home',
    component: home
  },
  {
    path: '/settings',
    name: 'Settings',
    component: settings,
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router