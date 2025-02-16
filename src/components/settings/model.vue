<template>
    <div>
        <el-form label-width="140px" class="settings-form">
            <el-form-item label="Base URL">
                <el-input 
                    v-model="baseURL" 
                    placeholder="请输入 Base URL"
                    class="w-full"
                />
            </el-form-item>

            <el-form-item label="API 密钥">
                <el-input 
                    v-model="apiKey" 
                    type="password" 
                    placeholder="请输入 API 密钥"
                    show-password
                    class="w-full"
                />
            </el-form-item>

            <el-form-item label="模型名称">
                <el-input 
                    v-model="model" 
                    placeholder="请输入模型名称"
                    class="w-full"
                />
            </el-form-item>

            <el-form-item>
                <div class="flex justify-end gap-4">
                    <el-button @click="handleCancel">取消</el-button>
                    <el-button type="primary" @click="handleSave">保存</el-button>
                </div>
            </el-form-item>
        </el-form>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useConfig } from '../../composables/useConfig'
import { ElMessage } from 'element-plus'

const { property: baseURL, setProperty: setBaseURL } = useConfig('llm.baseURL', 'https://api.siliconflow.cn/v1')
const { property: apiKey, setProperty: setApiKey } = useConfig('llm.apiKey', '')
const { property: model, setProperty: setModel } = useConfig('llm.model', '')

// 保存原始值
const originalBaseURL = ref('')
const originalApiKey = ref('')
const originalModel = ref('')

// 初始化时保存原始值
onMounted(() => {
    originalBaseURL.value = baseURL.value
    originalApiKey.value = apiKey.value
    originalModel.value = model.value
})

// 保存设置
const handleSave = () => {
    setBaseURL(baseURL.value)
    setApiKey(apiKey.value)
    setModel(model.value)
    
    // 更新原始值
    originalBaseURL.value = baseURL.value
    originalApiKey.value = apiKey.value
    originalModel.value = model.value
    
    ElMessage({
        message: '设置已保存',
        type: 'success'
    })
}

// 取消修改
const handleCancel = () => {
    baseURL.value = originalBaseURL.value
    apiKey.value = originalApiKey.value
    model.value = originalModel.value
}
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
    min-width: 140px;
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