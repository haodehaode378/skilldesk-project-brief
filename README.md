# SkillDesk

SkillDesk 是一个 Windows-first、本地只读的 agent 扩展健康仪表盘。

它用于盘点和审计本机 agent 生态中的：

- Codex / `.agents` / Claude Code skills
- Claude Code slash commands
- Claude Code agents
- Codex / Claude Code plugin manifests
- MCP server 配置
- 项目指令文件，例如 `AGENTS.md`、`CLAUDE.md`、`.mcp.json`

SkillDesk 不运行 agent，不执行 skill 脚本、hooks、插件命令或任意 MCP tools。

## 下载

Windows 安装包会发布在 GitHub Releases：

- `SkillDesk_0.1.0_x64-setup.exe`：NSIS 安装器，推荐普通用户使用。
- `SkillDesk_0.1.0_x64_en-US.msi`：MSI 安装包，适合企业或批量安装场景。

## 当前能力

- 中文默认 UI，并支持在设置里切换中英文。
- 扫描 Codex / Claude Code / `.agents` 的 skills、commands、agents、plugins 和 MCP 配置。
- 展示扩展清单、详情、MCP 服务、插件、来源、问题和设置视图。
- 支持可配置扫描根、插件缓存扫描策略和 MCP 轻量探测策略。
- 只读扫描：不会执行脚本、hooks、插件命令、agent 任务或任意 MCP tools。
- 导出 JSON 扫描报告。

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

## 技术栈

- Tauri
- React
- Vite
- TypeScript
- Rust scanner core
- pnpm

## 安全边界

SkillDesk 把本地扩展视为不可信输入。它只读取元数据、路径、文件状态、Git 状态、manifest 摘要和 MCP 配置摘要。

它不会显示 API key、auth token、session、log、credential 文件内容或带凭据的完整 URL。
