# SkillDesk

[![Release](https://img.shields.io/github/v/release/haodehaode378/skilldesk-project-brief?label=release)](https://github.com/haodehaode378/skilldesk-project-brief/releases/latest)
[![Windows](https://img.shields.io/badge/platform-Windows-2563eb)](#下载)
[![Tauri](https://img.shields.io/badge/Tauri-2.x-24c8db)](https://tauri.app/)
[![React](https://img.shields.io/badge/React-19-61dafb)](https://react.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-6-3178c6)](https://www.typescriptlang.org/)

SkillDesk 是一个 Windows-first、本地只读的 agent 扩展健康审计桌面软件。

它不是 agent runner。它负责审计 agent 依赖的本地生态：skills、slash commands、agents、plugins、MCP servers，以及项目指令文件，例如 `AGENTS.md`、`CLAUDE.md` 和 `.mcp.json`。

```text
Elves runs agents.
SkillDesk audits the local ecosystem agents depend on.
```

## 当前状态

当前发布版本：[`v0.1.0`](https://github.com/haodehaode378/skilldesk-project-brief/releases/tag/v0.1.0)

SkillDesk 已经从项目 brief 进入 Tauri 桌面 MVP 实现阶段。当前版本支持：

- 本地只读扫描。
- 中文优先 UI，设置中可切换 English。
- 扩展清单、详情、MCP、插件、来源、问题队列、设置等工作区。
- JSON 和 Markdown 报告导出。
- 可配置扫描根、插件缓存扫描策略、MCP 探测策略。
- 软件级 UX 规格、任务流、状态模型和表格规格文档。

## 为什么做

现代 agent 工具越来越依赖本地扩展：

- 一个 skill 可能引用脚本、资源和参考文件。
- 一个 plugin 可能携带内置 skill、命令、agent 或 MCP 配置。
- 一个项目目录可能包含自己的 `AGENTS.md`、`CLAUDE.md` 或 `.mcp.json`。
- Codex、Claude Code、`.agents` 和项目级配置分散在不同目录。

这些内容很难一眼看清来源、状态、风险和 Git 变更。

SkillDesk 的目标是把它们统一成一个可审计的本地清单，帮助你回答：

- 本机有哪些 agent 扩展？
- 它们属于 Codex、Claude Code、`.agents`、plugin cache，还是项目本身？
- 哪些文件缺少描述、格式异常、引用断链或疑似乱码？
- 哪些 MCP 配置不可达、未探测或需要复查？
- 哪些来源有 Git remote、branch、commit 或 dirty state？
- 哪些内容包含 shell、network、eval、credential 等需要人工复查的模式？

## 核心边界

SkillDesk 的 MVP 是只读软件。

它不会：

- 运行 agents。
- 执行 skill 脚本。
- 执行 hooks。
- 执行 plugin commands。
- 调用 MCP 工具动作。
- 安装、更新、删除或覆盖用户扩展。
- 创建 git worktrees。
- 打开嵌入式终端。
- 管理 memory 或 session replay。
- 显示 API key、auth token、session、log、credential 文件内容。

## 功能矩阵

| 类型 | 当前支持 |
| --- | --- |
| Codex skills | 扫描 `SKILL.md`、描述、文件结构、Git 状态和健康信号 |
| `.agents` skills | 扫描本地 `.agents\skills` |
| Claude Code skills | 扫描 `.claude\skills` |
| Claude slash commands | 扫描 `.claude\commands` Markdown |
| Claude agents | 扫描 `.claude\agents` Markdown、声明模型和工具 |
| Plugins | 扫描 Codex / Claude Code plugin manifests 和内置资源数量 |
| MCP servers | 解析 config，识别 transport、command、args count、URL host 和轻量可达性 |
| Project instructions | 扫描 `AGENTS.md`、`CLAUDE.md`、`.mcp.json` |
| Git metadata | 记录 remote、branch、commit、dirty state |
| Issues | 生成 metadata、format、encoding、security、mcp、path 等健康问题 |
| Export | 导出 JSON 和 Markdown 扫描报告 |
| UI language | 默认中文，支持 English |

## 界面

当前桌面软件包含这些工作区：

- **概览**：扩展总数、健康状态、MCP、插件、疑似乱码、Git dirty 来源和报告摘要。
- **扩展清单**：搜索、状态筛选、类型筛选、列表选择和右侧详情。
- **详情**：按 Skill / Command / Agent / Plugin / MCP / 指令文件显示专属元数据。
- **MCP 服务**：transport、command/host 安全摘要、probe 状态、工具/资源/Prompt 数量。
- **插件**：manifest、publisher、version、bundle summary、cache / backup 状态。
- **来源**：扫描根、自检状态、能力策略、项目指令文件。
- **问题队列**：严重级别筛选、问题说明、证据和建议。
- **设置**：语言、扫描根、插件缓存、MCP probe policy、缓存清理。

## 下载

从 GitHub Releases 下载：

- [`SkillDesk_0.1.0_x64-setup.exe`](https://github.com/haodehaode378/skilldesk-project-brief/releases/download/v0.1.0/SkillDesk_0.1.0_x64-setup.exe)：推荐普通 Windows 用户使用的安装器。
- [`SkillDesk_0.1.0_x64_en-US.msi`](https://github.com/haodehaode378/skilldesk-project-brief/releases/download/v0.1.0/SkillDesk_0.1.0_x64_en-US.msi)：适合企业或批量安装场景。
- [`SkillDesk_0.1.0_SHA256SUMS.txt`](https://github.com/haodehaode378/skilldesk-project-brief/releases/download/v0.1.0/SkillDesk_0.1.0_SHA256SUMS.txt)：安装包 SHA-256 校验。

说明：当前构建未做代码签名，Windows 可能显示未知发布者提示。

## 默认扫描范围

Windows 默认扫描根：

```text
%USERPROFILE%\.codex\skills
%USERPROFILE%\.agents\skills
%USERPROFILE%\.claude\skills
%USERPROFILE%\.claude\commands
%USERPROFILE%\.claude\agents
%USERPROFILE%\.codex\config.toml
%USERPROFILE%\.claude\mcp-configs
```

可选高级来源：

```text
%USERPROFILE%\.codex\plugins
%USERPROFILE%\.claude\plugins
```

默认排除：

- `node_modules`
- sessions
- history files
- logs
- sqlite databases
- credentials
- auth files
- sandbox secrets
- temporary directories
- backups
- marketplace cache expansion noise

## 技术栈

- Desktop shell: Tauri 2
- Frontend: React 19, Vite, TypeScript
- Scanner core: Rust
- Validation: Vitest, Cargo tests
- Package manager: pnpm
- Cache: JSON first, SQLite later if needed

## 开发

```powershell
cd skilldesk
pnpm install
pnpm dev
```

启动 Tauri 桌面开发环境：

```powershell
cd skilldesk
pnpm tauri dev
```

## 验证

```powershell
cd skilldesk
pnpm run typecheck
pnpm run lint
pnpm test
pnpm run build
cargo test --manifest-path src-tauri/Cargo.toml
cargo check --manifest-path src-tauri/Cargo.toml
pnpm tauri build
```

文档或中文文案变更后，还应运行 mojibake 检查：

```powershell
check-mojibake --root .
```

## 文档索引

| 文件 | 用途 |
| --- | --- |
| [`MVP.md`](MVP.md) | MVP 范围和验收标准 |
| [`DATA_MODEL.md`](DATA_MODEL.md) | Scanner、cache、UI 的数据契约 |
| [`SECURITY.md`](SECURITY.md) | 只读扫描和敏感信息安全规则 |
| [`UI_SPEC.md`](UI_SPEC.md) | 桌面软件 UX 总规格 |
| [`UX_FLOWS.md`](UX_FLOWS.md) | 首次启动、扫描、triage、设置、导出等任务流 |
| [`UI_STATES.md`](UI_STATES.md) | 加载、空态、错误、警告、选择、脱敏等 UI 状态 |
| [`TABLE_SPEC.md`](TABLE_SPEC.md) | 扩展清单、问题队列、MCP、插件等表格行为 |
| [`ROADMAP.md`](ROADMAP.md) | 阶段规划 |
| [`DECISIONS.md`](DECISIONS.md) | 架构和产品决策 |
| [`SCAFFOLD_PLAN.md`](SCAFFOLD_PLAN.md) | Tauri 项目脚手架计划 |
| [`AGENTS.md`](AGENTS.md) | 未来 agent 执行基线 |

## 仓库结构

```text
.
├── skilldesk/              # Tauri + React implementation
├── MVP.md                  # MVP scope and acceptance criteria
├── DATA_MODEL.md           # Scanner/cache/UI contracts
├── SECURITY.md             # Read-only scanner safety model
├── UI_SPEC.md              # Software UX specification
├── UX_FLOWS.md             # Desktop task flows
├── UI_STATES.md            # UI state model
├── TABLE_SPEC.md           # Inventory table behavior
├── ROADMAP.md              # Product roadmap
└── AGENTS.md               # Agent execution baseline
```

## Roadmap

近期优先级：

1. 把扩展清单升级为真正的桌面工作表：排序、键盘选择、路径复制、筛选记忆。
2. 把问题队列升级为 triage 工作区：按实体/严重级别/类别分组，点击问题跳到详情。
3. 完善扫描状态：扫描中、失败、缓存报告、本机报告的差异表达。
4. 改善大数据量体验：分页或虚拟列表。
5. 增加安全导出模式：脱敏路径、脱敏配置摘要。

长期方向：

- SQLite-backed history when repeated scan history requires it.
- Diff from previous scan.
- CI mode for project-level audits.
- More detailed plugin manifest support.
- More complete MCP protocol-level fixture coverage.
- Windows installer signing.

这些方向不能改变 MVP 的核心边界：SkillDesk audits; it does not run agents.

## License

MIT
