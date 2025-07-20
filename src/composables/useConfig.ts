// @ts-ignore - Vue import compatibility
import { onMounted, onUnmounted, ref } from 'vue'
import { emit, listen } from '@tauri-apps/api/event'
import { store } from '../utils/store'
import { useGetState } from './useGetState'
import { debounce } from '../utils/debounce'
import { logger } from '../utils/logger'

export function useConfig(key: string, defaultValue: any, options = { sync: true }) {
  const [property, setPropertyState, getProperty] = useGetState(null)
  const { sync = true } = options
  const isLoaded = ref(false)

  // 同步到 Store (State -> Store)
  const syncToStore = debounce(async (value: any) => {
    try {
      if (!store) {
        logger.error('Store is not initialized');
        return
      }
      await store.set(key, value)
      await store.save()
      // 发送事件通知其他组件
      const eventKey = key.replaceAll('.', '_').replaceAll('@', ':')
      await emit(`${eventKey}_changed`, value)
      logger.debug('Synced to store', { key, value });
    } catch (err) {
      logger.error('Failed to sync to store', err);
    }
  }, 300)

  // 同步到 State (Store -> State) 
  const syncToState = async (value: any) => {
    if (value !== null) {
      setPropertyState(value)
      isLoaded.value = true
      logger.debug('Synced state from external value', { key, value });
    } else {
      try {
        if (!store) {
          logger.error('Store is not initialized');
          setPropertyState(defaultValue)
          isLoaded.value = true
          return
        }
        const storeValue = await store.get(key)
        if (storeValue === null || storeValue === undefined) {
          logger.debug('Using default value for key', { key, defaultValue });
          setPropertyState(defaultValue)
          await store.set(key, defaultValue)
          await store.save()
        } else {
          logger.debug('Loaded value from store', { key, storeValue });
          setPropertyState(storeValue)
        }
        isLoaded.value = true
      } catch (err) {
        logger.error('Failed to sync from store', err);
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
    if (!store) {
      logger.error('Store is not initialized');
      return
    }
    const hasKey = await store.has(key)
    if (hasKey) {
      await store.delete(key)
      await store.save()
    }
  } catch (err) {
    logger.error('Failed to delete key', err);
  }
}

