//! Helpers shared by the crates' integration tests, like Go's `internal/testutil`.

use std::process::Command;

const CHILD: &str = "TSRS_ISOLATED_TEST";

/// Returns whether the caller should run its body.
///
/// Tests that change process-wide state (fast mode, checker history, write settling, the descriptor limit) or measure
/// the process's RSS run alone in a fresh process of their test binary. In that child process this returns true;
/// otherwise it runs `test` (its full name, as `--exact` takes it) in a child process, asserts that it passed and
/// returns false.
pub fn in_child(test: &str) -> bool {
    in_child_with_env(test, &[])
}

/// `in_child`, with `env` set in the child process.
pub fn in_child_with_env(test: &str, env: &[(&str, &str)]) -> bool {
    if std::env::var_os(CHILD).is_some_and(|name| name == test) {
        return true;
    }
    let output = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", test, "--nocapture", "--test-threads=1"])
        .envs(env.iter().copied())
        .env(CHILD, test)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "{stdout}{}", String::from_utf8_lossy(&output.stderr));
    assert!(stdout.contains("1 passed;"), "the child must run the isolated case:\n{stdout}");
    false
}
