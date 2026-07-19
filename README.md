# Astray Verify

> Record an MCP server's contract once. Catch breaking changes before your AI clients do.

[![CI](https://github.com/TheAstrayDev/astray-verify/actions/workflows/ci.yml/badge.svg)](https://github.com/TheAstrayDev/astray-verify/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-2b8a3e.svg)](LICENSE)
[![MCP](https://img.shields.io/badge/MCP-contract%20tests-161616.svg)](https://modelcontextprotocol.io/)

An MCP server can still start successfully after a release while an AI client has
quietly lost a tool, received a changed JSON schema, or begun seeing invalid data
on `stdout`. Astray Verify turns the server interface that already works today
into a committed regression test.

It is a tiny, local-first test runner for authors of MCP servers and teams that
maintain them.

## What it does

```text
working MCP server
       │
       ├── astray-verify record
       │       saves the expected tools and schemas
       │
       └── astray-verify test
               fails when a later change breaks that contract
```

In the first release, Astray Verify launches a stdio MCP server, performs the
standard handshake, snapshots `tools/list`, and replays it later. A fixture is
plain JSON, designed to be reviewed and committed with the server source.

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
PASS  echo
```

## Why not only use the MCP Inspector?

The official MCP Inspector is excellent for exploring and debugging a server
interactively. Astray Verify is for the next step: a small, repeatable check that
runs after every change in local development or CI.

| Inspector | Astray Verify |
| --- | --- |
| Explore and debug now | Preserve what worked for later |
| Interactive | Commit-friendly fixture |
| Individual calls | Regression check in CI |

## Current scope

- Stdio transport
- MCP `initialize` handshake
- `tools/list` contract snapshots
- Strict JSON-RPC stdout validation
- Human-readable failure output

## Roadmap

- [ ] Record and replay `tools/call` fixtures
- [ ] Stable, reviewable JSON diff output
- [ ] GitHub Action
- [ ] Streamable HTTP support
- [ ] Compatibility profiles for major MCP clients

## Contributing

Issues, fixtures from real servers, and focused pull requests are welcome.
Please do not add new transport or client support without a reproducible test
case. The project should stay small, predictable, and useful in CI.

## License

MIT. See [LICENSE](LICENSE).
