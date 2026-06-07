import type { Locale } from '../model'

type AppCopy = {
  brandSubtitle: string
  nav: {
    overview: string
    extensions: string
    mcpServers: string
    plugins: string
    sources: string
    issues: string
    settings: string
  }
  dashboard: {
    eyebrow: string
    title: string
    scanButton: string
    languageButton: string
    summary: {
      totalExtensions: string
      needsReview: string
      mcpServers: string
      broken: string
      skills: string
      plugins: string
    }
    panelTitle: string
    panelBody: string
    phaseTag: string
  }
}

export const appCopy: Record<Locale, AppCopy> = {
  'zh-CN': {
    brandSubtitle: '本地扩展体检',
    nav: {
      overview: '概览',
      extensions: '扩展',
      mcpServers: 'MCP 服务',
      plugins: '插件',
      sources: '来源',
      issues: '问题',
      settings: '设置',
    },
    dashboard: {
      eyebrow: '只读 MVP 预览',
      title: 'Agent 扩展生态体检台',
      scanButton: '扫描本地来源',
      languageButton: 'English',
      summary: {
        totalExtensions: '扩展总数',
        needsReview: '需要复查',
        mcpServers: 'MCP 服务',
        broken: '已损坏',
        skills: 'Skills',
        plugins: '插件',
      },
      panelTitle: '正在使用示例扫描数据',
      panelBody:
        '当前界面已接入安全 fixture。下一步会把这些视图继续细化，再连接只读 scanner core。',
      phaseTag: '阶段 3：fixture UI',
    },
  },
  'en-US': {
    brandSubtitle: 'Local extension health',
    nav: {
      overview: 'Overview',
      extensions: 'Extensions',
      mcpServers: 'MCP Servers',
      plugins: 'Plugins',
      sources: 'Sources',
      issues: 'Issues',
      settings: 'Settings',
    },
    dashboard: {
      eyebrow: 'Read-only MVP preview',
      title: 'Agent extension health dashboard',
      scanButton: 'Scan local roots',
      languageButton: '中文',
      summary: {
        totalExtensions: 'Total extensions',
        needsReview: 'Needs review',
        mcpServers: 'MCP servers',
        broken: 'Broken',
        skills: 'Skills',
        plugins: 'Plugins',
      },
      panelTitle: 'Using fixture scan data',
      panelBody:
        'This shell is now wired to safe fixture data. Next, the views will be refined before connecting the read-only scanner core.',
      phaseTag: 'Phase 3: fixture UI',
    },
  },
}

export function nextLocale(locale: Locale): Locale {
  return locale === 'zh-CN' ? 'en-US' : 'zh-CN'
}
