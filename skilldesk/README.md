# SkillDesk

SkillDesk 是一个 Windows-first、本地只读的 agent 扩展健康仪表盘。

它用于盘点和审计本机 agent 生态中的：

- Codex / `.agents` / Claude Code skills
- Claude Code slash commands
- Claude Code agents
- Codex plugin manifests
- MCP server 配置
- 项目指令文件，例如 `AGENTS.md`、`CLAUDE.md`、`.mcp.json`

SkillDesk 不运行 agent，不执行 skill 脚本、hooks、插件命令或 MCP tools。

## 技术栈

- Tauri
- React
- Vite
- TypeScript
- Rust scanner core
- pnpm

## 开发

```powershell
pnpm install
pnpm dev
```

## 验证

```powershell
pnpm typecheck
pnpm lint
pnpm test
pnpm build
cargo test --manifest-path src-tauri/Cargo.toml
cargo check --manifest-path src-tauri/Cargo.toml
pnpm tauri build --debug
```

## 只读扫描边界

当前 scanner 只读取文本元数据和文件状态：

- 路径、mtime、文件大小指纹
- Markdown 标题和描述
- plugin manifest 摘要
- MCP 配置名称、传输类型、命令名、参数数量、URL host
- Git remote、branch、commit、dirty 状态

安全约束：

- 不执行第三方脚本或二进制文件
- 不运行 hooks
- 不运行插件命令
- 不调用任意 MCP tools
- 不显示 API key、auth token、session、log、credential 文件内容
- MCP 主动探测默认禁用

## UI

默认语言是中文。设置页和顶部操作区都可以在中文和英文之间切换。

主要视图：

- 概览
- 扩展清单
- MCP 服务
- 插件
- 来源
- 问题
- 设置

## 构建产物

Debug 打包后会生成：

```text
src-tauri\target\debug\skilldesk.exe
src-tauri\target\debug\bundle\msi\SkillDesk_0.1.0_x64_en-US.msi
src-tauri\target\debug\bundle\nsis\SkillDesk_0.1.0_x64-setup.exe
```
