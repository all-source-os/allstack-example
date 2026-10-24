use allstack_example::handoff_receipt;

#[test]
fn supported_transfers_have_canonical_receipts() {
    assert_eq!(
        handoff_receipt("codex", "claude-code"),
        Ok("codex -> claude-code".into())
    );
    assert_eq!(
        handoff_receipt("claude-code", "codex"),
        Ok("claude-code -> codex".into())
    );
}

#[test]
fn unknown_and_empty_hosts_are_rejected() {
    for (from, to) in [
        ("", "claude-code"),
        ("codex", ""),
        ("gpt", "codex"),
        ("codex", "other"),
    ] {
        assert!(handoff_receipt(from, to).is_err());
    }
}

#[test]
fn unchanged_hosts_are_rejected() {
    for host in ["codex", "claude-code"] {
        assert!(handoff_receipt(host, host).is_err());
    }
}

#[test]
fn whitespace_is_not_normalized() {
    for (from, to) in [
        (" codex", "claude-code"),
        ("codex", "claude-code "),
        ("codex\n", "claude-code"),
    ] {
        assert!(handoff_receipt(from, to).is_err());
    }
}

#[test]
fn matching_is_case_sensitive() {
    assert!(handoff_receipt("Codex", "claude-code").is_err());
    assert!(handoff_receipt("codex", "Claude-Code").is_err());
}
