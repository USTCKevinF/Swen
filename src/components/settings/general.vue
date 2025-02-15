<template>
    <div>
        <el-form label-width="140px" class="settings-form">
            <el-form-item :label="t('settings.general.displayLanguage')">
                <el-select v-model="language" :placeholder="t('settings.general.displayLanguage')" >
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
import { useI18n } from 'vue-i18n'

export default {
  setup() {
    const { t, locale } = useI18n()
    const { property: language, setProperty: setLanguage } = useConfig('language', 'zh')
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
        locale.value = newValue // 更新 i18n 的语言设置
      }
    })

    return {
      t,
      language,
      languages
    }
  }
}
</script>

<style scoped>
.settings-form {
    max-width: 600px;
    margin: 20px auto;
}

.el-form-item {
    margin-bottom: 20px;
}
</style>