import { h, Component, ref } from 'vue'
import { HomeOutline, AddCircleOutline, AppsOutline, IdCardOutline,  WalletOutline, SettingsOutline } from '@vicons/ionicons5'
import { NIcon, type MenuOption } from 'naive-ui'
import { SquareTerminal } from 'lucide-vue-next';

const baseMenuOptions: MenuOption[] = [
  {
    label: '面板首页',
    icon: renderIcon(HomeOutline),
    key: 'dashboardIndex',
    link: '/dashboard/home',
  },
  {
    label: '创建隧道',
    icon: renderIcon(AddCircleOutline),
    key: 'create-tunnel',
    link: '/dashboard/proxy/create',
  },
  {
    label: '隧道管理',
    icon: renderIcon(AppsOutline),
    key: 'proxy-list',
    link: '/dashboard/proxy/list',
  },
  {
    label: '用户中心',
    icon: renderIcon(IdCardOutline),
    key: 'user-section',
    children: [
      {
        label: '个人中心',
        key: 'user-profile',
        link: '/dashboard/user/profile',
      },
        {
          label: '我的资料',
          key: 'user-my-profile',
          link: '/dashboard/user/my-profile',
        },
    ],
  },
  {
    label: '增值服务',
    key: 'cash',
    icon: renderIcon(WalletOutline),
    link: '/dashboard/cash',
  },
  {
    label: '日志',
    icon: renderIcon(SquareTerminal),
    key: 'logs',
    link: '/dashboard/logs',
  },
  {
    label: '系统设置',
    icon: renderIcon(SettingsOutline),
    key:'settings',
    link: '/dashboard/settings',
  },
]
export function getMenuOptions(): MenuOption[] {
  const userGroup = localStorage.getItem('group')
  const options = [...baseMenuOptions]
  return options
}

export function renderIcon(icon: Component) {
  return () => h(NIcon, {
    component: icon,
    size: 22
  })
}

export const defaultExpandedKeys = ref<string[]>(['more'])