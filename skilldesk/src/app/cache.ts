import {
  defaultAppSettings,
  appSettingsSchema,
  localeSchema,
  scanReportSchema,
  skillDeskCacheSchema,
  type AppSettings,
  type Locale,
  type ScanReport,
  type SkillDeskCache,
} from '../model'
import {
  defaultViewPreferences,
  type ViewPreferences,
} from '../features/viewState'

const localeKey = 'skilldesk.locale'
const cacheKey = 'skilldesk.lastReport'
const settingsKey = 'skilldesk.settings'
const viewPreferencesKey = 'skilldesk.viewPreferences'

type StorageLike = Pick<Storage, 'getItem' | 'setItem' | 'removeItem'>

function safeStorage(): StorageLike | undefined {
  if (typeof window === 'undefined') {
    return undefined
  }

  try {
    return window.localStorage
  } catch {
    return undefined
  }
}

export function loadLocale(storage = safeStorage()): Locale {
  if (!storage) {
    return defaultAppSettings.locale
  }

  try {
    return localeSchema.parse(storage.getItem(localeKey))
  } catch {
    return defaultAppSettings.locale
  }
}

export function saveLocale(locale: Locale, storage = safeStorage()) {
  try {
    storage?.setItem(localeKey, locale)
  } catch {
    // Ignore storage failures; settings still work for the current session.
  }
}

export function loadAppSettings(storage = safeStorage()): AppSettings {
  const rawSettings = storage?.getItem(settingsKey)
  if (!rawSettings) {
    return defaultAppSettings
  }

  try {
    return appSettingsSchema.parse({
      ...defaultAppSettings,
      ...JSON.parse(rawSettings),
    })
  } catch {
    try {
      storage?.removeItem(settingsKey)
    } catch {
      // Ignore storage failures; invalid settings simply won't be used.
    }
    return defaultAppSettings
  }
}

export function saveAppSettings(settings: AppSettings, storage = safeStorage()) {
  try {
    storage?.setItem(settingsKey, JSON.stringify(appSettingsSchema.parse(settings)))
  } catch {
    // Ignore storage failures; settings still work for the current session.
  }
}

function parseViewPreferences(value: unknown): ViewPreferences {
  const parsed =
    value && typeof value === 'object'
      ? (value as Partial<ViewPreferences>)
      : {}
  const extensions: Partial<ViewPreferences['extensions']> =
    parsed.extensions && typeof parsed.extensions === 'object'
      ? (parsed.extensions as Partial<ViewPreferences['extensions']>)
      : {}
  const issues: Partial<ViewPreferences['issues']> =
    parsed.issues && typeof parsed.issues === 'object'
      ? (parsed.issues as Partial<ViewPreferences['issues']>)
      : {}

  const statusFilterValues = [
    'all',
    'ok',
    'needs-review',
    'at-risk',
    'broken',
  ] as const
  const kindFilterValues = [
    'all',
    'skill',
    'command',
    'agent',
    'plugin',
    'mcp-server',
    'instruction-file',
  ] as const
  const severityFilterValues = ['all', 'info', 'low', 'medium', 'high'] as const
  const sortKeyValues = ['health', 'name', 'kind', 'platform', 'issues'] as const
  const sortDirectionValues = ['asc', 'desc'] as const

  return {
    extensions: {
      statusFilter: statusFilterValues.includes(extensions.statusFilter as never)
        ? (extensions.statusFilter as ViewPreferences['extensions']['statusFilter'])
        : defaultViewPreferences.extensions.statusFilter,
      kindFilter: kindFilterValues.includes(extensions.kindFilter as never)
        ? (extensions.kindFilter as ViewPreferences['extensions']['kindFilter'])
        : defaultViewPreferences.extensions.kindFilter,
      query:
        typeof extensions.query === 'string'
          ? extensions.query
          : defaultViewPreferences.extensions.query,
      sortKey: sortKeyValues.includes(extensions.sortKey as never)
        ? (extensions.sortKey as ViewPreferences['extensions']['sortKey'])
        : defaultViewPreferences.extensions.sortKey,
      sortDirection: sortDirectionValues.includes(
        extensions.sortDirection as never,
      )
        ? (extensions.sortDirection as ViewPreferences['extensions']['sortDirection'])
        : defaultViewPreferences.extensions.sortDirection,
    },
    issues: {
      severityFilter: severityFilterValues.includes(
        issues.severityFilter as never,
      )
        ? (issues.severityFilter as ViewPreferences['issues']['severityFilter'])
        : defaultViewPreferences.issues.severityFilter,
      query:
        typeof issues.query === 'string'
          ? issues.query
          : defaultViewPreferences.issues.query,
    },
  }
}

export function loadViewPreferences(storage = safeStorage()): ViewPreferences {
  const rawPreferences = storage?.getItem(viewPreferencesKey)
  if (!rawPreferences) {
    return defaultViewPreferences
  }

  try {
    return parseViewPreferences(JSON.parse(rawPreferences))
  } catch {
    try {
      storage?.removeItem(viewPreferencesKey)
    } catch {
      // Ignore storage failures; invalid preferences simply won't be used.
    }
    return defaultViewPreferences
  }
}

export function saveViewPreferences(
  preferences: ViewPreferences,
  storage = safeStorage(),
) {
  try {
    storage?.setItem(
      viewPreferencesKey,
      JSON.stringify(parseViewPreferences(preferences)),
    )
  } catch {
    // Ignore storage failures; preferences still work for the current session.
  }
}

export function loadCachedReport(storage = safeStorage()): ScanReport | undefined {
  const rawCache = storage?.getItem(cacheKey)
  if (!rawCache) {
    return undefined
  }

  try {
    const parsedCache = skillDeskCacheSchema.parse(JSON.parse(rawCache))
    return parsedCache.lastReport
  } catch {
    try {
      storage?.removeItem(cacheKey)
    } catch {
      // Ignore storage failures; invalid cache simply won't be used.
    }
    return undefined
  }
}

export function saveCachedReport(report: ScanReport, storage = safeStorage()) {
  const parsedReport = scanReportSchema.parse(report)
  const cache: SkillDeskCache = {
    cacheVersion: '0.1',
    lastReport: parsedReport,
  }

  try {
    storage?.setItem(cacheKey, JSON.stringify(cache))
  } catch {
    // Ignore storage failures; the live scan result remains visible.
  }
}

export function clearCachedReport(storage = safeStorage()) {
  try {
    storage?.removeItem(cacheKey)
  } catch {
    // Ignore storage failures; the current session can still reset in memory.
  }
}
