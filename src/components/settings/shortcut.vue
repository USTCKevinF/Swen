<template>
  <div class="w-full">
    <div class="grid grid-cols-[repeat(auto-fit,minmax(300px,1fr))] gap-5">
      <div class="bg-white p-2">
        <div class="flex items-center gap-2 mb-4">
          <el-icon class="text-lg text-primary"><Pointer /></el-icon>
          <h3 class="m-0 text-[15px] font-medium text-gray-800">选中解释快捷键</h3>
        </div>
        <el-input
          v-model="selectionTranslate"
          placeholder="点击输入快捷键"
          @keydown="(e) => keyDown(e, setSelectionTranslate)"
          @focus="() => handleFocus(selectionTranslate, setSelectionTranslate)"
        >
          <template #append>
            <el-button
              type="primary"
              v-if="selectionTranslate"
              @click="registerHandler('hotkey_selection_get', selectionTranslate)"
            >
              保存
            </el-button>
          </template>
        </el-input>
      </div>

      <div class="bg-white p-2">
        <div class="flex items-center gap-2 mb-4">
          <el-icon class="text-lg text-primary"><Crop /></el-icon>
          <h3 class="m-0 text-[15px] font-medium text-gray-800">截图解释快捷键</h3>
        </div>
        <el-input
          v-model="inputTranslate"
          placeholder="点击输入快捷键"
          @keydown="(e) => keyDown(e, setInputTranslate)"
          @focus="() => handleFocus(inputTranslate, setInputTranslate)"
        >
          <template #append>
            <el-button
              type="primary"
              v-if="inputTranslate"
              @click="registerHandler('hotkey_input_translate', inputTranslate)"
            >
              保存
            </el-button>
          </template>
        </el-input>
      </div>
    </div>
  </div>
</template>

<script setup>
import { useConfig } from '../../composables/useConfig'
import { unregister, isRegistered } from '@tauri-apps/plugin-global-shortcut'
import { ElMessage } from 'element-plus'
import { invoke } from '@tauri-apps/api/core'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
const { Pointer, Crop } = ElementPlusIconsVue

const keyMap = {
  Backquote: '`',
  Backslash: '\\',
  BracketLeft: '[',
  BracketRight: ']',
  Comma: ',',
  Equal: '=',
  Minus: '-',
  Plus: 'PLUS',
  Period: '.',
  Quote: "'",
  Semicolon: ';',
  Slash: '/',
  Backspace: 'Backspace',
  CapsLock: 'Capslock',
  ContextMenu: 'Contextmenu',
  Space: 'Space',
  Tab: 'Tab',
  Convert: 'Convert',
  Delete: 'Delete',
  End: 'End',
  Help: 'Help',
  Home: 'Home',
  PageDown: 'Pagedown',
  PageUp: 'Pageup',
  Escape: 'Esc',
  PrintScreen: 'Printscreen',
  ScrollLock: 'Scrolllock',
  Pause: 'Pause',
  Insert: 'Insert',
  Suspend: 'Suspend'
}

const { property: selectionTranslate, setProperty: setSelectionTranslate } = useConfig('hotkey_selection_get', '')
const { property: inputTranslate, setProperty: setInputTranslate } = useConfig('hotkey_input_translate', '')

const keyDown = (e, setKey) => {
  e.preventDefault()
  if (e.keyCode === 8) {
    setKey('')
    return
  }

  let newValue = ''
  if (e.ctrlKey) {
    newValue = 'Ctrl'
  }
  if (e.shiftKey) {
    newValue = `${newValue}${newValue.length > 0 ? '+' : ''}Shift`
  }
  if (e.metaKey) {
    newValue = `${newValue}${newValue.length > 0 ? '+' : ''}Command`
  }
  if (e.altKey) {
    newValue = `${newValue}${newValue.length > 0 ? '+' : ''}Option`
  }

  let code = e.code
  if (code.startsWith('Key')) {
    code = code.substring(3)
  } else if (code.startsWith('Digit')) {
    code = code.substring(5)
  } else if (code.startsWith('Numpad')) {
    code = 'Num' + code.substring(6)
  } else if (code.startsWith('Arrow')) {
    code = code.substring(5)
  } else if (code.startsWith('Intl')) {
    code = code.substring(4)
  } else if (/F\d+/.test(code)) {
    // 保持F键不变
  } else if (keyMap[code] !== undefined) {
    code = keyMap[code]
  } else {
    code = ''
  }

  setKey(`${newValue}${newValue.length > 0 && code.length > 0 ? '+' : ''}${code}`)
}

const handleFocus = async (currentKey, setKey) => {
  if (currentKey) {
    await unregister(currentKey)
    setKey('')
  }
}

const registerHandler = async (name, key) => {
  try {
    const registered = await isRegistered(key)
    if (registered) {
      ElMessage.error('该快捷键已被注册')
      return
    }

    await invoke('register_shortcut_by_frontend', {
      name,
      shortcut: key
    })
    ElMessage.success('快捷键注册成功')
  } catch (error) {
    ElMessage.error(error.toString())
  }
}
</script>

<style>
.el-input :deep(.el-input__wrapper) {
  @apply shadow-[0_0_0_1px_#e4e4e4_inset];
}

.el-input :deep(.el-input__wrapper:hover) {
  @apply shadow-[0_0_0_1px_var(--el-color-primary)_inset];
}

.el-input :deep(.el-input__wrapper.is-focus) {
  @apply shadow-[0_0_0_1px_var(--el-color-primary)_inset];
}
</style>