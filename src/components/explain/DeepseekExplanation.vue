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
import { ref, watch } from "vue";
import { fetch } from '@tauri-apps/plugin-http'
import { MdPreview } from 'md-editor-v3';
import 'md-editor-v3/lib/preview.css';
import ArrowIcon from '../../assets/arrow.svg';
import CopyIcon from '../../assets/copy.svg';
import RedoIcon from '../../assets/redo.svg';
import { ElMessage } from 'element-plus'
import { useConfig } from '../../composables/useConfig'

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
    const response = await fetch(`${baseURL.value}/chat/completions`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${apiKey.value}`
      },
      body: JSON.stringify({
        model: model.value,
        messages: [
          { role: "system", content: systemPrompt },
          { role: "user", content: props.inputText }
        ],
        stream: true
      })
    });

    
    // 第一步：验证响应是否可读
    console.log('响应状态:', response);
    console.log('Content-Type:', response.headers.get('Content-Type'));
    if (!response.body) {
      console.error('响应体为空');
      return;
    }

    // 新增流式特性验证
    console.log('可读流类型:', response.body?.constructor.name); // 应该显示 ReadableStream
    console.log('流锁定状态:', response.body?.locked); // 应该为 false

    const reader = response.body!
      .pipeThrough(new TextDecoderStream())
      .getReader();

    // 新增流量控制计数器
    let chunkCounter = 0;
    let eventCounter = 0;
    const startTime = Date.now();

    // 使用环形缓冲区处理数据
    let buffer = '';
    while (true) {
      const { done, value } = await reader.read();
      if (done) {
        console.log(`[${Date.now() - startTime}ms] 流结束`);
        break;
      }

      chunkCounter++;
      console.log(`[${Date.now() - startTime}ms] 收到第 ${chunkCounter} 个数据块，长度: ${value.length}`);

      buffer += value;
      let boundary;
      
      // 改进的事件分割逻辑
      while ((boundary = buffer.indexOf('\n\n')) >= 0) {
        eventCounter++;
        const eventData = buffer.slice(0, boundary);
        buffer = buffer.slice(boundary + 2);
        
        console.groupCollapsed(`[Event ${eventCounter}]`);
        console.log('原始事件数据:', eventData);
        
        // 处理多行数据
        const lines = eventData.split('\n').filter(line => line.startsWith('data:'));
        const combinedData = lines.map(line => line.slice(5).trim()).join('');
        
        try {
          const data = JSON.parse(combinedData);
          console.log('解析结果:', data);
          if (data.choices?.[0]?.delta?.content) {
            const content = data.choices[0].delta.content;
            console.log('追加内容:', content);
            deepseekResponse.value += content;
          }
        } catch (err) {
          console.error('解析异常:', err);
        }
        console.groupEnd();
      }
    }
    
    console.log(`总计处理 ${chunkCounter} 个数据块，${eventCounter} 个事件`);
    
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

<style scoped>
.copy-button {
  display: none;
}
</style> 