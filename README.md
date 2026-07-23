<div align="center">

# Astray Verify

> Record an MCP server's contract once. Catch breaking changes before your AI clients do.

[![Latest Release](https://img.shields.io/github/v/release/TheAstrayDev/astray-verify?include_prereleases&sort=semver&style=for-the-badge)](https://github.com/TheAstrayDev/astray-verify/releases/latest)
[![CI](https://img.shields.io/github/actions/workflow/status/TheAstrayDev/astray-verify/ci.yml?branch=main&style=for-the-badge)](https://github.com/TheAstrayDev/astray-verify/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-2b8a3e.svg?style=for-the-badge)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.74%2B-orange.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org)
[![MCP](https://img.shields.io/badge/MCP-contract%20tests-161616.svg?style=for-the-badge)](https://modelcontextprotocol.io/)
[![Crates.io](https://img.shields.io/crates/v/astray-verify.svg?style=for-the-badge)](https://crates.io/crates/astray-verify)
[![Downloads](https://img.shields.io/crates/d/astray-verify.svg?style=for-the-badge)](https://crates.io/crates/astray-verify)
[![Stars](https://img.shields.io/github/stars/TheAstrayDev/astray-verify?style=for-the-badge)](https://github.com/TheAstrayDev/astray-verify/stargazers)
[![Issues](https://img.shields.io/github/issues/TheAstrayDev/astray-verify?style=for-the-badge)](https://github.com/TheAstrayDev/astray-verify/issues)

A tiny, local-first test runner for **Model Context Protocol** servers.
Snapshot your `tools/list`, `resources/list`, and `prompts/list` contract once,
then catch breaking changes locally and in CI.

[Install](#install) · [Quick start](#quick-start) · [Commands](#commands) · [Roadmap](#roadmap) · [Contributors](#contributors)

</div>

---

## Why Astray Verify?

An MCP server can still start successfully after a release while an AI client
has quietly lost a tool, received a changed JSON schema, or begun seeing
invalid data on `stdout`. Astray Verify turns the server interface that
already works today into a committed regression test.

```text
working MCP server
       │
       ├── astray-verify record
       │       saves the expected tools, resources, and prompts
       │
       └── astray-verify test
       │       fails when a later change breaks that contract
       │
       └── astray-verify audit
               ranks protocol and contract weaknesses, names the weakest link
```

Fixtures are plain JSON, designed to be reviewed and committed with the
server source.

## Install

### Linux and macOS

```bash
curl -fsSL https://raw.githubusercontent.com/TheAstrayDev/astray-verify/main/install.sh | sh
```

### Windows PowerShell

```powershell
curl.exe -fsSL https://raw.githubusercontent.com/TheAstrayDev/astray-verify/main/install.ps1 | powershell -NoProfile -ExecutionPolicy Bypass -
```

The installers download the matching binary from the latest GitHub Release.

### From source

```bash
git clone https://github.com/TheAstrayDev/astray-verify.git
cd astray-verify
cargo install --path .
```

### From crates.io

```bash
cargo install astray-verify
```

## Quick start

Run these commands inside the repository that contains your MCP server:

```bash
astray-verify init

# Everything after -- is the command that starts your MCP server.
astray-verify record --name filesystem -- \
  npx -y @modelcontextprotocol/server-filesystem ./demo

astray-verify test
```

You will get two files worth committing:

```text
astray.verify.json
fixtures/
  filesystem.mcp.json
```

When an intentional interface change is made, record the fixture again and
review the diff just like any other API change.

## Example

The repository includes a minimal MCP demo server:

```bash
mkdir /tmp/astray-verify-demo && cd /tmp/astray-verify-demo
astray-verify init
astray-verify record --name echo -- \
  python3 /path/to/astray-verify/examples/echo_server.py
astray-verify test
```

Expected result:

```text
ASTRAY VERIFY  MCP CONTRACTS
────────────────────────────────────────
Verify contracts
● Checking echo
✓ PASS  echo
✓ 1 passed · 0 failed
```

## Commands

| Command | What it does |
| --- | --- |
| `astray-verify init` | Create `astray.verify.json` and the fixtures directory. |
| `astray-verify record` | Launch the MCP server, snapshot its contract into a fixture. |
| `astray-verify test` | Replay saved fixtures and report interface regressions. |
| `astray-verify audit` | Inspect the MCP server and rank protocol/contract weaknesses, with a single named weakest link. |
| `astray-verify config` | Show the resolved project configuration and defaults. |
| `astray-verify doctor` | One-shot project health check: config, fixtures, log, optional test pass. |
| `astray-verify watch` | Re-run `test` whenever a fixture or configuration file changes. |

Global flags:

```bash
astray-verify --color never test    # disable ANSI colour explicitly
astray-verify --json test           # one structured JSON report on stdout
astray-verify --log test.jsonl test # append JSON Lines execution log
```

## CLI experience

The normal output is deliberately short enough for CI and expressive enough
for local work. In an interactive terminal it adds a compact `ASTRAY VERIFY`
header and coloured status markers; redirected output stays clean and
colour-free.

```text
ASTRAY VERIFY  MCP CONTRACTS
────────────────────────────────────────
Verify contracts
● Checking filesystem
✓ PASS  filesystem
✓ 1 passed · 0 failed
```

When a contract changes, the failure explains the shape of the change
instead of only reporting a generic mismatch:

```text
✗ FAIL  filesystem
   removed tool `read_file`
   changed contract for `write_file`
```

## Configurable verification

`record` can snapshot each MCP discovery surface independently. The fixture
stores these settings, so CI uses the same checks and timeout as the
baseline:

```bash
# Tools only (default)
astray-verify record --name filesystem --checks tools -- npx your-server

# Tools, resources, and prompts, with a custom timeout
astray-verify record --name complete --checks tools,resources,prompts \
  --timeout-ms 45000 -- npx your-server

# Override a fixture timeout for one CI run
astray-verify test --name complete --timeout-ms 60000
```

`astray.verify.json` also supports project defaults:

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

Use `astray-verify config` to inspect the resolved project defaults.
Existing version 1 config files and fixtures remain supported.

## MCP audit and logs

`audit` starts an MCP server, exercises the handshake plus all discovery
surfaces, and ranks protocol and contract weaknesses. It calls out the
single highest-risk area first with a concrete repair action.

```bash
astray-verify audit -- npx your-server
astray-verify audit --name filesystem
astray-verify --log audit.jsonl audit -- npx your-server
```

Logs are newline-delimited JSON with command, timestamp, status, and error
fields. `--json` produces one complete machine-readable report on stdout
for every command, including all audit findings and the selected weakest
link.

## Doctor and watch

`doctor` checks the project state in a single pass: configuration
presence, fixture count, log file, and (with `--with-test`) a fresh run
of the test suite. It also creates the project layout on demand so a
fresh clone becomes usable with one call.

```bash
astray-verify doctor
astray-verify doctor --with-test
```

`watch` keeps the contract honest during development. It runs the test
suite once, then re-runs it whenever `astray.verify.json` or any fixture
file changes.

```bash
astray-verify watch --initial
```

## Why not only use the MCP Inspector?

The official MCP Inspector is excellent for exploring and debugging a
server interactively. Astray Verify is for the next step: a small,
repeatable check that runs after every change in local development or CI.

| Inspector | Astray Verify |
| --- | --- |
| Explore and debug now | Preserve what worked for later |
| Interactive | Commit-friendly fixture |
| Individual calls | Regression check in CI |

## Current scope

- Stdio transport
- MCP `initialize` handshake
- Contract snapshots for `tools/list`, `resources/list`, and `prompts/list`
- Per-fixture check selection and timeouts
- MCP contract audit with actionable weakest-link diagnosis
- Optional JSON Lines execution logs
- Doctor and watch automation commands
- Human-readable or JSON output

## Roadmap

- [ ] Record and replay `tools/call` fixtures
- [ ] Stable, reviewable JSON diff output
- [ ] GitHub Action
- [ ] Streamable HTTP support
- [ ] Compatibility profiles for major MCP clients

## Contributors

This project is maintained by:

- **TheAstrayDev** — [@TheAstrayDev](https://github.com/TheAstrayDev)

Issues, fixtures from real servers, and focused pull requests are
welcome. Please do not add new transport or client support without a
reproducible test case. The project should stay small, predictable, and
useful in CI.

## License

MIT. See [LICENSE](LICENSE).
