<template>
  <!-- PC端导航栏 -->
  <NLayoutHeader bordered class="navbar pc-navbar" style="user-select: none" data-tauri-drag-region>
    <div class="navbar-content">
      <div class="logo" style="margin-left: 20px;">
        <RouterLink to="/" class="logo-link">
          <h2>{{ packageData.title }}</h2>
        </RouterLink>
      </div>

      <!-- 桌面端菜单 -->
      <div class="nav-links">
        <NSpace class="desktop-menu">
          <NSwitch size="small" style="margin-right: 15px;" :value="isDarkMode" @update:value="toggleTheme" :rail-style="switchButtonRailStyle">
            <template #checked>
              <NIcon :component="Moon" />
            </template>
            <template #unchecked>
              <NIcon :component="Sunny" />
            </template>
          </NSwitch>
          <div class="control" style="margin-right: 0px; transform: translateY(2.5px);">
            <NButton text @click="handleToRefresh">
          <NIcon size="28" style="margin-right: 15px;">
            <RefreshOutline />
          </NIcon>
         </NButton>
         <NButton text @click="handleToMinimize">
          <NIcon size="28" style="margin-right: 22px;">
            <RemoveOutline />
          </NIcon>
         </NButton text>
         <NButton text style="margin-right: 12px;" @click="handleToMaximize">
          <NIcon size="27">
            <ScanOutline />
          </NIcon>
         </NButton>
          <NButton text @click="ToClose = true">
            <NIcon size="28">
              <CloseOutline />
            </NIcon>
          </NButton>
        </div>
        </NSpace>
      </div>
    </div>
  </NLayoutHeader>

  <!-- 移动端导航栏 -->
  <NLayoutHeader bordered class="navbar mobile-navbar" style="user-select: none">
    <div class="mobile-header">
      <NPopover trigger="click" placement="bottom-start" :show="showMenu" @update:show="showMenu = $event">
        <template #trigger>
          <NButton text class="menu-button">
            <NIcon size="24">
              <MenuOutline />
            </NIcon>
          </NButton>
        </template>
        <div class="mobile-menu">
          <NMenu :options="menuOptions"  @update:value="handleMenuSelect" />
        </div>
      </NPopover>
      <div class="logo">
        <RouterLink to="/" class="logo-link">
          <h2>{{ packageData.title }}</h2>
        </RouterLink>
      </div>
    </div>
  </NLayoutHeader>
  <NModal v-model:show="ToClose" preset="dialog"  style="width: 400px">
      <template #header>
        你确定要关闭吗?
      </template>
        你也可以同样点击右上角的X图标来关闭当前弹窗。
        <br>
      <template #action>
        <NButton size="small" @click="handleToClose">确定</NButton>
        <NButton size="small" type="primary" @click="handleToCloseToPanel">最小化托盘</NButton>
      </template>
  </NModal>
</template>

<script setup lang="ts">
import packageData from '../../package.json'
import { h, inject, Ref, ref } from 'vue'
import { useRouter, RouterLink } from 'vue-router'
import { NLayoutHeader, NButton, NSpace, NSwitch, NIcon, NPopover, NMenu, MenuOption, NModal } from 'naive-ui'
import { MenuOutline, Moon, RefreshOutline, Sunny, RemoveOutline, ScanOutline, CloseOutline } from '@vicons/ionicons5'
import {
  HomeOutline,
  DocumentTextOutline,
  LogInOutline,
  PersonAddOutline,
  InformationCircleOutline,
  ShieldCheckmarkOutline,
  DocumentLockOutline
} from '@vicons/ionicons5'
import { switchButtonRailStyle } from '../constants/theme'
import { invoke } from '@tauri-apps/api/core'

const showMenu = ref(false)
const router = useRouter()
const ToClose = ref(false)
const { isDarkMode, toggleTheme } = inject('theme', {
  isDarkMode: ref(false),
  toggleTheme: () => { }
}) as {
  isDarkMode: Ref<boolean>
  toggleTheme: () => void
}

const handleToRefresh = () => {
  router.go(0)
}

const handleToClose = async () => {
  await invoke('quit_window');
}

const handleToMinimize = async () => {
  await invoke('minimize_window');
}

const handleToMaximize = async () => {
  await invoke('toggle_maximize');
}


const handleToCloseToPanel = async () => {
  ToClose.value = false
  new Notification('FRP客户端', {
    body: 'FRP客户端已最小化到托盘',
    silent: true,
  })
  await invoke('hide_to_tray');
}

function renderIcon(icon: any) {
  return () => h(NIcon, null, { default: () => h(icon) })
}

const menuOptions: MenuOption[] = [
  {
    label: '首页',
    key: 'home',
    icon: renderIcon(HomeOutline)
  },
  {
    label: '文档',
    key: 'docs',
    icon: renderIcon(DocumentTextOutline)
  },
  {
    type: 'divider',
    key: 'd1'
  },
  {
    label: '登录',
    key: 'login',
    icon: renderIcon(LogInOutline)
  },
  {
    label: '注册',
    key: 'register',
    icon: renderIcon(PersonAddOutline)
  },
  {
    type: 'divider',
    key: 'd2'
  },
  {
    label: '隐私政策',
    key: 'privacy',
    icon: renderIcon(ShieldCheckmarkOutline)
  },
  {
    label: '内容策略',
    key: 'content',
    icon: renderIcon(DocumentLockOutline)
  },
  {
    label: '服务条款',
    key: 'terms',
    icon: renderIcon(InformationCircleOutline)
  },
]

function handleMenuSelect(key: string) {
  switch (key) {
    case 'home':
      router.push('/')
      break
    case 'docs':
      router.push('/docs')
      break
    case 'login':
      router.push('/auth/login')
      break
    case 'register':
      router.push('/auth/register')
      break
    case 'privacy':
      router.push('/privacy')
      break
    case 'content':
      router.push('/content')
      break
    case 'terms':
      router.push('/terms')
      break
    case 'contact':
      router.push('/contact')
      break
  }
}
</script>

<style lang="scss" scoped>
@use '../assets/styles/components/homeMenu.scss' as *;
.navbar {
  -webkit-app-region: drag; /* 添加这一行启用拖动 */
}
/* 确保表单元素不被拖动区域覆盖 */
.n-form, .n-button {
  -webkit-app-region: no-drag;
}
</style>
