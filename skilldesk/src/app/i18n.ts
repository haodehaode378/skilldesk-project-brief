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
    scanningButton: string
    exportButton: string
    languageButton: string
    exportSuccess: string
    exportError: string
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
    localPanelTitle: string
    localPanelBody: string
    localPhaseTag: string
  }
  labels: {
    name: string
    kind: string
    platform: string
    source: string
    status: string
    issues: string
    path: string
    transport: string
    tools: string
    roots: string
    statusOk: string
    statusNeedsReview: string
    statusAtRisk: string
    statusBroken: string
    allStatuses: string
    emptyExtensions: string
    emptyMcp: string
    emptyPlugins: string
    emptySources: string
    emptyIssues: string
  }
  views: {
    extensionsTitle: string
    extensionsBody: string
    detailTitle: string
    mcpTitle: string
    pluginsTitle: string
    sourcesTitle: string
    issuesTitle: string
    settingsTitle: string
    settingsBody: string
    languageSetting: string
    pluginCacheMode: string
    mcpProbePolicy: string
    readOnlyNotice: string
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
      scanningButton: '正在扫描',
      exportButton: '导出报告',
      languageButton: 'English',
      exportSuccess: '报告已导出',
      exportError: '导出失败',
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
      localPanelTitle: '正在使用本机只读扫描数据',
      localPanelBody:
        '扫描器已读取默认 Skill 来源，只收集路径、数量、修改时间和基础元数据，不执行脚本或插件命令。',
      localPhaseTag: '阶段 4：本机扫描',
    },
    labels: {
      name: '名称',
      kind: '类型',
      platform: '平台',
      source: '来源',
      status: '状态',
      issues: '问题',
      path: '路径',
      transport: '传输',
      tools: '工具',
      roots: '扫描来源',
      statusOk: '正常',
      statusNeedsReview: '需要复查',
      statusAtRisk: '有风险',
      statusBroken: '已损坏',
      allStatuses: '全部状态',
      emptyExtensions: '没有发现符合条件的扩展。',
      emptyMcp: '没有发现 MCP 服务配置。',
      emptyPlugins: '没有发现插件 manifest。',
      emptySources: '没有扫描来源记录。',
      emptyIssues: '当前扫描没有发现问题。',
    },
    views: {
      extensionsTitle: '扩展清单',
      extensionsBody: '统一展示 Skill、命令、Agent、插件、MCP 和项目指令文件。',
      detailTitle: '详情',
      mcpTitle: 'MCP 服务',
      pluginsTitle: '插件',
      sourcesTitle: '来源',
      issuesTitle: '问题',
      settingsTitle: '设置',
      settingsBody: '界面语言和最近一次扫描报告会缓存在本机浏览器存储中。',
      languageSetting: '界面语言',
      pluginCacheMode: '插件缓存扫描',
      mcpProbePolicy: 'MCP 探测策略',
      readOnlyNotice: '只读模式：不会执行脚本、hooks、插件命令或 Agent 任务。',
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
      scanningButton: 'Scanning',
      exportButton: 'Export report',
      languageButton: '中文',
      exportSuccess: 'Report exported',
      exportError: 'Export failed',
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
      localPanelTitle: 'Using local read-only scan data',
      localPanelBody:
        'The scanner reads default skill sources and collects paths, counts, mtimes, and basic metadata without executing scripts or plugin commands.',
      localPhaseTag: 'Phase 4: local scan',
    },
    labels: {
      name: 'Name',
      kind: 'Kind',
      platform: 'Platform',
      source: 'Source',
      status: 'Status',
      issues: 'Issues',
      path: 'Path',
      transport: 'Transport',
      tools: 'Tools',
      roots: 'Scan roots',
      statusOk: 'OK',
      statusNeedsReview: 'Needs review',
      statusAtRisk: 'At risk',
      statusBroken: 'Broken',
      allStatuses: 'All statuses',
      emptyExtensions: 'No extensions match the current filters.',
      emptyMcp: 'No MCP server configurations were found.',
      emptyPlugins: 'No plugin manifests were found.',
      emptySources: 'No scan roots were recorded.',
      emptyIssues: 'No issues were found in the current scan.',
    },
    views: {
      extensionsTitle: 'Extension inventory',
      extensionsBody:
        'Unified view of skills, commands, agents, plugins, MCP servers, and instruction files.',
      detailTitle: 'Detail',
      mcpTitle: 'MCP Servers',
      pluginsTitle: 'Plugins',
      sourcesTitle: 'Sources',
      issuesTitle: 'Issues',
      settingsTitle: 'Settings',
      settingsBody:
        'Interface language and the latest scan report are cached in local browser storage.',
      languageSetting: 'Interface language',
      pluginCacheMode: 'Plugin cache scanning',
      mcpProbePolicy: 'MCP probe policy',
      readOnlyNotice:
        'Read-only mode: scripts, hooks, plugin commands, and agent tasks are never executed.',
    },
  },
}

export function nextLocale(locale: Locale): Locale {
  return locale === 'zh-CN' ? 'en-US' : 'zh-CN'
}
