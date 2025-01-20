<script setup lang="ts">
import { ref } from "vue";
import OpenAI from "openai";
import { MdPreview } from 'md-editor-v3';
import 'md-editor-v3/lib/preview.css';
import ArrowIcon from '../assets/arrow.svg';
import CopyIcon from '../assets/copy.svg';
import RedoIcon from '../assets/redo.svg';
import { ElMessage } from 'element-plus'

const props = defineProps<{
  inputText: string
}>();

const deepseekResponse = ref("");
const isLoading = ref(false);

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
    ElMessage({
      message: '复制成功',
      type: 'success',
    });
  } catch (err) {
    console.error('复制失败:', err);
    ElMessage({
      message: '复制失败',
      type: 'error',
    });
  }
}

function handleRedo() {
  getDeepseekExplanation();
}
</script>

<template>
  <div class="option-item">
    <div class="option-header" @click="getDeepseekExplanation">
      <span>DeepSeek 解释</span>
      <div class="header-actions">
        <span class="loading-icon" v-if="isLoading">
          <span class="spinner"></span>
        </span>
        <img v-else :src="ArrowIcon" class="arrow" alt="arrow" />
      </div>
    </div>
    <div v-if="deepseekResponse" class="response-container">
      <MdPreview 
        :modelValue="deepseekResponse"
        :preview-theme="'default'"
        class="custom-md-preview"
      />
      <div v-if="!isLoading" class="copy-container">
        <div class="icon-wrapper">
          <img 
            :src="CopyIcon" 
            alt="copy" 
            @click="copyToClipboard"
            class="action-icon"
          />
          <span class="icon-tooltip">复制</span>
        </div>
        <div class="icon-wrapper">
          <img 
            :src="RedoIcon" 
            alt="redo" 
            @click="handleRedo"
            class="action-icon"
          />
          <span class="icon-tooltip">重新生成</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.response-container {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.copy-container {
  position: relative;
  display: flex;
  align-items: center;
  gap: 10px;
  padding-left: 13px;
  padding-bottom: 10px;
}

.action-icon {
  width: 20px;
  height: 20px;
  cursor: pointer;
  opacity: 0.9;
}

.action-icon:hover {
  opacity: 1;
}

.copy-tooltip {
  position: absolute;
  left: 30px;
  background: rgba(0, 0, 0, 0.7);
  color: white;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
}
.copy-button {
  display: none;
}

.icon-wrapper {
  position: relative;
  display: inline-block;
}

.icon-tooltip {
  position: absolute;
  bottom: 100%;
  left: 50%;
  transform: translateX(-50%);
  background: rgba(22, 22, 22, 0.7);
  color: white;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 10px;
  white-space: nowrap;
  opacity: 0;
  visibility: hidden;
  transition: opacity 0.2s, visibility 0.2s;
}

.icon-wrapper:hover .icon-tooltip {
  opacity: 1;
  visibility: visible;
}
</style> 