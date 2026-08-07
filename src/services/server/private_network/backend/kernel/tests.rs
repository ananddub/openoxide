use super::KernelWireGuardBackend;

#[test]
fn rotation_error_reports_incomplete_rollback() {
    let restored = KernelWireGuardBackend::rotation_error("rotation failed", Ok(()));
    assert!(restored.to_string().contains("was restored"));

    let incomplete = KernelWireGuardBackend::rotation_error(
        "rotation failed",
        Err("remote restore failed".into()),
    );
    assert!(incomplete.to_string().contains("rollback was incomplete"));
    assert!(incomplete.to_string().contains("remote restore failed"));
}
