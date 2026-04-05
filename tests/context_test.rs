use shellmate::context::ShellContext;

#[test]
fn test_context_build() {
    let ctx = ShellContext::build("bash");
    assert!(!ctx.current_directory.is_empty());
    assert!(!ctx.os_type.is_empty());
    assert_eq!(ctx.shell, "bash");
    assert!(ctx.history.len() <= 8);
}
