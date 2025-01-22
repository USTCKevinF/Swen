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
    <div v-if="deepseekResponse" class="flex flex-col gap-2.5">
      <MdPreview 
        :modelValue="deepseekResponse"
        :preview-theme="'default'"
        class="custom-md-preview"
      />
      <div v-if="!isLoading" class="relative flex items-center gap-2.5 pl-3.5 pb-2.5">
        <div class="relative inline-block">
          <img 
            :src="CopyIcon" 
            alt="copy" 
            @click="copyToClipboard"
            class="w-5 h-5 cursor-pointer opacity-90 hover:opacity-100"
          />
          <span class="absolute bottom-full left-1/2 -translate-x-1/2 bg-[rgba(22,22,22,0.7)] text-white px-2 py-1 rounded text-[10px] whitespace-nowrap opacity-0 invisible transition-opacity duration-200 group-hover:opacity-100 group-hover:visible">复制</span>
        </div>
        <div class="relative inline-block">
          <img 
            :src="RedoIcon" 
            alt="redo" 
            @click="handleRedo"
            class="w-5 h-5 cursor-pointer opacity-90 hover:opacity-100"
          />
          <span class="absolute bottom-full left-1/2 -translate-x-1/2 bg-[rgba(22,22,22,0.7)] text-white px-2 py-1 rounded text-[10px] whitespace-nowrap opacity-0 invisible transition-opacity duration-200 group-hover:opacity-100 group-hover:visible">重新生成</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.copy-button {
  display: none;
}
</style> 