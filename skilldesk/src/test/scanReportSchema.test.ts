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

  it('accepts local scanner shaped command and agent entities', () => {
    const report = scanReportSchema.parse({
      schemaVersion: '0.1',
      generatedAt: '2026-06-07T10:10:00Z',
      machine: {
        platform: 'win32',
        homeDir: 'C:\\Users\\example',
      },
      roots: [
        {
          path: 'C:\\Users\\example\\.claude\\commands',
          kind: 'directory',
          status: 'scanned',
        },
        {
          path: 'C:\\Users\\example\\.claude\\agents',
          kind: 'directory',
          status: 'scanned',
        },
      ],
      entities: [
        {
          id: 'command:claude-code:c--users-example--claude-commands-review-md',
          kind: 'command',
          platform: 'claude-code',
          name: 'review',
          title: 'Review Command',
          description: 'Runs a structured local review workflow description.',
          path: 'C:\\Users\\example\\.claude\\commands\\review.md',
          source: 'local',
          tags: [],
          discoveredAt: '2026-06-07T10:10:00Z',
          health: {
            status: 'ok',
            issues: [],
          },
          commandType: 'slash-command',
          file: 'C:\\Users\\example\\.claude\\commands\\review.md',
        },
        {
          id: 'agent:claude-code:c--users-example--claude-agents-researcher-md',
          kind: 'agent',
          platform: 'claude-code',
          name: 'researcher',
          description: 'Researches local project context without executing external tools.',
          path: 'C:\\Users\\example\\.claude\\agents\\researcher.md',
          source: 'local',
          tags: [],
          discoveredAt: '2026-06-07T10:10:00Z',
          health: {
            status: 'ok',
            issues: [],
          },
          file: 'C:\\Users\\example\\.claude\\agents\\researcher.md',
          declaredTools: ['Read', 'Grep'],
          declaredModel: 'sonnet',
        },
      ],
      totals: {
        entities: 2,
        skills: 0,
        commands: 1,
        agents: 1,
        plugins: 0,
        mcpServers: 0,
        instructionFiles: 0,
        byStatus: {
          ok: 2,
          'needs-review': 0,
          'at-risk': 0,
          broken: 0,
        },
      },
      issues: [],
    })

    expect(report.totals.commands).toBe(1)
    expect(report.totals.agents).toBe(1)
  })
})
