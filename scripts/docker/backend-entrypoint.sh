#!/bin/sh
set -eu

APP_UID="${APP_UID:-1000}"
APP_GID="${APP_GID:-1000}"

fix_ownership_if_exists() {
  path="$1"
  if [ -e "$path" ]; then
    chown -R "$APP_UID:$APP_GID" "$path" 2>/dev/null || true
  fi
}

fix_ownership_if_exists /app/logs
fix_ownership_if_exists /app/users.json

exec gosu "$APP_UID:$APP_GID" "$@"