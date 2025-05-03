<template>
  <Suspense>
    <NConfigProvider :theme="theme" :theme-overrides="themeOverrides">
      <NDialogProvider>
        <NMessageProvider>
          <NNotificationProvider>
            <NLoadingBarProvider>
              <AppContent />
            </NLoadingBarProvider>
          </NNotificationProvider>
        </NMessageProvider>
      </NDialogProvider>
    </NConfigProvider>
  </Suspense>
</template>

<script setup lang="ts">
import { ref, computed, provide, onMounted } from 'vue'
import {
  NConfigProvider,
  NMessageProvider,
  NDialogProvider,
  NNotificationProvider,
  NLoadingBarProvider,
  darkTheme
} from 'naive-ui'
import { themeOverrides } from './constants/theme'
import AppContent from './components/AppContent.vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

// 主题状态
const prefersDark = window.matchMedia('(prefers-color-scheme: dark)')
const isDarkMode = ref(prefersDark.matches)
const theme = computed(() => isDarkMode.value ? darkTheme : null)


// 监听系统主题变化
prefersDark.addEventListener('change', (e) => {
  isDarkMode.value = e.matches
})
const cleanupFunctions = ref<(() => void)[]>([]);

// 主题切换函数
const toggleTheme = () => {
  isDarkMode.value = !isDarkMode.value
}

// 提供给全局使用
provide('theme', {
  isDarkMode,
  theme,
  toggleTheme
})

// 禁用所有按键
const Debug = true
if (!Debug){
  document.addEventListener('keydown', (e) => {
    e.preventDefault()
  })
  document.addEventListener('contextmenu', (e) => {
     e.preventDefault() 
  })
  document.addEventListener('copy', (e) => {
    e.preventDefault() 
  })
  document.addEventListener('paste', (e) => {
    e.preventDefault()
  })
  document.addEventListener('cut', (e) => {
    e.preventDefault() 
  })
  //禁用开发者
  window.oncontextmenu = () => {
    return false 
  }
}

onMounted(async () => {
  try {
    const updateInfo = await invoke<{
      code: number;
      message: string | null;
      data: {
        current_version: string;
        latest_info: {
          data: any
          version: string;
          download_url: string;
          release_notes: string;
          force_update: boolean;
        };
        needs_update: boolean;
      };
    }>('check_update');

    if (updateInfo.data.current_version !== updateInfo.data.latest_info.data.latest_info.version) {
      (window as any).$notification?.info({
        title: `新版本 ${updateInfo.data.latest_info.data.latest_info.version} 可用`,
        content: updateInfo.data.latest_info.data.latest_info.release_notes,
        duration: 0, 
      })
      setTimeout( async () => {
        await invoke('emit_event', {
          event: 'log',
          payload: {
            message: `新版本 ${updateInfo.data.latest_info.data.latest_info.version} 可用`,
          }
        });
        appendLog(`新版本 ${updateInfo.data.latest_info.data.latest_info.version} 可用`)
      }, 500) 
    }
    console.log('检查更新成功:', updateInfo);
  } catch (error) {
    console.error('检查更新失败:', error);
  }
});
const checkFrpcHas = async () => {
  try {
    const hasFrpc = await invoke<boolean>('check_frpc_exists')
    if (!hasFrpc) {
      (window as any).$notification?.error({
        title: 'frpc.exe不存在',
        content: '请到系统设置下载frpc.exe',
        duration: 0, 
      })
      setTimeout( async () => {
        await invoke('emit_event', {
          event: 'log',
          payload: {
            message: `frpc.exe不存在，请到系统设置下载frpc.exe`,
          }
        });
        appendLog('frpc.exe不存在，请到系统设置中下载frpc.exe')
      }, 500)
    
    }
  } catch (error) {
    console.error('检查frpc.exe失败:', error)
  }
}

const appendLog = (message: string) => {
  const timestamp = new Date().toLocaleTimeString();
  const logMessage = `[${timestamp}] [系统] ${message}\n`;
  const savedLogs = localStorage.getItem('frpcLogs') || '';
  localStorage.setItem('frpcLogs', savedLogs + '\n\n' + logMessage);
};
onMounted(async () => {
  localStorage.setItem('frpcLogs', '');
  checkFrpcHas()
  try{
    const globalLogUnlisten = await listen('log', (event: any) => {
      appendLog(event.payload.message);
    });
    cleanupFunctions.value.push(globalLogUnlisten);
  }catch(error){
    console.error('监听日志失败:', error) 
  }
})
</script>

<style lang="scss">
@use "./assets/styles/transitions.scss";
input, textarea, select {
  font-size: 16px !important;
}

@media screen and (max-width: 768px) {
  input, textarea, select {
    font-size: 16px !important;
  }
}
</style>