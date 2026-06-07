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
    clearCacheButton: string
    cacheCleared: string
    summary: {
      totalExtensions: string
      needsReview: string
      mcpServers: string
      broken: string
      skills: string
      commands: string
      agents: string
      plugins: string
      mojibake: string
      dirtyGitSources: string
    }
    panelTitle: string
    panelBody: string
    phaseTag: string
    localPanelTitle: string
    localPanelBody: string
    localPhaseTag: string
    reportSummaryTitle: string
    reportSource: string
    reportSourceFixture: string
    reportSourceCached: string
    reportSourceScanning: string
    reportSourceLocal: string
    reportSourceError: string
    generatedAt: string
    scanRoots: string
    machinePlatform: string
    schemaVersion: string
  }
  labels: {
    name: string
    kind: string
    kindSkill: string
    kindCommand: string
    kindAgent: string
    kindPlugin: string
    kindMcpServer: string
    kindInstructionFile: string
    platform: string
    source: string
    status: string
    severity: string
    issues: string
    path: string
    transport: string
    command: string
    args: string
    skillFile: string
    scripts: string
    references: string
    assets: string
    declaredVersion: string
    commandType: string
    namespace: string
    model: string
    declaredTools: string
    host: string
    tools: string
    version: string
    publisher: string
    cache: string
    backup: string
    yes: string
    no: string
    commands: string
    agents: string
    manifest: string
    appliesTo: string
    lines: string
    resources: string
    prompts: string
    probe: string
    probeAttempted: string
    probeSkipped: string
    reachable: string
    latency: string
    error: string
    file: string
    hooks: string
    mcpServers: string
    cacheFamily: string
    roots: string
    scanned: string
    missing: string
    skipped: string
    errors: string
    git: string
    gitRoot: string
    branch: string
    commit: string
    remote: string
    dirty: string
    clean: string
    enabled: string
    recommendation: string
    evidence: string
    statusOk: string
    statusNeedsReview: string
    statusAtRisk: string
    statusBroken: string
    allStatuses: string
    allKinds: string
    severityInfo: string
    severityLow: string
    severityMedium: string
    severityHigh: string
    allSeverities: string
    search: string
    searchIssues: string
    resultCount: string
    emptyExtensions: string
    emptyMcp: string
    emptyPlugins: string
    emptySources: string
    emptyInstructionFiles: string
    emptyIssues: string
    emptyFilteredIssues: string
  }
  views: {
    extensionsTitle: string
    extensionsBody: string
    detailTitle: string
    mcpTitle: string
    pluginsTitle: string
    sourcesTitle: string
    instructionFilesTitle: string
    issuesTitle: string
    settingsTitle: string
    settingsBody: string
    languageSetting: string
    pluginCacheMode: string
    mcpProbePolicy: string
    defaultScanRoots: string
    scanRootsHelp: string
    scanSafety: string
    scanSafetyBody: string
    scannerSelfCheck: string
    scannerCapabilities: string
    readOnlyScanning: string
    pluginCacheSummaryOnly: string
    mcpProbeDisabled: string
    mcpProbeLocalOnly: string
    mcpProbeAll: string
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
      clearCacheButton: '清除缓存报告',
      cacheCleared: '缓存报告已清除，当前显示示例数据。',
      summary: {
        totalExtensions: '扩展总数',
        needsReview: '需要复查',
        mcpServers: 'MCP 服务',
        broken: '已损坏',
        skills: 'Skills',
        commands: '命令',
        agents: 'Agents',
        plugins: '插件',
        mojibake: '疑似乱码',
        dirtyGitSources: 'Git 未提交来源',
      },
      panelTitle: '正在使用示例扫描数据',
      panelBody:
        '当前显示内置示例数据。点击扫描本地来源后，会切换为本机只读扫描报告。',
      phaseTag: '示例数据',
      localPanelTitle: '正在使用本机只读扫描数据',
      localPanelBody:
        '扫描器已读取默认 Skill 来源，只收集路径、数量、修改时间和基础元数据，不执行脚本或插件命令。',
      localPhaseTag: '本机只读扫描',
      reportSummaryTitle: '报告摘要',
      reportSource: '数据来源',
      reportSourceFixture: '示例数据',
      reportSourceCached: '本机缓存报告',
      reportSourceScanning: '正在扫描',
      reportSourceLocal: '刚完成的本机扫描',
      reportSourceError: '扫描失败，显示上次报告',
      generatedAt: '生成时间',
      scanRoots: '来源数量',
      machinePlatform: '平台',
      schemaVersion: 'Schema',
    },
    labels: {
      name: '名称',
      kind: '类型',
      kindSkill: 'Skill',
      kindCommand: '命令',
      kindAgent: 'Agent',
      kindPlugin: '插件',
      kindMcpServer: 'MCP 服务',
      kindInstructionFile: '项目指令',
      platform: '平台',
      source: '来源',
      status: '状态',
      severity: '严重级别',
      issues: '问题',
      path: '路径',
      transport: '传输',
      command: '命令',
      args: '参数',
      skillFile: 'Skill 文件',
      scripts: '脚本',
      references: '参考资料',
      assets: '资源文件',
      declaredVersion: '声明版本',
      commandType: '命令类型',
      namespace: '命名空间',
      model: '模型',
      declaredTools: '声明工具',
      host: 'Host',
      tools: '工具',
      version: '版本',
      publisher: '发布者',
      cache: '缓存',
      backup: '备份',
      yes: '是',
      no: '否',
      commands: '命令',
      agents: 'Agents',
      manifest: 'Manifest',
      appliesTo: '适用路径',
      lines: '行数',
      resources: '资源',
      prompts: 'Prompts',
      probe: '探测',
      probeAttempted: '已记录',
      probeSkipped: '未主动探测',
      reachable: '可连接',
      latency: '延迟',
      error: '错误',
      file: '文件',
      hooks: 'Hooks',
      mcpServers: 'MCP 服务',
      cacheFamily: '缓存族',
      roots: '扫描来源',
      scanned: '已扫描',
      missing: '缺失',
      skipped: '已跳过',
      errors: '出错',
      git: 'Git',
      gitRoot: '仓库',
      branch: '分支',
      commit: '提交',
      remote: '远程',
      dirty: '有未提交变化',
      clean: '干净',
      enabled: '已启用',
      recommendation: '建议',
      evidence: '证据',
      statusOk: '正常',
      statusNeedsReview: '需要复查',
      statusAtRisk: '有风险',
      statusBroken: '已损坏',
      allStatuses: '全部状态',
      allKinds: '全部类型',
      severityInfo: '提示',
      severityLow: '低',
      severityMedium: '中',
      severityHigh: '高',
      allSeverities: '全部级别',
      search: '搜索扩展',
      searchIssues: '搜索问题',
      resultCount: '显示 {shown} / {total}',
      emptyExtensions: '没有发现符合条件的扩展。',
      emptyMcp: '没有发现 MCP 服务配置。',
      emptyPlugins: '没有发现插件 manifest。',
      emptySources: '没有扫描来源记录。',
      emptyInstructionFiles: '没有发现项目指令文件。',
      emptyIssues: '当前扫描没有发现问题。',
      emptyFilteredIssues: '没有发现符合条件的问题。',
    },
    views: {
      extensionsTitle: '扩展清单',
      extensionsBody: '统一展示 Skill、命令、Agent、插件、MCP 和项目指令文件。',
      detailTitle: '详情',
      mcpTitle: 'MCP 服务',
      pluginsTitle: '插件',
      sourcesTitle: '来源',
      instructionFilesTitle: '项目指令文件',
      issuesTitle: '问题',
      settingsTitle: '设置',
      settingsBody: '界面语言和最近一次扫描报告会缓存在本机浏览器存储中。',
      languageSetting: '界面语言',
      pluginCacheMode: '插件缓存扫描',
      mcpProbePolicy: 'MCP 探测策略',
      defaultScanRoots: '默认扫描范围',
      scanRootsHelp: '每行一个扫描根。仅支持 MVP 明确的 Skill、命令、Agent、插件、MCP 配置和项目指令文件来源。',
      scanSafety: '扫描安全',
      scanSafetyBody:
        '当前版本只读取文件元数据和安全摘要，不执行脚本、hooks、插件命令或 Agent 任务。',
      scannerSelfCheck: '扫描器自检',
      scannerCapabilities: '扫描能力',
      readOnlyScanning: '只读扫描',
      pluginCacheSummaryOnly: '仅汇总 manifest 和内置资源数量',
      mcpProbeDisabled: '已禁用主动连接探测',
      mcpProbeLocalOnly: '仅允许本地 MCP 轻量探测',
      mcpProbeAll: '允许本地和远程 MCP 轻量探测',
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
      clearCacheButton: 'Clear cached report',
      cacheCleared: 'Cached report cleared. Fixture data is now shown.',
      summary: {
        totalExtensions: 'Total extensions',
        needsReview: 'Needs review',
        mcpServers: 'MCP servers',
        broken: 'Broken',
        skills: 'Skills',
        commands: 'Commands',
        agents: 'Agents',
        plugins: 'Plugins',
        mojibake: 'Possible mojibake',
        dirtyGitSources: 'Dirty Git sources',
      },
      panelTitle: 'Using fixture scan data',
      panelBody:
        'The built-in fixture report is currently shown. Scan local roots to switch to a read-only local report.',
      phaseTag: 'Fixture data',
      localPanelTitle: 'Using local read-only scan data',
      localPanelBody:
        'The scanner reads default skill sources and collects paths, counts, mtimes, and basic metadata without executing scripts or plugin commands.',
      localPhaseTag: 'Local read-only scan',
      reportSummaryTitle: 'Report summary',
      reportSource: 'Data source',
      reportSourceFixture: 'Fixture data',
      reportSourceCached: 'Cached local report',
      reportSourceScanning: 'Scanning',
      reportSourceLocal: 'Fresh local scan',
      reportSourceError: 'Scan failed, showing previous report',
      generatedAt: 'Generated at',
      scanRoots: 'Scan roots',
      machinePlatform: 'Platform',
      schemaVersion: 'Schema',
    },
    labels: {
      name: 'Name',
      kind: 'Kind',
      kindSkill: 'Skill',
      kindCommand: 'Command',
      kindAgent: 'Agent',
      kindPlugin: 'Plugin',
      kindMcpServer: 'MCP server',
      kindInstructionFile: 'Instruction file',
      platform: 'Platform',
      source: 'Source',
      status: 'Status',
      severity: 'Severity',
      issues: 'Issues',
      path: 'Path',
      transport: 'Transport',
      command: 'Command',
      args: 'Args',
      skillFile: 'Skill file',
      scripts: 'Scripts',
      references: 'References',
      assets: 'Assets',
      declaredVersion: 'Declared version',
      commandType: 'Command type',
      namespace: 'Namespace',
      model: 'Model',
      declaredTools: 'Declared tools',
      host: 'Host',
      tools: 'Tools',
      version: 'Version',
      publisher: 'Publisher',
      cache: 'Cache',
      backup: 'Backup',
      yes: 'Yes',
      no: 'No',
      commands: 'Commands',
      agents: 'Agents',
      manifest: 'Manifest',
      appliesTo: 'Applies to',
      lines: 'Lines',
      resources: 'Resources',
      prompts: 'Prompts',
      probe: 'Probe',
      probeAttempted: 'Recorded',
      probeSkipped: 'Not actively probed',
      reachable: 'Reachable',
      latency: 'Latency',
      error: 'Error',
      file: 'File',
      hooks: 'Hooks',
      mcpServers: 'MCP servers',
      cacheFamily: 'Cache family',
      roots: 'Scan roots',
      scanned: 'Scanned',
      missing: 'Missing',
      skipped: 'Skipped',
      errors: 'Errors',
      git: 'Git',
      gitRoot: 'Repository',
      branch: 'Branch',
      commit: 'Commit',
      remote: 'Remote',
      dirty: 'Dirty',
      clean: 'Clean',
      enabled: 'Enabled',
      recommendation: 'Recommendation',
      evidence: 'Evidence',
      statusOk: 'OK',
      statusNeedsReview: 'Needs review',
      statusAtRisk: 'At risk',
      statusBroken: 'Broken',
      allStatuses: 'All statuses',
      allKinds: 'All kinds',
      severityInfo: 'Info',
      severityLow: 'Low',
      severityMedium: 'Medium',
      severityHigh: 'High',
      allSeverities: 'All severities',
      search: 'Search extensions',
      searchIssues: 'Search issues',
      resultCount: 'Showing {shown} / {total}',
      emptyExtensions: 'No extensions match the current filters.',
      emptyMcp: 'No MCP server configurations were found.',
      emptyPlugins: 'No plugin manifests were found.',
      emptySources: 'No scan roots were recorded.',
      emptyInstructionFiles: 'No project instruction files were found.',
      emptyIssues: 'No issues were found in the current scan.',
      emptyFilteredIssues: 'No issues match the current filters.',
    },
    views: {
      extensionsTitle: 'Extension inventory',
      extensionsBody:
        'Unified view of skills, commands, agents, plugins, MCP servers, and instruction files.',
      detailTitle: 'Detail',
      mcpTitle: 'MCP Servers',
      pluginsTitle: 'Plugins',
      sourcesTitle: 'Sources',
      instructionFilesTitle: 'Project instruction files',
      issuesTitle: 'Issues',
      settingsTitle: 'Settings',
      settingsBody:
        'Interface language and the latest scan report are cached in local browser storage.',
      languageSetting: 'Interface language',
      pluginCacheMode: 'Plugin cache scanning',
      mcpProbePolicy: 'MCP probe policy',
      defaultScanRoots: 'Default scan roots',
      scanRootsHelp:
        'One scan root per line. Only MVP sources for skills, commands, agents, plugins, MCP configs, and instruction files are supported.',
      scanSafety: 'Scan safety',
      scanSafetyBody:
        'This version only reads file metadata and safe summaries. It never executes scripts, hooks, plugin commands, or agent tasks.',
      scannerSelfCheck: 'Scanner self-check',
      scannerCapabilities: 'Scanner capabilities',
      readOnlyScanning: 'Read-only scanning',
      pluginCacheSummaryOnly: 'Manifest and bundled resource counts only',
      mcpProbeDisabled: 'Active connection probing is disabled',
      mcpProbeLocalOnly: 'Allow local MCP lightweight probes only',
      mcpProbeAll: 'Allow local and remote MCP lightweight probes',
      readOnlyNotice:
        'Read-only mode: scripts, hooks, plugin commands, and agent tasks are never executed.',
    },
  },
}

export function nextLocale(locale: Locale): Locale {
  return locale === 'zh-CN' ? 'en-US' : 'zh-CN'
}
