<script setup lang="ts">
import { ref } from "vue";
// import { invoke } from "@tauri-apps/api/core";

const inputText = ref("");

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//   greetMsg.value = await invoke("greet", { name: name.value });
// }
</script>

<template>
  <main class="container">
    <div class="input-section">
      <textarea 
        v-model="inputText" 
        class="main-input" 
        placeholder="请输入文本..."
      ></textarea>
    </div>

    <div class="options-section">
      <div class="option-item">
        <div class="option-header">
          <span>自动检测</span>
          <span class="arrow">▼</span>
        </div>
      </div>

      <div class="option-item">
        <div class="option-header">
          <span>金山词霸</span>
          <span class="arrow">▼</span>
        </div>
      </div>

      <div class="option-item">
        <div class="option-header">
          <span>4o-mini 翻译词/句</span>
          <span class="arrow">▼</span>
        </div>
      </div>

      <div class="option-item">
        <div class="option-header">
          <span>DeepSeek 解释</span>
          <span class="arrow">▼</span>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>
.container {
  padding: 20px;
  height: 100vh;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.input-section {
  flex: 0 0 auto;
}

.main-input {
  width: 100%;
  height: 150px;
  padding: 10px;
  border: 1px solid #ccc;
  border-radius: 8px;
  resize: none;
}

.options-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.option-item {
  border: 1px solid #ccc;
  border-radius: 8px;
  overflow: hidden;
}

.option-header {
  padding: 10px 15px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: #f5f5f5;
  cursor: pointer;
}

.arrow {
  font-size: 12px;
}

/* 深色模式适配 */
@media (prefers-color-scheme: dark) {
  .option-header {
    background-color: #2a2a2a;
  }
  
  .main-input {
    background-color: #1f1f1f;
    color: #fff;
    border-color: #444;
  }
  
  .option-item {
    border-color: #444;
  }
}
</style>