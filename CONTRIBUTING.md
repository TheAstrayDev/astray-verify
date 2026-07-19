# Contributing to Astray Verify

Small, reproducible improvements are the fastest way to help.

1. Open an issue before large or API-changing work.
2. Keep pull requests focused on one problem.
3. Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test --locked`.
4. Never include API keys, access tokens, or private MCP traffic in fixtures or logs.

For a new MCP behavior, add a minimal fixture and an end-to-end test whenever possible.
