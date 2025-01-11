<script setup lang="ts">
import { ref } from "vue";
import OpenAI from "openai";
import { MdPreview } from 'md-editor-v3';
import 'md-editor-v3/lib/preview.css';

const props = defineProps<{
  inputText: string
}>();

const deepseekResponse = ref("");
const isLoading = ref(false);
const copySuccess = ref(false);

const openai = new OpenAI({
  baseURL: 'https://api.deepseek.com',
  apiKey: 'sk-258b756def1b41dab057106a0998f1ff',
  dangerouslyAllowBrowser: true
});

async function getDeepseekExplanation() {
  if (!props.inputText.trim()) return;
  
  isLoading.value = true;
  deepseekResponse.value = "";
  
  try {
    const completion = await openai.chat.completions.create({
      messages: [
        { role: "system", content: "You are a helpful assistant.Please use $ instead of \\( and \\) for LaTeX math expressions. " },
        { role: "user", content: props.inputText }
      ],
      model: "deepseek-chat",
      stream: true,
    });
    
    for await (const chunk of completion) {
      let content = chunk.choices[0]?.delta?.content || "";
      deepseekResponse.value += content;
    }
  } catch (error) {
    console.error('DeepSeek API 调用失败:', error);
    deepseekResponse.value = "抱歉，解释生成失败，请稍后重试。";
  } finally {
    isLoading.value = false;
  }
}

async function copyToClipboard() {
  try {
    await navigator.clipboard.writeText(deepseekResponse.value);
    copySuccess.value = true;
    setTimeout(() => {
      copySuccess.value = false;
    }, 2000);
  } catch (err) {
    console.error('复制失败:', err);
  }
}
</script>

<template>
  <div class="option-item">
    <div class="option-header" @click="getDeepseekExplanation">
      <span>DeepSeek 解释</span>
      <div class="header-actions">
        <button 
          v-if="deepseekResponse" 
          class="copy-button" 
          @click.stop="copyToClipboard"
        >
          <span v-if="copySuccess">✓ 已复制</span>
          <span v-else>复制内容</span>
        </button>
        <span class="loading-icon" v-if="isLoading">
          <span class="spinner"></span>
        </span>
        <span class="arrow" v-else>▼</span>
      </div>
    </div>
    <MdPreview 
      v-if="deepseekResponse" 
      :modelValue="deepseekResponse"
      :preview-theme="'default'"
      class="custom-md-preview"
    />
  </div>
</template>

<style scoped>
/* 保留相关样式 */
.option-item {
  border: 1px solid #ccc;
  border-radius: 8px;
  overflow: hidden;
}

.option-header {
  font-size: 15px;
  padding: 10px 15px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: #f5f5f5;
  cursor: pointer;
}

/* 其他样式保持不变... */
</style> 