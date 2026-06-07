import { describe, expect, it } from 'vitest'

import { scanReportSchema } from '../model'

describe('scanReportSchema', () => {
  it('accepts a local scanner shaped skill report', () => {
    const report = scanReportSchema.parse({
      schemaVersion: '0.1',
      generatedAt: '2026-06-07T09:59:00Z',
      machine: {
        platform: 'win32',
        homeDir: 'C:\\Users\\example',
      },
      roots: [
        {
          path: 'C:\\Users\\example\\.codex\\skills',
          kind: 'directory',
          status: 'scanned',
        },
      ],
      entities: [
        {
          id: 'skill:codex:c--users-example--codex-skills-demo',
          kind: 'skill',
          platform: 'codex',
          name: 'demo',
          title: 'Demo Skill',
          description: 'Use when testing the local scanner schema contract.',
          path: 'C:\\Users\\example\\.codex\\skills\\demo',
          source: 'local',
          tags: [],
          discoveredAt: '2026-06-07T09:59:00Z',
          lastModified: '2026-06-07T09:58:00Z',
          fingerprint: 'size:120:mtime:1780819080',
          health: {
            status: 'ok',
            issues: [],
          },
          files: {
            skillMd: 'C:\\Users\\example\\.codex\\skills\\demo\\SKILL.md',
            scripts: [],
            references: [],
            assets: [],
          },
        },
      ],
      totals: {
        entities: 1,
        skills: 1,
        commands: 0,
        agents: 0,
        plugins: 0,
        mcpServers: 0,
        instructionFiles: 0,
        byStatus: {
          ok: 1,
          'needs-review': 0,
          'at-risk': 0,
          broken: 0,
        },
      },
      issues: [],
    })

    expect(report.entities).toHaveLength(1)
  })
})
