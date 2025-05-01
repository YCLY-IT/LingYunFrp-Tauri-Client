<template>
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
</template>

<script setup lang="ts">
import { ref, computed, provide } from 'vue'
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

// 主题状态
const prefersDark = window.matchMedia('(prefers-color-scheme: dark)')
const isDarkMode = ref(prefersDark.matches)
const theme = computed(() => isDarkMode.value ? darkTheme : null)

// 监听系统主题变化
prefersDark.addEventListener('change', (e) => {
  isDarkMode.value = e.matches
})

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
  window.onload = function() {
      document.onkeydown = function() {
          var e = window.event || arguments[0];
          //屏蔽F12：按键码123
          if(e.keyCode == 123) {
              console.log('暂时不支持开发者模式！！');
              return false;
          //屏蔽Ctrl+Shift+I
          }else if((e.ctrlKey) && (e.shiftKey) && (e.keyCode == 73)){
              console.log('当前提示，本网站禁止审查元素');
              return false;
          //屏蔽Ctrl+U(火狐下查看网页源代码快捷键)
          }else if((e.ctrlKey) && (e.keyCode == 85)){
              console.log('本网站禁止使用审查元素')
              return false;
          //屏蔽Shift+F10
          }else if((e.shiftKey) && (e.keyCode == 121)){
              console.log('本网站禁止审查元素！');
              return false;
          //禁止保存页面：ctrl + s
          }else if(event.ctrlKey  &&  window.event.keyCode==83 ){ 
              console.log('本网站禁止保存文件！');
              return false;
          }
      };
      //屏蔽右键单击
      if (window.Event)
          document.captureEvents(Event.MOUSEUP);
      function nocontextmenu()
      {
          event.cancelBubble = true
          event.returnValue = false;
          return false;
      }
      function norightclick(e)
      {if (window.Event)
      {
          if (e.which == 2 || e.which == 3)
              return false;
      }
      else
          if (event.button == 2 || event.button == 3)
          {
              event.cancelBubble = true
              event.returnValue = false;
              return false;
          }
      }
      document.oncontextmenu = nocontextmenu; // for IE5+
      document.onmousedown = norightclick; // for all others
  }
}
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