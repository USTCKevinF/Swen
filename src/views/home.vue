<script setup lang="ts">
import { ref } from "vue";
import DeepseekExplanation from '../components/DeepseekExplanation.vue';
import ArrowIcon from '../assets/arrow.svg';

const inputText = ref("");
</script>

<template>
  <main class="container">
    <el-input
        v-model="inputText"
        class="main-input"
        resize="none"
        :rows="4"
        type="textarea"
    />

    <div class="options-section">
      <div class="option-item">
        <div class="option-header">
          <span>金山词霸</span>
          <img :src="ArrowIcon" class="arrow" alt="arrow" />
        </div>
      </div>

      <div class="option-item">
        <div class="option-header">
          <span>4o-mini 翻译词/句</span>
          <img :src="ArrowIcon" class="arrow" alt="arrow" />
        </div>
      </div>

      <DeepseekExplanation :inputText="inputText" />
    </div>
  </main>
</template>
<style>
.md-editor-preview-wrapper {
    position: relative;
    flex: 1;
    box-sizing: border-box;
    overflow: auto;
    padding: 0px 15px;
}

.md-editor-preview {
  font-size: 13px !important;
  word-break: break-all !important;
  overflow: hidden !important;
}

/* 保持原有的 scoped 样式 */
.container {
  padding: 7px;
  height: 100vh;
  display: flex;
  flex-direction: column;
  gap: 7px;
  max-width: 100%;
}

.input-section {
  flex: 0 0 auto;
}

.main-input {
  width: 100%;
  /* border: 1px solid #ccc; */
  resize: none;
  font-size: 14px;
  outline: none;    
}

.options-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 10px;
  width: 100%;
}

.option-item {
  border: 0px solid #ccc;
  border-radius: 8px;
  overflow: hidden;
  background-color: #f8f8f8;
}

.option-header {
  font-size: 13px;
  padding: 7px 15px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: #f8f8f8;
  cursor: pointer;
}

.arrow {
  width: 12px;
  height: 12px;
  transition: transform 0.3s ease;
}

.arrow.active {
  transform: rotate(180deg);
}

.custom-md-preview {
  width: 100%;
  font-size: 12px; /* 调整为您需要的字体大小 */
  background-color: #f8f8f8;
}

.katex-error {
  color: black!important;
}

.loading-icon {
  display: inline-flex;
  align-items: center;
}

.spinner {
  width: 12px;
  height: 12px;
  border: 2px solid #f3f3f3;
  border-top: 2px solid #3498db;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

.copy-button {
  padding: 4px 12px;
  background-color: #f8f9fa;
  border: 1px solid #dee2e6;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  color: #495057;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 80px;
  height: 28px;
}

.copy-button:hover {
  background-color: #e9ecef;
  border-color: #ced4da;
}

.copy-button:active {
  background-color: #dee2e6;
  transform: translateY(1px);
}
</style>
