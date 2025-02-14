<template>
    <div>
        <el-form label-width="120px" class="settings-form">
            <el-form-item label="显示语言">
                <el-select v-model="selectedLanguage" placeholder="请选择语言" @change="handleLanguageChange">
                    <el-option
                        v-for="lang in languages"
                        :key="lang.value"
                        :label="lang.label"
                        :value="lang.value"
                    />
                </el-select>
            </el-form-item>
        </el-form>
    </div>
</template>

<script>
import { store } from '../../utils/store'

export default {
    data() {
        return {
            selectedLanguage: 'zh',
            languages: [
                { value: 'zh', label: '中文' },
                { value: 'en', label: 'English' }
            ]
        }
    },
    async created() {
        // 从 store 中读取语言设置
        const lang = await store?.get('language') || 'zh'
        this.selectedLanguage = lang
    },
    methods: {
        async handleLanguageChange(newValue) {
            if (store) {
                // 将新的语言设置保存到 store
                await store.set('language', newValue)
                await store.save()
                console.log('语言已切换并保存至 store:', newValue)
            }
        }
    }
}
</script>

<style scoped>
.settings-form {
    max-width: 500px;
    margin: 20px auto;
}
</style>