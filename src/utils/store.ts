import { Store } from '@tauri-apps/plugin-store';
import { appConfigDir, join } from '@tauri-apps/api/path';
import { watch } from '@tauri-apps/plugin-fs';
import { invoke } from '@tauri-apps/api/core';
import { load } from '@tauri-apps/plugin-store';
// 使用 null 初始化
export let store: Store | null = null;
export async function initStore(): Promise<void> {
    const appConfigDirPath: string = await appConfigDir();
    const appConfigPath: string = await join(appConfigDirPath, 'config.json');
    store = await load(appConfigPath, { autoSave: false });
    await watch(appConfigPath, async () => {
        if (store) {
            await load(appConfigPath, { autoSave: false });
            await invoke('reload_store');
        }
    });
} 