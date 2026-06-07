import {
  defaultAppSettings,
  localeSchema,
  scanReportSchema,
  skillDeskCacheSchema,
  type Locale,
  type ScanReport,
  type SkillDeskCache,
} from '../model'

const localeKey = 'skilldesk.locale'
const cacheKey = 'skilldesk.lastReport'

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
