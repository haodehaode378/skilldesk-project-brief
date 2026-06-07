import type { AppSettings } from './types'

export const defaultAppSettings: AppSettings = {
  locale: 'zh-CN',
  scanRoots: [
    '%USERPROFILE%\\.codex\\skills',
    '%USERPROFILE%\\.agents\\skills',
    '%USERPROFILE%\\.claude\\skills',
    '%USERPROFILE%\\.claude\\commands',
    '%USERPROFILE%\\.claude\\agents',
    '%USERPROFILE%\\.codex\\plugins',
    '%USERPROFILE%\\.claude\\plugins',
    '%USERPROFILE%\\.codex\\config.toml',
    '%USERPROFILE%\\.claude\\mcp-configs',
  ],
  includePluginCaches: false,
  mcpProbePolicy: 'disabled',
}
