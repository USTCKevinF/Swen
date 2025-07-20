// @ts-ignore - Vue import compatibility
import { ref } from 'vue'

export function useGetState(initialValue: any) {
  const state = ref(initialValue)
  
  // 获取最新状态的函数
  const getState = () => state.value
  
  // 设置状态的函数
  const setState = (newValue: any) => {
    state.value = newValue
  }
  
  return [state, setState, getState]
}