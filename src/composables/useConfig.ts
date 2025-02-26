import { onMounted, onUnmounted } from 'vue'
import { emit, listen } from '@tauri-apps/api/event'
import { store } from '../utils/store'
import { useGetState } from './useGetState'
import { debounce } from '../utils/debounce'
import { ref } from 'vue'

export function useConfig(key: string, defaultValue: any, options = { sync: true }) {
  const [property, setPropertyState, getProperty] = useGetState(null)
  const { sync = true } = options
  const isLoaded = ref(false)

  // 同步到 Store (State -> Store)
  const syncToStore = debounce(async (value: any) => {
    try {
      console.log('syncToStore:', store)
      await store.set(key, value)
      await store.save()
      // 发送事件通知其他组件
      const eventKey = key.replaceAll('.', '_').replaceAll('@', ':')
      await emit(`${eventKey}_changed`, value)
    } catch (err) {
      console.error('Failed to sync to store:', err)
    }
  }, 300)

  // 同步到 State (Store -> State) 
  const syncToState = async (value: any) => {
    console.log('syncToState:', value)
    if (value !== null) {
      setPropertyState(value)
      isLoaded.value = true
    } else {
      try {
        const storeValue = await store.get(key)
        if (storeValue === null || storeValue === undefined) {
          console.log('storeValue is null', defaultValue)
          setPropertyState(defaultValue)
          await store.set(key, defaultValue)
          await store.save()
        } else {
          console.log('storeValue is not null', storeValue)
          setPropertyState(storeValue)
        }
        isLoaded.value = true
      } catch (err) {
        console.error('Failed to sync from store:', err)
      }
    }
  }

  // 设置属性值
  const setProperty = (value: any, forceSync = false) => {
    setPropertyState(value)
    const shouldSync = forceSync || sync
    if (shouldSync) {
      syncToStore(value)
    }
  }

  // 生命周期钩子
  onMounted(async () => {
    await syncToState(null)
    const eventKey = key.replaceAll('.', '_').replaceAll('@', ':')
    const unlisten = await listen(`${eventKey}_changed`, (event: any) => {
      syncToState(event.payload)
    })

    // 清理事件监听
    onUnmounted(() => {
      unlisten()
    })
  })

  return {
    property,
    setProperty,
    getProperty,
    isLoaded
  }
}

// 删除配置项
export const deleteKey = async (key: string) => {
  try {
    const hasKey = await store.has(key)
    if (hasKey) {
      await store.delete(key)
      await store.save()
    }
  } catch (err) {
    console.error('Failed to delete key:', err)
  }
}

