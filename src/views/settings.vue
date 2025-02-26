<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import { onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import General from '../components/settings/general.vue';
import Shortcut from '../components/settings/shortcut.vue';
import Model from '../components/settings/model.vue';
import About from '../components/settings/about.vue';
import Funding from '../components/settings/funding.vue';
import History from '../components/settings/history.vue';
import { Setting, Operation, Monitor, InfoFilled, MilkTea, Document } from '@element-plus/icons-vue'

const { t } = useI18n();
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
  } catch (err) {
    console.error(err)
  }
})

const getPageTitle = () => {
  switch(currentComponent.value) {
    case 'General': return t('settings.general.title');
    case 'Shortcut': return t('settings.shortcut.title');
    case 'Model': return t('settings.model.title');
    case 'About': return t('settings.about.title');
    case 'Funding': return t('settings.funding.title');
    case 'History': return t('settings.history.title');
    default: return t('settings.title');
  }
};

// 创建一个映射表，将字符串标识符映射到组件
const componentMap = {
  General,
  Shortcut,
  Model,
  About,
  Funding,
  History
};
</script>

<template>
  <div class="common-layout h-full">
    <el-container class="h-full">
      <el-header class="h-9 bg-[#f7f6f6] " data-tauri-drag-region='true'></el-header>
      <el-container>
        <el-aside class="w-[240px] border-r border-gray-100 bg-white overflow-hidden fixed-aside">
          <div class="py-4 px-4 font-medium text-gray-400 text-sm">{{ t('settings.title') }}</div>
          <el-menu class="h-full !border-0" :default-active="activeIndex">
            <el-menu-item index="1" @click="switchComponent('General', '1')" class="menu-item">
              <el-icon><Setting /></el-icon>
              <span>{{ t('settings.general.title') }}</span>
            </el-menu-item>
            <el-menu-item index="2" @click="switchComponent('Shortcut', '2')" class="menu-item">
              <el-icon><Operation /></el-icon>
              <span>{{ t('settings.shortcut.title') }}</span>
            </el-menu-item>
            <el-menu-item index="3" @click="switchComponent('Model', '3')" class="menu-item">
              <el-icon><Monitor /></el-icon>
              <span>{{ t('settings.model.title') }}</span>
            </el-menu-item>
            <el-menu-item index="6" @click="switchComponent('History', '6')" class="menu-item">
              <el-icon><Document /></el-icon>
              <span>{{ t('settings.history.title') }}</span>
            </el-menu-item>
            <div class="py-4 px-4 font-medium text-gray-400 text-sm">{{ t('settings.about.title') }}</div>
            <el-menu-item index="4" @click="switchComponent('About', '4')" class="menu-item">
              <el-icon><InfoFilled /></el-icon>
              <span>{{ t('settings.about.title') }}</span>
            </el-menu-item>
            <el-menu-item index="5" @click="switchComponent('Funding', '5')" class="menu-item">
              <el-icon><MilkTea /></el-icon>
              <span>{{ t('settings.funding.title') }}</span>
            </el-menu-item>
          </el-menu>
        </el-aside> 
        <el-main class="bg-gray-50 p-8 scrollable-main">
          <h2 class="text-xl font-medium mb-6">{{ getPageTitle() }}</h2>
          <div class="bg-white rounded-lg p-3 pl-1 shadow-sm h-5/6">
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

.fixed-aside {
  position: fixed;
  top: 36px;
  bottom: 0;
  left: 0;
}

.scrollable-main {
  margin-left: 240px;
  overflow-y: auto;
  height: calc(100vh - 36px);
}
</style>