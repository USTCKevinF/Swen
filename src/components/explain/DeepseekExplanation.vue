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

<script setup lang="ts">
import { ref } from "vue";
import { MdPreview } from 'md-editor-v3';
import 'md-editor-v3/lib/preview.css';
import ArrowIcon from '../../assets/arrow.svg';
import CopyIcon from '../../assets/copy.svg';
import RedoIcon from '../../assets/redo.svg';
import { ElMessage } from 'element-plus'
import { useConfig } from '../../composables/useConfig'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core';

const props = defineProps<{
  inputText: string
}>();

const deepseekResponse = ref("");
const isLoading = ref(false);


const { property: baseURL } = useConfig('llm.baseURL', '')
const { property: apiKey } = useConfig('llm.apiKey', '')
const { property: model } = useConfig('llm.model', '')

async function getDeepseekExplanation() {
  if (!props.inputText.trim()) return;
  
  isLoading.value = true;
  deepseekResponse.value = "";
  let systemPrompt = "你是一个包罗万象的知识专家，擅长于给用户解释其提出的概念，用户是一名好学且好奇的学生，会提供一些名词或者概念给你。规则：- 你需要详细地解答，并且以易懂的方式叙述你的观点 - 你的目标是让用户更深入的理解其提供的概念 - 无论用户提供的是什么语言，均用中文进行回复 - 输出完解释之后立刻停止，不要说类似如果你有更多的问题，欢迎继续提问的话.Please use $ instead of \\( and \\) for LaTeX math expressions.";

  try {
    const unlisten = await listen('fetch-stream-data', (event: any) => {
      const { message } = event.payload;
      console.log('Received stream data:', message);
      if (message && typeof message === 'string') {
        // 按照两个换行符分割多条数据
        const messages = message.split('\n\n');
        
        messages.forEach(msg => {
          if (!msg.trim()) return; // 跳过空消息
          
          try {
            // 检查是否是结束标记
            if (msg.includes('[DONE]')) {
              return;
            }
            
            // 移除 "data: " 前缀
            const jsonStr = msg.replace(/^data: /, '');
            const data = JSON.parse(jsonStr);
            
            // 获取实际的内容
            const content = data.choices[0]?.delta?.content;
            if (content) {
              deepseekResponse.value += content;
            }
          } catch (e) {
            console.error('解析消息失败:', e, msg);
          }
        });
      }
    });

    // 调用后端
    await invoke('receive_stream', {
      url: `${baseURL.value}/chat/completions`,
      cookie: `Bearer ${apiKey.value}`,
      prompt: JSON.stringify({
        model: model.value,
        messages: [
          { role: "system", content: systemPrompt },
          { role: "user", content: props.inputText }
        ],
        stream: true
      })
    });

    unlisten();
  } catch (error) {
    console.error('流处理错误:', error);
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

<style scoped>
.copy-button {
  display: none;
}
</style> 