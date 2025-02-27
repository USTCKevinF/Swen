<template>
  <div class="about-container">
    <!-- 应用图标 -->
    <div class="logo-container">
      <img src="/src-tauri/icons/icon.png" alt="Swen Logo" class="app-logo" />
    </div>
    
    <!-- 应用名称和版本 -->
    <div class="app-info">
      <h1>Swen {{ version }}</h1>
    </div>
    
    <!-- 链接区域 -->
    <div class="links-container">
      <div class="links-row">
        <a href="https://www.swen-app.com" class="link-item" target="_blank">
          <i class="icon website-icon"></i> Swen 官方网站
        </a>
        <div class="divider"></div>
        <a href="https://www.swen-app.com" class="link-item" target="_blank">
          <i class="icon qq-icon"></i> QQ 交流群
        </a>
      </div>
      
      <div class="links-row">
        <a href="https://www.swen-app.com" class="link-item" target="_blank">
          <i class="icon privacy-icon"></i> 隐私政策
        </a>
        <div class="divider"></div>
        <a href="https://www.swen-app.com" class="link-item" target="_blank">
          <i class="icon terms-icon"></i> 使用条款
        </a>
      </div>
    </div>
  </div>
</template>

<script>
import { reactive, toRefs } from 'vue';
import tauriConfig from '../../../src-tauri/tauri.conf.json';

export default {
  setup() {
    const state = reactive({
      version: tauriConfig.version
    });
    
    return {
      ...toRefs(state)
    };
  }
};
</script>

<style scoped>
.about-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 2rem;
  max-width: 600px;
  margin: 0 auto;
  text-align: center;
}

.logo-container {
  margin-bottom: 1rem;
}

.app-logo {
  width: 120px;
  height: 120px;
  border-radius: 20px;
}

.app-info {
  margin-bottom: 2rem;
}

.app-info h1 {
  font-size: 1.8rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
}

.pro-badge {
  background-color: rgba(200, 230, 240, 0.6);
  padding: 0.2rem 0.6rem;
  border-radius: 0.3rem;
  font-size: 0.9rem;
  font-weight: normal;
}

.pro-description {
  color: #666;
  margin-top: 1rem;
}

.links-container {
  width: 100%;
  margin-bottom: 2rem;
}

.links-row {
  display: flex;
  justify-content: center;
  align-items: center;
  margin-bottom: 1rem;
}

.link-item {
  display: flex;
  align-items: center;
  color: #333;
  text-decoration: none;
  padding: 0.5rem 1rem;
}

.link-item:hover {
  color: #0066cc;
}

.icon {
  margin-right: 0.5rem;
  width: 20px;
  height: 20px;
  display: inline-block;
}

.divider {
  width: 1px;
  height: 20px;
  background-color: #ddd;
  margin: 0 1rem;
}

</style>