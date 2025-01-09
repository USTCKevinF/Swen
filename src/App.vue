<script setup lang="ts">
import { ref, computed } from "vue";
import OpenAI from "openai";
import { marked } from 'marked';

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
        { role: "system", content: "You are a helpful assistant." },
        { role: "user", content: inputText.value }
      ],
      model: "deepseek-chat",
      stream: true,
    });
    
    for await (const chunk of completion) {
      deepseekResponse.value += chunk.choices[0]?.delta?.content || "";
    }
  } catch (error) {
    console.error('DeepSeek API 调用失败:', error);
    deepseekResponse.value = "抱歉，解释生成失败，请稍后重试。";
  } finally {
    isLoading.value = false;
  }
}

const formattedResponse = computed(() => {
  return marked(deepseekResponse.value);
});
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
        <div class="option-header" @click="getDeepseekExplanation">
          <span>DeepSeek 解释</span>
          <span class="arrow">{{ isLoading ? '...' : '▼' }}</span>
        </div>
        <div v-if="deepseekResponse" class="option-content" v-html="formattedResponse"></div>
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

.option-content {
  padding: 15px;
  line-height: 1.5;
}

/* 添加 markdown 样式 */
:deep(.option-content) {
  h1, h2, h3, h4, h5, h6 {
    margin-top: 1em;
    margin-bottom: 0.5em;
  }
  
  p {
    margin: 0.5em 0;
  }
  
  code {
    background-color: #f0f0f0;
    padding: 0.2em 0.4em;
    border-radius: 3px;
    font-family: monospace;
  }
  
  pre {
    background-color: #f5f5f5;
    padding: 1em;
    border-radius: 5px;
    overflow-x: auto;
  }
  
  blockquote {
    border-left: 4px solid #ddd;
    padding-left: 1em;
    margin: 1em 0;
    color: #666;
  }
}

/* 深色模式下的 markdown 样式 */
@media (prefers-color-scheme: dark) {
  :deep(.option-content) {
    code {
      background-color: #2d2d2d;
    }
    
    pre {
      background-color: #2a2a2a;
    }
    
    blockquote {
      border-left-color: #444;
      color: #999;
    }
  }
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
  
  .option-content {
    background-color: #1f1f1f;
    color: #fff;
  }
}
</style>