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
import OpenAI from "openai";
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


const { property: baseURL, setProperty: setBaseURL } = useConfig('llm.baseURL', 'https://api.siliconflow.cn/v1')
const { property: apiKey, setProperty: setApiKey } = useConfig('llm.apiKey', '')
const { property: model, setProperty: setModel } = useConfig('llm.model', 'deepseek-ai/DeepSeek-V3')

let openai = new OpenAI({
  baseURL: baseURL.value,
  apiKey: apiKey.value,
  dangerouslyAllowBrowser: true
});

// 监听配置变化时重新创建实例
watch([baseURL, apiKey], (newValues: [string, string]) => {
  const [newBaseURL, newApiKey] = newValues;
  openai = new OpenAI({
    baseURL: newBaseURL,
    apiKey: newApiKey,
    dangerouslyAllowBrowser: true
  });
});

async function getDeepseekExplanation() {
  if (!props.inputText.trim()) return;
  
  isLoading.value = true;
  deepseekResponse.value = "";
  let systemPrompt = "你是一个包罗万象的知识专家，擅长于给用户解释其提出的概念，用户是一名好学且好奇的学生，会提供一些名词或者概念给你。规则：- 你需要详细地解答，并且以易懂的方式叙述你的观点 - 你的目标是让用户更深入的理解其提供的概念 - 无论用户提供的是什么语言，均用中文进行回复 - 输出完解释之后立刻停止，不要说类似如果你有更多的问题，欢迎继续提问的话.Please use $ instead of \\( and \\) for LaTeX math expressions.";

  try {
    const completion = await openai.chat.completions.create({
      messages: [
        { role: "system", content: systemPrompt },
        { role: "user", content: props.inputText }
      ],
      model: model.value,
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

// 监听变化并保存
watch(baseURL, (newValue: string) => {
    setBaseURL(newValue)
})

watch(apiKey, (newValue: string) => {
    setApiKey(newValue)
})

watch(model, (newValue: string) => {
    setModel(newValue)
})
</script>

<style scoped>
.copy-button {
  display: none;
}
</style> 