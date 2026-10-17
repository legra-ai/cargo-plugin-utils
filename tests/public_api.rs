//! Integration checks shipped in the public archive.

use cargo_plugin_utils::get_owner_repo;

#[test]
fn explicit_repository_identity_needs_both_components() {
    assert!(get_owner_repo(Some("legra-ai".to_owned()), None).is_err());
    assert!(get_owner_repo(None, Some("example".to_owned())).is_err());
    let identity = get_owner_repo(Some("legra-ai".to_owned()), Some("example".to_owned()))
        .expect("complete explicit repository identity");
    assert_eq!(identity, ("legra-ai".to_owned(), "example".to_owned()));
}

#[tokio::test]
async fn subprocess_eof_finishes_without_timeout_fallbacks() {
    let mut logger = cargo_plugin_utils::Logger::new();
    let output = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        cargo_plugin_utils::logger::run_subprocess(
            &mut logger,
            || {
                #[cfg(unix)]
                let mut command = portable_pty::CommandBuilder::new("sh");
                #[cfg(unix)]
                command.args(["-c", "printf 'complete-output'; exit 7"]);
                #[cfg(windows)]
                let mut command = portable_pty::CommandBuilder::new("cmd.exe");
                #[cfg(windows)]
                command.args(["/d", "/c", "echo complete-output&exit /b 7"]);
                command
            },
            Some(3),
        ),
    )
    .await
    .expect("an exited child must close the PTY and renderer promptly")
    .expect("successful output capture");
    assert_eq!(output.exit_code(), 7);
    let captured = output.stderr_str().expect("UTF-8 output");
    #[cfg(unix)]
    assert_eq!(captured, "complete-output");
    #[cfg(windows)]
    assert!(captured.contains("complete-output"));
}
