<template>
    <div>
        <el-form label-width="140px" class="settings-form">
            <el-form-item :label="t('settings.model.baseURL')">
                <el-input 
                    v-model="baseURL" 
                    placeholder="https://api.openai.com/v1"
                    class="w-full"
                />
            </el-form-item>

            <el-form-item :label="t('settings.model.apiKey')">
                <el-input 
                    v-model="apiKey" 
                    type="password" 
                    placeholder="sk-1234567890"
                    show-password
                    class="w-full"
                />
            </el-form-item>

            <el-form-item :label="t('settings.model.modelName')">
                <el-input 
                    v-model="model" 
                    placeholder="gpt-4"
                    class="w-full"
                />
            </el-form-item>

            <el-form-item :label="t('settings.model.maxContextLength')">
                <el-input-number
                    v-model="maxContextLength"
                    :min="1"
                    :max="16"
                    class="w-30"
                />
            </el-form-item>

            <el-form-item>
                <div class="flex gap-2 justify-end w-full">
                    <el-button 
                        @click="resetSettings" 
                        :disabled="!hasChanges"
                    >{{ t('settings.model.cancel') }}</el-button>
                    <el-button 
                        type="primary" 
                        @click="saveSettings" 
                        :disabled="!hasChanges"
                    >{{ t('settings.model.save') }}</el-button>
                </div>
            </el-form-item>

            <el-form-item :label="t('settings.model.testConnection')">
                <el-input 
                    v-model="testInput"
                    :placeholder="t('settings.model.testInput')"
                    class="w-full"
                    :disabled="isTesting"
                />
                <el-button 
                    type="primary" 
                    @click="testConnection"
                    :loading="isTesting"
                    class="mt-2"
                >
                    {{ t('settings.model.testButton') }}
                </el-button>
                <el-alert
                    v-if="testOutput"
                    :title="isConnected ? t('settings.model.connectionSuccess') : t('settings.model.connectionFailed')"
                    :type="isConnected ? 'success' : 'error'"
                    class="mt-2"
                >
                    {{ testOutput }}
                </el-alert>
            </el-form-item>
        </el-form>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useConfig } from '../../composables/useConfig'
import { fetch } from '@tauri-apps/plugin-http'
import { ElMessage } from 'element-plus'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const { property: baseURL, setProperty: setBaseURL, isLoaded: isBaseURLLoaded } = useConfig('llm.baseURL', '')
const { property: apiKey, setProperty: setApiKey, isLoaded: isApiKeyLoaded } = useConfig('llm.apiKey', '')
const { property: model, setProperty: setModel, isLoaded: isModelLoaded } = useConfig('llm.model', '')
const { property: maxContextLength, setProperty: setMaxContextLength, isLoaded: isMaxContextLengthLoaded } = useConfig('llm.maxContextLength', 6)

// 新增测试相关状态
const testInput = ref(t('settings.model.testInput'))
const testOutput = ref('')
const isConnected = ref(false)
const isTesting = ref(false)

const originalBaseURL = ref('')
const originalApiKey = ref('')
const originalModel = ref('')
const originalMaxContextLength = ref(0)

onMounted(async () => {
  // 等待配置加载完成
  await new Promise(resolve => {
    const check = () => {
      if (baseURL.value !== undefined && 
          apiKey.value !== undefined && 
          model.value !== undefined &&
          maxContextLength.value !== undefined) {
        resolve(true)
      } else {
        setTimeout(check, 50)
      }
    }
    check()
  })
  
  originalBaseURL.value = baseURL.value
  originalApiKey.value = apiKey.value
  originalModel.value = model.value
  originalMaxContextLength.value = maxContextLength.value
})

// 添加计算属性检查设置是否有变动
const hasChanges = computed(() => {
    return baseURL.value !== originalBaseURL.value ||
           apiKey.value !== originalApiKey.value ||
           model.value !== originalModel.value ||
           maxContextLength.value !== originalMaxContextLength.value
})

const testConnection = async () => {
    try {
        isTesting.value = true
        testOutput.value = ''
        
        const response = await fetch(`${baseURL.value}/chat/completions`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${apiKey.value}`
            },
            body: JSON.stringify({
                model: model.value,
                messages: [
                    {
                        role: "system",
                        content: "你是一个专业的AI助手，请用一句话回答用户的问题，用户用什么语言，你就用什么语言回答"
                    },
                    {
                        role: "user",
                        content: testInput.value
                    }
                ]
            })
        });

        const data = await response.json()
        if (data.choices?.[0]?.message?.content) {
            testOutput.value = data.choices[0].message.content
            isConnected.value = true
        } else {
            throw new Error('Invalid response format')
        }
    } catch (error) {
        console.error('完整错误对象:', error)
        testOutput.value = `连接失败: ${error instanceof Error ? error.message : String(error)}`
        isConnected.value = false
    } finally {
        isTesting.value = false
    }
}

const saveSettings = () => {
    setBaseURL(baseURL.value)
    setApiKey(apiKey.value)
    setModel(model.value)
    setMaxContextLength(maxContextLength.value)
    
    originalBaseURL.value = baseURL.value
    originalApiKey.value = apiKey.value
    originalModel.value = model.value
    originalMaxContextLength.value = maxContextLength.value
    
    ElMessage.success(t('settings.model.saveSuccess'))
}

const resetSettings = () => {
    baseURL.value = originalBaseURL.value
    apiKey.value = originalApiKey.value
    model.value = originalModel.value
    maxContextLength.value = originalMaxContextLength.value
    ElMessage.info(t('settings.model.cancelChanges'))
}

const allConfigLoaded = computed(() => 
  isBaseURLLoaded.value && 
  isApiKeyLoaded.value && 
  isModelLoaded.value &&
  isMaxContextLengthLoaded.value
)

watch(allConfigLoaded, (loaded: boolean) => {
  if (loaded) {
    originalBaseURL.value = baseURL.value
    originalApiKey.value = apiKey.value
    originalModel.value = model.value
    originalMaxContextLength.value = maxContextLength.value
  }
})
</script>

<style scoped>
.settings-form {
    max-width: 600px;
    margin: 20px auto;
    font-size: 14px;
}

.el-form-item {
    margin-bottom: 20px;
}

.el-form-item :deep(.el-form-item__content) {
    justify-content: flex-end;
    display: flex;
}

.el-form-item :deep(.el-form-item__label) {
    white-space: nowrap;
    min-width: 180px;
}

.el-form-item :deep(.el-form-item__content) {
    flex: 1;
    min-width: 200px;
}

.settings-form :deep(.el-input),
.settings-form :deep(.el-select) {
    font-size: 14px;
}
</style>