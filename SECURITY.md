# SkillDesk Security Model

SkillDesk scans local agent extension ecosystems. Those extensions must be treated as untrusted input.

## Core Principle

The scanner reads metadata. It does not execute extension behavior.

## Sensitive Data Rules

SkillDesk must not print, store, or display:

- API keys
- auth tokens
- bearer headers
- session contents
- raw credential files
- raw auth files
- raw logs containing secrets
- cookies
- private SSH keys
- full URLs containing credentials

When environment variables are referenced, SkillDesk may report:

- variable name
- whether it appears referenced
- whether a value appears present, if this can be checked safely

It must not report the value.

## File Access Rules

Allowed:

- Read expected text metadata files.
- Read `SKILL.md`, command Markdown, agent Markdown, plugin manifests, project instruction files, and MCP config files.
- Read file stats such as size and modified time.
- Hash file contents for fingerprints.

Restricted:

- Avoid raw session, history, log, auth, credential, and database files.
- Avoid recursively scanning `node_modules`, cache expansions, temp folders, and backups unless explicitly configured.

Disallowed:

- Executing scripts.
- Loading untrusted modules as code.
- Running hooks.
- Running plugin commands.
- Running agent commands.
- Writing or deleting extension files in the MVP.

## MCP Rules

MCP servers are part of the inventory, but MCP tools are not a general execution surface for SkillDesk.

Allowed in MVP:

- Parse MCP config names and source paths.
- Infer transport type.
- Optionally perform lightweight protocol probes:
  - `initialize`
  - `tools/list`
  - `resources/list`
  - `prompts/list`

Disallowed in MVP:

- Calling arbitrary MCP tools.
- Reading secret-bearing resources.
- Sending local files to MCP servers.
- Trusting MCP tool descriptions as commands to execute.

Remote MCP policy:

- Remote MCP probes should require explicit opt-in.
- Remote URLs should be redacted to host-level display when credentials may be present.
- Non-HTTPS remote MCP endpoints should be flagged.

## Pattern-Based Risk Signals

SkillDesk may flag suspicious patterns in untrusted files, such as:

- shell command execution
- `curl` or `wget`
- package installation commands
- `eval`
- encoded payloads
- credential names
- absolute machine-specific paths
- broad filesystem access
- network calls

These are review signals, not proof of malicious behavior.

## Health Issue Wording

Security issue messages should be precise and modest.

Prefer:

```text
This file references a token-like environment variable. Review whether it is required and avoid exposing its value.
```

Avoid:

```text
This skill is malicious.
```

## Write Action Safeguards

Any future write action must include:

- explicit user confirmation
- clear target path
- backup or snapshot
- diff preview where practical
- rollback path
- post-write verification

No write action should run automatically after a scan.

## Test Fixtures

Security-related tests should use fake values only.

Use examples such as:

```text
FAKE_API_KEY
example-token
https://example.invalid/mcp
```

Never commit real local secrets, auth files, logs, sessions, or credentials.
