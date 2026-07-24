use std::fs;
use std::process::Command;

/// Generates an SSH keypair of the specified type ("ed25519" or "rsa").
/// Returns (private_key, public_key) as strings.
pub fn generate_keypair(key_type: &str) -> Result<(String, String), std::io::Error> {
    let temp_dir = tempfile::tempdir()?;
    let key_path = temp_dir.path().join("id_key");

    let mut cmd = Command::new("ssh-keygen");
    cmd.arg("-t").arg(key_type);

    if key_type == "rsa" {
        cmd.arg("-b").arg("4096");
    }

    cmd.arg("-N")
        .arg("") // No passphrase
        .arg("-f")
        .arg(&key_path)
        .arg("-q"); // Quiet mode

    let output = cmd.output()?;
    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("ssh-keygen failed: {}", err_msg.trim()),
        ));
    }

    let private_key = fs::read_to_string(&key_path)?;
    let public_key_path = temp_dir.path().join("id_key.pub");
    let public_key = fs::read_to_string(public_key_path)?;

    Ok((private_key, public_key))
}
