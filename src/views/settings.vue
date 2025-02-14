<script setup lang="ts">
import { getCurrentWebviewWindow} from '@tauri-apps/api/webviewWindow';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { onMounted, ref } from 'vue';
import General from '../components/settings/general.vue';
import Shortcut from '../components/settings/shortcut.vue';
import Model from '../components/settings/model.vue';
import About from '../components/settings/about.vue';
import Funding from '../components/settings/funding.vue';
import { Setting, Operation, Monitor, InfoFilled, MilkTea } from '@element-plus/icons-vue'

const webviewWindow = getCurrentWebviewWindow();
const currentWindow = getCurrentWindow();

const currentComponent = ref('General');
const activeIndex = ref('1');

const switchComponent = (componentName: string, index: string) => {
  currentComponent.value = componentName;
  activeIndex.value = index;
};

onMounted(async () => {
  try {
    currentWindow.show();
    // 使用 dialog 显示信息
    console.log(webviewWindow.label)
  } catch (err) {
    console.error(err)
  }
})

const getPageTitle = () => {
  switch(currentComponent.value) {
    case 'General': return '通用设置';
    case 'Shortcut': return '快捷键设置';
    case 'Model': return '模型设置';
    case 'About': return '关于应用';
    case 'Funding': return '赞助奶茶';
    default: return '设置';
  }
};

// 创建一个映射表，将字符串标识符映射到组件
const componentMap = {
  General,
  Shortcut,
  Model,
  About,
  Funding
};
</script>

<template>
  <div class="common-layout h-full">
    <el-container class="h-full">
      <el-header class="h-9 bg-[#f7f6f6] " data-tauri-drag-region='true'></el-header>
      <el-container>
        <el-aside class="w-[240px] border-r border-gray-100 bg-white overflow-hidden">
          <div class="py-4 px-4 font-medium text-gray-400 text-sm">设置</div>
          <el-menu class="h-full !border-0" :default-active="activeIndex">
            <el-menu-item index="1" @click="switchComponent('General', '1')" class="menu-item">
              <el-icon><Setting /></el-icon>
              <span>通用设置</span>
            </el-menu-item>
            <el-menu-item index="2" @click="switchComponent('Shortcut', '2')" class="menu-item">
              <el-icon><Operation /></el-icon>
              <span>快捷键设置</span>
            </el-menu-item>
            <el-menu-item index="3" @click="switchComponent('Model', '3')" class="menu-item">
              <el-icon><Monitor /></el-icon>
              <span>模型设置</span>
            </el-menu-item>
            <div class="py-4 px-4 font-medium text-gray-400 text-sm">关于</div>
            <el-menu-item index="4" @click="switchComponent('About', '4')" class="menu-item">
              <el-icon><InfoFilled /></el-icon>
              <span>关于应用</span>
            </el-menu-item>
            <el-menu-item index="5" @click="switchComponent('Funding', '5')" class="menu-item">
              <el-icon><MilkTea /></el-icon>
              <span>赞助奶茶</span>
            </el-menu-item>
          </el-menu>
        </el-aside> 
        <el-main class="bg-gray-50 p-8">
          <h2 class="text-xl font-medium mb-6">{{ getPageTitle() }}</h2>
          <div class="bg-white rounded-lg p-6 shadow-sm">
            <component :is="componentMap[currentComponent]" />
          </div>
        </el-main>
      </el-container>
    </el-container>
  </div>
</template>

<style scoped>
:global(body) {
  background: transparent;
  height: 100vh;
}

.config-container {
  padding: 20px;
}

.config-content {
  margin-top: 20px;
}

.hide-btn {
  padding: 8px 16px;
  background-color: #4a5568;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.hide-btn:hover {
  background-color: #2d3748;
}

.menu-item {
  margin: 8px 14px;
  border-radius: 8px;
  height: 45px !important;
  font-size: 14px;
}

:deep(.el-menu-item) {
  &:hover {
    background-color: var(--el-menu-hover-bg-color);
    border-radius: 8px;
  }
  
  &.is-active {
    background-color: var(--el-color-primary-light-9);
    color: var(--el-color-primary);
    border-radius: 8px;
    font-weight: 500;
  }
}

.el-icon {
  font-size: 16px;
  margin-right: 12px;
  color: #666;
}

:deep(.el-main) {
  --el-main-padding: 32px;
}
</style>