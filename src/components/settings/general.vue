<template>
    <div>
        <el-form label-width="140px" class="settings-form">
            <el-form-item :label="t('settings.general.displayLanguage')">
                <el-select v-model="language" :placeholder="t('settings.general.displayLanguage')" >
                    <template #prefix>
                        <span :class="`fi fi-${LanguageFlag[language]}`"></span>
                    </template>
                    <el-option
                        v-for="lang in languages"
                        :key="lang.value"
                        :label="lang.label"
                        :value="lang.value"
                    >
                        <span class="flex items-center gap-2">
                            <span :class="`fi fi-${LanguageFlag[lang.value]}`"></span>
                            {{ lang.label }}
                        </span>
                    </el-option>
                </el-select>
            </el-form-item>

            <el-form-item :label="t('settings.general.windowPosition')">
                <el-select v-model="windowPosition" :placeholder="t('settings.general.windowPosition')">
                    <el-option
                        v-for="pos in windowPositions"
                        :key="pos.value"
                        :label="t(pos.label)"
                        :value="pos.value"
                    />
                </el-select>
            </el-form-item>

            <el-form-item :label="t('settings.general.rememberSize')">
                <el-switch v-model="rememberSize" />
            </el-form-item>

            <el-form-item :label="t('settings.general.alwaysOnTop')">
                <el-switch v-model="alwaysOnTop" />
            </el-form-item>
        </el-form>
    </div>
</template>

<script setup lang="ts">
import { useConfig } from '../../composables/useConfig'
import { watch, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import 'flag-icons/css/flag-icons.min.css'
import { LanguageFlag } from '../../utils/language'

const { t, locale } = useI18n()
const { property: language, setProperty: setLanguage } = useConfig('language', 'zh')
console.log('初始语言设置：', language.value)

const languages = ref([
  { value: 'zh', label: '中文' },
  { value: 'en', label: 'English' }
])

const windowPositions = ref([
  { value: 'followMouse', label: 'settings.general.followMouse' },
  { value: 'center', label: 'settings.general.center' },
  { value: 'remember', label: 'settings.general.remember' }
])

const { property: windowPosition, setProperty: setWindowPosition } = useConfig('windowPosition', 'center')
const { property: rememberSize, setProperty: setRememberSize } = useConfig('rememberSize', false)
const { property: alwaysOnTop, setProperty: setAlwaysOnTop } = useConfig('alwaysOnTop', true)

// 监听语言变化
watch(language, (newValue: string) => {
  console.log('当前选择的语言：', newValue)
  if (newValue) {
    setLanguage(newValue)
    locale.value = newValue // 更新 i18n 的语言设置
  }
})

// 监听窗口位置变化
watch(windowPosition, (newValue: string) => {
  if (newValue) {
    setWindowPosition(newValue)
  }
})

// 监听记住窗口大小设置变化
watch(rememberSize, (newValue: boolean) => {
  setRememberSize(newValue)
})

// 监听窗口置顶设置变化
watch(alwaysOnTop, (newValue: boolean) => {
  setAlwaysOnTop(newValue)
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

/* 添加以下样式以确保标签不会换行 */
.el-form-item :deep(.el-form-item__label) {
    white-space: nowrap;
    min-width: 140px;
}

/* 确保内容区域有足够的空间 */
.el-form-item :deep(.el-form-item__content) {
    flex: 1;
    min-width: 200px;
}

/* 确保下拉菜单和开关的字体也相应缩小 */
.settings-form :deep(.el-select),
.settings-form :deep(.el-switch) {
    font-size: 14px;
}

/* .fi {
    width: 1.2em;
    height: 1.2em;
    border-radius: 2px;
} */

:deep(.el-select-dropdown__item) {
    padding: 0 20px;
}
</style>