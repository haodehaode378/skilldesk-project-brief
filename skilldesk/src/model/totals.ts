import type { HealthStatus, ManagedEntity, ScanTotals } from './types'

const statuses: HealthStatus[] = ['ok', 'needs-review', 'at-risk', 'broken']

export function createEmptyTotals(): ScanTotals {
  return {
    entities: 0,
    skills: 0,
    commands: 0,
    agents: 0,
    plugins: 0,
    mcpServers: 0,
    instructionFiles: 0,
    byStatus: {
      ok: 0,
      'needs-review': 0,
      'at-risk': 0,
      broken: 0,
    },
  }
}

export function calculateScanTotals(entities: ManagedEntity[]): ScanTotals {
  const totals = createEmptyTotals()

  for (const entity of entities) {
    totals.entities += 1
    totals.byStatus[entity.health.status] += 1

    switch (entity.kind) {
      case 'skill':
        totals.skills += 1
        break
      case 'command':
        totals.commands += 1
        break
      case 'agent':
        totals.agents += 1
        break
      case 'plugin':
        totals.plugins += 1
        break
      case 'mcp-server':
        totals.mcpServers += 1
        break
      case 'instruction-file':
        totals.instructionFiles += 1
        break
    }
  }

  for (const status of statuses) {
    totals.byStatus[status] ??= 0
  }

  return totals
}
