# Changelog

All notable changes to **Astray Verify** are documented here. The format
follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the
project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2026-07-24

### Added

- **GitHub Action** `TheAstrayDev/astray-verify@v0.2.1`: composite
  Action that installs the CLI, runs any subcommand, supports per-run
  `--checks`, `--timeout-ms`, and `--log-file`, and optionally audits
  every fixture when the run fails.
- New `--fixtures-dir` global flag that overrides the configured
  fixtures directory for a single invocation. Useful for monorepos and
  the new Action.
- Expanded crate keywords: `ai-tools`, `mcp`, `model-context-protocol`,
  `testing`, `contract`, `cli`, `regression`, `snapshot`, `json-rpc`,
  `ci`.
- Expanded crate categories: `command-line-utilities`,
  `development-tools::testing`, `development-tools::debugging`.

### Changed

- README, About, and badge set refreshed for v0.2.1. Roadmap updated:
  per-fixture checks, audit, JSONL logs, doctor, watch, and the
  GitHub Action are now marked complete.
- `load_config` and `ensure_project` now accept an override directory,
  removing duplicated fixture lookups in `audit`, `config`, and
  `doctor`.
- `init` accepts the override directory at command time, so CI can
  bootstrap a custom layout without rewriting the config.

### Maintainer

- TheAstrayDev — https://github.com/TheAstrayDev

## [0.2.0] - 2026-07-24

### Added

- Per-fixture MCP check selection: `tools`, `resources`, `prompts`.
- Per-fixture request timeout (`--timeout-ms`) with project-wide defaults
  in `astray.verify.json`.
- New `audit` command: ranks protocol and contract weaknesses, names the
  single highest-risk area, and suggests a concrete repair action.
- New `config` command to inspect the resolved project configuration.
- New `doctor` command for one-shot project health checks (config,
  fixtures, JSONL log, optional test pass).
- New `watch` command that re-runs the test suite when a fixture or
  `astray.verify.json` changes.
- Optional JSON Lines execution log via the global `--log` flag.
- Stable, machine-readable output via the global `--json` flag for every
  command.
- Explicit colour control via `--color auto|always|never`.
- Crate metadata: `authors`, `repository`, `keywords`, and `categories`
  for discoverability on crates.io.

### Changed

- Configuration format is now version 2 and supports per-project defaults
  (`defaults.checks`, `defaults.timeout_ms`). Version 1 files and fixtures
  are still loaded transparently.
- Failure summaries in `test` now distinguish removed, added, and
  changed tools per MCP surface.

### Maintainer

- TheAstrayDev — https://github.com/TheAstrayDev

## [0.1.0] - 2026-07-20

### Added

- Initial release.
- Stdio transport and MCP `initialize` handshake.
- `tools/list` contract snapshots saved as plain JSON fixtures.
- Human-readable and JSON output for `init`, `record`, and `test`.
- Linux/macOS and Windows PowerShell installers.

