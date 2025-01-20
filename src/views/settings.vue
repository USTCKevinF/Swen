<script setup lang="ts">
import { getCurrentWebviewWindow} from '@tauri-apps/api/webviewWindow';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { onMounted } from 'vue';

const webviewWindow = getCurrentWebviewWindow();
const currentWindow = getCurrentWindow();

const hideWindow = async () => {
  try {
    console.log("close")
    await currentWindow.close();
  } catch (err) {
    console.error(err);
  }
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
</script>

<template>
  <div class="config-container">
    <h2>配置页面</h2>
    <div class="config-content">
      <!-- 配置内容 -->
       12312313123
       <button @click="hideWindow" class="hide-btn">隐藏窗口</button>
    </div>
  </div>
</template>

<style scoped>
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
</style>