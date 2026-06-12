# SkillDesk

[![Release](https://img.shields.io/github/v/release/haodehaode378/skilldesk-project-brief?label=release)](https://github.com/haodehaode378/skilldesk-project-brief/releases/latest)
[![Windows](https://img.shields.io/badge/platform-Windows-2563eb)](#下载)
[![Tauri](https://img.shields.io/badge/Tauri-2.x-24c8db)](https://tauri.app/)
[![React](https://img.shields.io/badge/React-19-61dafb)](https://react.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-6-3178c6)](https://www.typescriptlang.org/)

SkillDesk 是一个 Windows-first、本地只读的 agent 扩展健康仪表盘。
它不是 agent runner。它负责审计 agent 依赖的本地生态：skills、slash commands、agents、plugins、MCP servers，以及项目指令文件。

```text
Elves runs agents.
SkillDesk audits the local ecosystem agents depend on.
```

## 为什么做

现代 agent 工具越来越依赖本地扩展：一个 skill 可能引用脚本，一个 plugin 可能携带 MCP 配置，一个项目目录可能有自己的 `AGENTS.md`。这些文件分散在不同目录，状态、来源、风险和 Git 变更很难一眼看清。

SkillDesk 的目标是把这些本地扩展统一成一个可审计的清单，让你知道：

- 本机有哪些 agent 扩展。
- 它们属于 Codex、Claude Code、`.agents`，还是项目本身。
- 哪些文件缺少描述、格式异常、引用断链或疑似乱码。
- 哪些 MCP 配置不可达、非 HTTPS 或需要复查。
- 哪些来源有 Git remote、branch、commit 或 dirty state。
- 哪些内容包含 shell、network、eval、credential 等高风险模式。

## 当前版本

当前发布版本：[`v0.1.0`](https://github.com/haodehaode378/skilldesk-project-brief/releases/tag/v0.1.0)

这是第一个 Windows 桌面 MVP。它已经可以本地只读扫描、展示、过滤、导出报告，并提供可配置扫描根和 MCP 探测策略。

变更记录见 [`CHANGELOG.md`](CHANGELOG.md)。

## 下载

从 GitHub Releases 下载：

- [`SkillDesk_0.1.0_x64-setup.exe`](https://github.com/haodehaode378/skilldesk-project-brief/releases/download/v0.1.0/SkillDesk_0.1.0_x64-setup.exe)：推荐普通用户使用的 Windows 安装器。
- [`SkillDesk_0.1.0_x64_en-US.msi`](https://github.com/haodehaode378/skilldesk-project-brief/releases/download/v0.1.0/SkillDesk_0.1.0_x64_en-US.msi)：适合企业或批量安装场景。
- [`SkillDesk_0.1.0_SHA256SUMS.txt`](https://github.com/haodehaode378/skilldesk-project-brief/releases/download/v0.1.0/SkillDesk_0.1.0_SHA256SUMS.txt)：安装包 SHA-256 校验。

说明：当前构建未做代码签名，Windows 可能显示未知发布者提示。

## 能力矩阵

| 类型 | 当前支持 |
| --- | --- |
| Codex skills | 扫描 `SKILL.md`、描述、文件结构、Git 状态和风险信号 |
| `.agents` skills | 扫描本地 `.agents\skills` |
| Claude Code skills | 扫描 `.claude\skills` |
| Claude slash commands | 扫描 `.claude\commands` Markdown |
| Claude agents | 扫描 `.claude\agents` Markdown、声明模型和工具 |
| Plugins | 扫描 Codex / Claude Code plugin manifests 和内置资源数量 |
| MCP servers | 解析 config，识别 transport、command、args、URL host 和轻量可达性 |
| Project instructions | 扫描 `AGENTS.md`、`CLAUDE.md`、`.mcp.json` |
| Export | 导出 JSON 扫描报告 |
| UI language | 默认中文，设置中可切换 English |

## 只读安全模型

SkillDesk 把本地扩展视为不可信输入。扫描过程只读取元数据和安全摘要。

它不会：

- 运行 agents。
- 执行 skill 脚本。
- 执行 hooks。
- 执行 plugin commands。
- 调用任意 MCP tools。
- 展示 API key、auth token、session、log、credential 文件内容。
- 展示带凭据的完整 URL。

MCP 探测是策略控制的轻量 TCP reachability probe。默认关闭；即使开启，也不会调用 MCP tools、resources 或 prompts。

## 默认扫描范围

```text
%USERPROFILE%\.codex\skills
%USERPROFILE%\.agents\skills
%USERPROFILE%\.claude\skills
%USERPROFILE%\.claude\commands
%USERPROFILE%\.claude\agents
%USERPROFILE%\.codex\plugins
%USERPROFILE%\.claude\plugins
%USERPROFILE%\.codex\config.toml
%USERPROFILE%\.claude\mcp-configs
```

设置页支持编辑扫描根。未知或不属于 MVP 支持范围的路径会被标记为 skipped，不会被盲目递归扫描。

## 界面

SkillDesk 当前包含这些视图：

- 概览：总量、风险状态、MCP、插件、乱码和 Git dirty 来源。
- 扩展清单：搜索、状态过滤、类型过滤。
- 详情：按 Skill / Command / Agent / Plugin / MCP / 指令文件展示专属元数据。
- MCP 服务：transport、command、host、probe 状态。
- 插件：manifest、publisher、version、bundle summary、cache / backup 状态。
- 来源：扫描根、自检状态、能力策略。
- 问题：严重级别过滤、证据、建议。
- 设置：语言、扫描根、插件缓存、MCP probe policy、缓存清理。

## 技术栈

- Desktop shell: Tauri 2
- Frontend: React 19, Vite, TypeScript
- Scanner core: Rust
- Validation: Vitest, Cargo tests
- Package manager: pnpm

## 开发

```powershell
cd skilldesk
pnpm install
pnpm dev
```

## 验证

```powershell
cd skilldesk
pnpm typecheck
pnpm lint
pnpm test
pnpm build
cargo test --manifest-path src-tauri/Cargo.toml
cargo check --manifest-path src-tauri/Cargo.toml
pnpm tauri build
```

当前版本的验证覆盖：

- 前端模型、缓存、导出、双语文案 key、fixture 渲染。
- Rust scanner health checks、Git metadata、MCP URL 脱敏、MCP probe、configured local scan fixture。
- Windows release build 和安装包生成。

## 路线图

`v0.1.0` 重点是只读扫描、统一模型和桌面 MVP。

后续方向：

- 更完整的 MCP protocol-level list probe fixture。
- 更细的 plugin manifest schema 支持。
- SQLite cache。
- UI 截图/视觉回归测试。
- 安装包代码签名。

## 仓库结构

```text
.
├── skilldesk/              # Tauri + React implementation
├── MVP.md                  # MVP scope and acceptance criteria
├── DATA_MODEL.md           # Scanner/cache/UI contracts
├── SECURITY.md             # Read-only scanner safety model
├── UI_SPEC.md              # UI structure and interaction notes
└── AGENTS.md               # Agent execution baseline
```

## License

MIT
