<div align="center">

# Astray Verify

### Keep MCP changes intentional.

Record the public contract of an MCP server, commit it with the server, and detect accidental breaking changes in local development or CI.

[![Release](https://img.shields.io/github/v/release/TheAstrayDev/astray-verify?style=flat-square)](https://github.com/TheAstrayDev/astray-verify/releases/latest)
[![CI](https://img.shields.io/github/actions/workflow/status/TheAstrayDev/astray-verify/ci.yml?branch=main&style=flat-square&label=checks)](https://github.com/TheAstrayDev/astray-verify/actions/workflows/ci.yml)
[![MIT license](https://img.shields.io/badge/license-MIT-176a4b?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.74%2B-orange?style=flat-square)](https://www.rust-lang.org/)

[Get started](#get-started) · [GitHub Action](#github-action) · [CLI reference](#cli-reference) · [Contributing](CONTRIBUTING.md)

</div>

## What it protects

An MCP server can start normally while clients fail because a tool was renamed, its JSON schema changed, or a discovery surface drifted. Astray Verify makes the interface explicit:

```text
record a known-good server  →  commit its fixture  →  replay it on every change
```

It is local-first: no model calls, accounts, tokens, dashboards, or hosted state. Fixtures are readable JSON files that belong beside your server code.

## Get started

### Install a release binary

Linux and macOS:

```bash
curl -fsSL https://raw.githubusercontent.com/TheAstrayDev/astray-verify/main/install.sh | sh
```

Windows PowerShell:

```powershell
curl.exe -fsSL https://raw.githubusercontent.com/TheAstrayDev/astray-verify/main/install.ps1 | powershell -NoProfile -ExecutionPolicy Bypass -
```

Or install from source:

```bash
cargo install astray-verify
```

### Create and verify a contract

Run this in the repository that contains your MCP server. Everything after `--` starts the server you are testing.

```bash
astray-verify init
astray-verify record --name filesystem -- \
  npx -y @modelcontextprotocol/server-filesystem ./demo
astray-verify test
```

Commit the files Astray Verify creates:

```text
astray.verify.json
fixtures/
  filesystem.mcp.json
```

When an API change is deliberate, record the fixture again and review its diff with the code change.

## What is checked

| Surface | Default | Detects |
| --- | --- | --- |
| `tools/list` | Yes | Added, removed, or changed tool definitions and schemas |
| `resources/list` | Optional | Resource discovery contract changes |
| `prompts/list` | Optional | Prompt discovery contract changes |

Record a complete discovery contract with a suitable timeout:

```bash
astray-verify record --name complete \
  --checks tools,resources,prompts --timeout-ms 45000 -- npx your-mcp-server
```

For a temporary CI policy, override the fixture's stored surfaces or timeout:

```bash
astray-verify test --checks tools,resources,prompts --timeout-ms 60000
```

## GitHub Action

Use the composite Action to install the latest release and test committed fixtures:

```yaml
- name: Verify MCP contracts
  uses: TheAstrayDev/astray-verify@v0.2.2
  with:
    command: test
    checks: tools,resources,prompts
    timeout-ms: 45000
    log-file: logs/astray-verify.jsonl
    audit-on-failure: "true"
```

The Action installs the Linux release binary, writes an optional JSON Lines log, and audits each fixture after a failed run to help locate the weak link.

## CLI reference

| Command | Purpose |
| --- | --- |
| `init` | Create the project configuration and fixture directory. |
| `record` | Start a server and save its selected discovery surfaces. |
| `test` | Replay fixtures and report contract drift. |
| `audit` | Inspect a server or fixture and rank protocol risks. |
| `config` | Print the resolved project configuration. |
| `doctor` | Check configuration, fixtures, logs, and optionally run tests. |
| `watch` | Re-run tests whenever configuration or fixtures change. |

Global options are available on every command:

```bash
astray-verify --json test              # one machine-readable report
astray-verify --log logs/verify.jsonl test
astray-verify --fixtures-dir contracts test
astray-verify --color never test
```

## Configuration

`astray.verify.json` holds portable defaults for the repository:

```json
{
  "version": 2,
  "fixtures_dir": "fixtures",
  "defaults": {
    "checks": ["tools", "resources", "prompts"],
    "timeout_ms": 30000
  }
}
```

Version 1 fixtures and configuration remain supported.

## Scope and direction

Current support is stdio MCP transport, the `initialize` handshake, discovery snapshots, structured output, execution logs, audit, and watch mode. Planned work includes `tools/call` fixtures, reviewable JSON diffs, Streamable HTTP, and client compatibility profiles.

## Contributor

Astray Verify is created and maintained by [TheAstrayDev](https://github.com/TheAstrayDev). Contributor information lives in [CONTRIBUTORS.md](CONTRIBUTORS.md).

## License

MIT — see [LICENSE](LICENSE).
