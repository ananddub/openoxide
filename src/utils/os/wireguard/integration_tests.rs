use std::process::Command;

#[test]
#[ignore = "requires Linux user namespaces and WireGuard kernel support"]
fn wireguard_peers_complete_real_kernel_handshake() {
    if !cfg!(target_os = "linux") {
        return;
    }

    let script = r#"
set -euo pipefail
umask 077
test_dir=$(mktemp -d)
unshare --net sleep 30 & peer_a=$!
unshare --net sleep 30 & peer_b=$!
cleanup() {
    kill "$peer_a" "$peer_b" 2>/dev/null || true
    rm -rf "$test_dir"
}
trap cleanup EXIT

ip link add bridge0 type bridge
ip link set bridge0 up
ip link add underlay-a type veth peer name peer-a
ip link add underlay-b type veth peer name peer-b
ip link set underlay-a master bridge0
ip link set underlay-b master bridge0
ip link set underlay-a up
ip link set underlay-b up
ip link set peer-a netns "$peer_a"
ip link set peer-b netns "$peer_b"

nsenter -t "$peer_a" -n ip link set lo up
nsenter -t "$peer_b" -n ip link set lo up
nsenter -t "$peer_a" -n ip link set peer-a up
nsenter -t "$peer_b" -n ip link set peer-b up
nsenter -t "$peer_a" -n ip address add 192.0.2.11/24 dev peer-a
nsenter -t "$peer_b" -n ip address add 192.0.2.12/24 dev peer-b

wg genkey > "$test_dir/a.key"
wg pubkey < "$test_dir/a.key" > "$test_dir/a.pub"
wg genkey > "$test_dir/b.key"
wg pubkey < "$test_dir/b.key" > "$test_dir/b.pub"

nsenter -t "$peer_a" -n ip link add wg-a type wireguard
nsenter -t "$peer_b" -n ip link add wg-b type wireguard
nsenter -t "$peer_a" -n ip address add 10.90.0.1/32 dev wg-a
nsenter -t "$peer_b" -n ip address add 10.90.0.2/32 dev wg-b
nsenter -t "$peer_a" -n wg set wg-a private-key "$test_dir/a.key" listen-port 52031 peer "$(cat "$test_dir/b.pub")" allowed-ips 10.90.0.2/32 endpoint 192.0.2.12:52032
nsenter -t "$peer_b" -n wg set wg-b private-key "$test_dir/b.key" listen-port 52032 peer "$(cat "$test_dir/a.pub")" allowed-ips 10.90.0.1/32 endpoint 192.0.2.11:52031
nsenter -t "$peer_a" -n ip link set wg-a up
nsenter -t "$peer_b" -n ip link set wg-b up
nsenter -t "$peer_a" -n ip route add 10.90.0.2/32 dev wg-a
nsenter -t "$peer_b" -n ip route add 10.90.0.1/32 dev wg-b

nsenter -t "$peer_a" -n ping -c 2 -W 2 10.90.0.2
handshake=$(nsenter -t "$peer_a" -n wg show wg-a latest-handshakes | cut -f2)
test "$handshake" -gt 0
"#;

    let output = Command::new("unshare")
        .args([
            "--user",
            "--map-root-user",
            "--mount",
            "--net",
            "bash",
            "-c",
            script,
        ])
        .output()
        .expect("failed to start WireGuard namespace test");

    assert!(
        output.status.success(),
        "WireGuard namespace test failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
