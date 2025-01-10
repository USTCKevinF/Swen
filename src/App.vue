<script setup lang="ts">
import { ref } from "vue";
import OpenAI from "openai";
import { MdPreview } from 'md-editor-v3';
import 'md-editor-v3/lib/preview.css';

const inputText = ref("");
const deepseekResponse = ref("");
const isLoading = ref(false);

const openai = new OpenAI({
  baseURL: 'https://api.deepseek.com',
  apiKey: 'sk-258b756def1b41dab057106a0998f1ff',
  dangerouslyAllowBrowser: true // 允许在浏览器中使用
});

async function getDeepseekExplanation() {
  if (!inputText.value.trim()) return;
  
  isLoading.value = true;
  deepseekResponse.value = "";
  
  try {
    const completion = await openai.chat.completions.create({
      messages: [
        { role: "system", content: "You are a helpful assistant.Please use $ instead of \\( and \\) for LaTeX math expressions. " },
        { role: "user", content: inputText.value }
      ],
      model: "deepseek-chat",
      stream: true,
    });
    
    for await (const chunk of completion) {
      console.log('收到的chunk:', chunk);
      let content = chunk.choices[0]?.delta?.content || "";
      deepseekResponse.value += content;
    }
    console.log(deepseekResponse.value);
  } catch (error) {
    console.error('DeepSeek API 调用失败:', error);
    deepseekResponse.value = "抱歉，解释生成失败，请稍后重试。";
  } finally {
    isLoading.value = false;
  }
}
</script>

<!-- <template>
  <div class="container">
    <div class="input-section">
      <textarea
        v-model="inputText"
        placeholder="请输入你的问题..."
        rows="4"
      ></textarea>
      <button @click="getDeepseekExplanation" :disabled="isLoading">
        {{ isLoading ? '生成中...' : '生成解释' }}
      </button>
    </div>

    <div class="response-section">
      <MdPreview 
        v-if="deepseekResponse" 
        :modelValue="deepseekResponse"
        :preview-theme="'default'"
        :style="{ width: '100%' }"
      />
    </div>
  </div>
</template> -->

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
          <MdPreview 
          :modelValue="inputText"
          :preview-theme="'default'"
          class="custom-md-preview"
        />
        </div>
      </div>

      <div class="option-item">
        <div class="option-header" @click="getDeepseekExplanation">
          <span>DeepSeek 解释</span>
          <span class="arrow">{{ isLoading ? '...' : '▼' }}</span>
        </div>
        <MdPreview 
          v-if="deepseekResponse" 
          :modelValue="deepseekResponse"
          :preview-theme="'default'"
          class="custom-md-preview"
        />
      </div>
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
/* .md-editor-katex-inline {
  display: flex;
  justify-content: center;
}
全局样式，移除 scoped */
.md-editor-preview {
  font-size: 13px !important;
  word-break: break-all !important;
  overflow: hidden !important;
}

/* 保持原有的 scoped 样式 */
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
  font-size: 14px;
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

.custom-md-preview {
  width: 100%;
  font-size: 12px; /* 调整为您需要的字体大小 */
}
.katex-error {
  color: black!important;
}
</style>
