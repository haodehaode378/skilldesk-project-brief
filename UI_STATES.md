# SkillDesk UI States

This document defines reusable UI states for the SkillDesk desktop app. It should be used with `UI_SPEC.md` and `UX_FLOWS.md` before implementing or changing views.

SkillDesk is Chinese-first, read-only, and Windows-first. UI states must preserve that product contract.

## State Principles

- Always explain what data is being shown.
- Preserve the last usable report on transient failures.
- Keep error messages safe: paths and metadata are allowed; secrets and raw logs are not.
- Prefer local panel states over full-window blocking states.
- Avoid modal dialogs for routine scan states.

## Global Report Source States

| State | Chinese label | English label | Meaning | Primary UI treatment |
| --- | --- | --- | --- | --- |
| `fixture` | 示例数据 | Fixture data | Built-in demo report is shown. | Amber dot, non-blocking note. |
| `cached` | 缓存报告 | Cached report | Last local report from app cache is shown. | Green dot, timestamp. |
| `scanning` | 正在扫描 | Scanning | A local scan is running. | Spinner or pulsing dot, disable conflicting actions. |
| `local` | 本机只读扫描 | Local read-only scan | Fresh local scan report is shown. | Green dot, generated time. |
| `error` | 扫描失败 | Scan failed | Scan failed and previous report remains visible. | Red dot, inline error message. |

Rules:

- The state must be visible in the top command bar.
- Export actions are disabled while `scanning`.
- Navigation remains available while `scanning` unless data consistency requires otherwise.
- `error` does not clear the current report.

## Global Loading States

### App Boot

Use when initial settings/cache are being loaded.

Treatment:

- Show the shell quickly.
- Use compact skeletons in the main area.
- Avoid a full-screen splash unless app startup exceeds a noticeable delay.

Copy:

- zh-CN: `正在加载本地报告`
- en-US: `Loading local report`

### Scan In Progress

Use after the user starts local scan.

Treatment:

- Top state changes to `正在扫描 / Scanning`.
- Scan button text changes to `正在扫描 / Scanning`.
- Scan and export buttons are disabled.
- Existing report remains visible with subtle disabled/revalidating treatment if needed.

Future progress details:

- current root index
- roots completed
- entities discovered
- issues generated

Do not show:

- terminal logs
- raw file contents
- raw session or auth paths beyond safe path metadata

## Empty States

Empty states should be concise and local to the affected panel.

| Context | zh-CN | en-US | Next action |
| --- | --- | --- | --- |
| No extensions | 没有发现扩展。 | No extensions found. | Check scan roots. |
| Filtered extensions empty | 当前筛选条件没有匹配的扩展。 | No extensions match the current filters. | Clear filters. |
| No issues | 当前扫描没有发现问题。 | No issues were found in the current scan. | Review inventory. |
| Filtered issues empty | 当前筛选条件没有匹配的问题。 | No issues match the current filters. | Clear filters. |
| No MCP servers | 没有发现 MCP 服务配置。 | No MCP server configurations were found. | Check config roots. |
| No plugins | 没有发现插件 manifest。 | No plugin manifests were found. | Check plugin roots. |
| No sources | 没有扫描来源记录。 | No scan roots were recorded. | Reset default roots. |
| No instruction files | 没有发现项目指令文件。 | No project instruction files were found. | Continue audit. |

Rules:

- Do not use empty states to explain the whole product.
- Do not imply that absence of issues proves content is safe.
- Empty state copy should fit in one or two short lines.

## Error States

### Scan Root Missing

Use when a configured root path does not exist.

Display:

- root path
- status: missing
- safe reason

Copy:

- zh-CN: `这个扫描来源不存在。`
- en-US: `This scan root is missing.`

### Permission Denied

Use when the scanner cannot read a configured root.

Display:

- root path
- status: error
- safe permission summary

Copy:

- zh-CN: `没有权限读取这个扫描来源。`
- en-US: `SkillDesk does not have permission to read this scan root.`

Rules:

- Do not ask the user to weaken system security.
- Do not show raw OS logs.

### Invalid Config Or Manifest

Use when JSON/TOML/frontmatter/manifest parsing fails.

Display:

- file path
- parser category
- line/column if safe and available
- concise recommendation

Copy:

- zh-CN: `配置格式无法解析。`
- en-US: `The configuration format could not be parsed.`

### MCP Probe Failed

Use when lightweight MCP probe fails under allowed policy.

Display:

- server name
- transport
- safe command or host summary
- safe error class

Copy:

- zh-CN: `MCP 轻量探测失败。`
- en-US: `The lightweight MCP probe failed.`

Rules:

- Do not display auth headers.
- Do not display full URLs that may include credentials.
- Do not call arbitrary MCP tools as a retry.

### Export Failed

Use when report export cannot be written.

Display:

- target directory or safe path
- safe error message

Copy:

- zh-CN: `导出失败。`
- en-US: `Export failed.`

## Warning States

### Plugin Cache Summary Only

Meaning: plugin cache content was summarized but not deeply scanned.

Copy:

- zh-CN: `插件缓存当前仅汇总 manifest 和内置资源数量。`
- en-US: `Plugin cache scanning is summary-only.`

Treatment:

- Use info or low severity.
- Do not make cache summary-only look broken.

### MCP Probes Disabled

Meaning: configured MCP servers were discovered, but active probing is disabled.

Copy:

- zh-CN: `MCP 探测已禁用。`
- en-US: `MCP probes are disabled.`

Treatment:

- Show in Sources and MCP views.
- Do not show as a hard error.

### Possible Mojibake

Meaning: text appears garbled and needs review.

Copy:

- zh-CN: `检测到疑似乱码。`
- en-US: `Possible mojibake detected.`

Treatment:

- Needs review or medium severity depending on context.
- Link to file path when safe.

## Selection States

### Table Row Selection

Required behavior:

- Selected row is visually persistent.
- Detail panel updates immediately.
- Keyboard selection should be added before large-inventory workflows are considered complete.

Visual treatment:

- subtle background
- left accent or border when density allows
- no layout shift

### Filter Selection

Required behavior:

- Active filter is obvious.
- Result count updates.
- Clearing filters is easy when no results remain.

Visual treatment:

- active filter uses accent border or background
- inactive filter remains low contrast but readable

## Data Sensitivity States

### Redacted Value

Use when a value exists but should not be shown.

Display:

- `已隐藏 / Hidden`
- `已脱敏 / Redacted`
- count or host-only summary when useful

Examples:

- command argument count instead of secret-bearing args
- URL host instead of full URL
- environment variable name and presence, never value

### Omitted Value

Use when data was intentionally not collected.

Display:

- zh-CN: `未收集`
- en-US: `Not collected`

### Unknown Value

Use when data cannot be determined.

Display:

- `-`

Rules:

- Do not use `unknown` when data was intentionally redacted.
- Do not use `-` for known sensitive values that were hidden.

## Narrow Window States

Below the full desktop width:

- Sidebar can become top-stacked.
- Split list/detail becomes single column.
- Details should move below the list.
- Tables may preserve columns but must not create horizontal page overflow.

Required checks:

- `documentElement.scrollWidth <= documentElement.clientWidth`
- top actions remain reachable
- long paths wrap or truncate safely

## State Acceptance Checklist

- Report source is always visible.
- Scan failure preserves previous report.
- Empty states are local and concise.
- Permission errors are readable and non-destructive.
- MCP disabled state is not shown as broken.
- Redacted values are distinguishable from missing values.
- Narrow windows do not overflow horizontally.
- Chinese and English states have matching meaning.
