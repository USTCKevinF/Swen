<template>
    <div>
        <el-form label-width="120px" class="settings-form">
            <el-form-item label="显示语言">
                <el-select v-model="language" placeholder="请选择语言" >
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
import { useConfig } from '../../composables/useConfig'
import { watch, ref } from 'vue'

export default {
  setup() {
    const { property: language, setProperty: setLanguage, getProperty: getLanguage } = useConfig('language', '')
    console.log('初始语言设置：', language.value)

    const languages = ref([
      { value: 'zh', label: '中文' },
      { value: 'en', label: 'English' }
    ])

    // 监听语言变化
    watch(language, (newValue) => {
      console.log('当前选择的语言：', newValue)
      if (newValue) {
        setLanguage(newValue)
      }
    })

    return {
      language,
      languages
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