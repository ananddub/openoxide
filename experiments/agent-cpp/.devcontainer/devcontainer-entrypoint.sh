#!/bin/sh
# Runs as root (containerUser) before VS Code starts the server as vscode.
# Grants the vscode user access to the bind-mounted host docker.sock without
# touching the socket's own permissions: create a group with the socket's
# numeric gid inside the container and add vscode to it.
set -e

if [ -S /var/run/docker.sock ]; then
	gid=$(stat -c %g /var/run/docker.sock)
	name=$(getent group "$gid" | cut -d: -f1)
	if [ -z "$name" ]; then
		groupadd -g "$gid" dockersock
		name=dockersock
	fi
	usermod -aG "$name" vscode || true
fi

exec "$@"
