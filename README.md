# Allstack example

A dependency-free Rust example for an installed Allstack delivery run across Codex CLI and Claude Code.

`handoff_receipt(from, to)` accepts exactly `codex` and `claude-code`, requires a host change, and returns `FROM -> TO`. Unknown, empty, unchanged, whitespace-padded, and differently cased host names are rejected.

```sh
cargo test --locked
```

The [delivery request](docs/requests/handoff-receipt.md) freezes the acceptance contract. Repository-local host guidance, workflow methods, documentation, and verification policy are installed by `allstack init --profile rust`. The [CLI guide](docs/reference/allstack.md) describes managed planning, handoff, verification, recovery, and shipping.

Private AllSource, Chronis, and Prime stores remain ignored. Public source and the indexed delivery plan are reviewable through the proof pull request. Ship means a verified feature commit and an open PR; merge and deployment are separate actions.
